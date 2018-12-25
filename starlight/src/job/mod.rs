// This code heavily based on the following files:
// https://github.com/rust-lang-nursery/futures-rs/blob/0.2/futures-executor/src/thread_pool.rs
// https://github.com/tokio-rs/tokio/blob/master/tokio-threadpool/src/lib.rs
// In fact, it is mostly a stripped-down version of the latter using the former's interface.

use deque::{self, Stealer, Worker, Stolen};

use futures;
use futures::prelude::*;
use futures::executor::SpawnError;
use futures::task::{Wake, Waker, LocalMap};

use num_cpus;

use rand::{self, Rng};

use std::ops::Range;
use std::sync::{Arc, Weak};
use std::cell::Cell;
use std::ptr;
use std::thread;

mod mpsc_queue;
mod unpark_mutex;
mod scheduler;

use self::mpsc_queue::{self as mpsc, Producer, Consumer, PopResult};
use self::unpark_mutex::UnparkMutex;

trait AssertSendSync: Send + Sync {}
impl AssertSendSync for WorkStealingPool {}

thread_local! {
    static WORKER_THREAD: Cell<*const WorkerThread> = Cell::new(ptr::null());
}

/// RNG, currently uses `rand::thread_rng()`, may test a simpler algorithm if it improves performance.
fn rand_in_range(range: Range<usize>) -> usize {
    rand::thread_rng().gen_range(range.start, range.end)
}

struct WakeHandle {
    exec: WorkStealingPool,
    mutex: UnparkMutex<Task>,
}

impl Wake for WakeHandle {
    fn wake(this: &Arc<WakeHandle>) {
        match this.mutex.notify() {
            Ok(task) => { this.exec.spawn_internal(task); },
            Err(()) => { }
        };
    }
}

struct Task {
    spawn: Box<Future<Item = (), Error = Never> + Send>,
    map: task::LocalMap,
    exec: WorkStealingPool,
    wake_handle: Arc<WakeHandle>,
}

impl Task {
    /// Actually run the task (invoking `poll` on its future) on the current
    /// thread.
    pub fn run(self) {
        let Task { mut spawn, wake_handle, mut map, mut exec } = self;
        let waker = Waker::from(wake_handle.clone());

        // SAFETY: the ownership of this `Task` object is evidence that
        // we are in the `POLLING`/`REPOLL` state for the mutex.
        unsafe {
            wake_handle.mutex.start_poll();

            loop {
                let res = {
                    let mut cx = task::Context::new(&mut map, &waker, &mut exec);
                    spawn.poll(&mut cx)
                };
                match res {
                    Ok(Async::Pending) => {}
                    Ok(Async::Ready(())) => return wake_handle.mutex.complete(),
                    Err(never) => match never {},
                }
                let task = Task {
                    spawn,
                    map,
                    wake_handle: wake_handle.clone(),
                    exec: exec
                };
                match wake_handle.mutex.wait(task) {
                    Ok(()) => return,            // we've waited
                    Err(r) => { // someone's notified us
                        spawn = r.spawn;
                        map = r.map;
                        exec = r.exec;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct WorkStealingPool {
    inner: Arc<PoolInner>
}

impl WorkStealingPool {
    fn spawn_internal(&self, task: Task) {
        WorkerThread::with_current(&self.inner, |worker| {
            match worker {
                Some(worker) => worker.internal_queue.push(task),
                None => self.inner.producers[rand_in_range(0..self.inner.size)].push(task),
            }
        });
    }
}

impl Executor for WorkStealingPool {
    fn spawn(&mut self, f: Box<Future<Item = (), Error = Never> + Send>) -> Result<(), SpawnError> {
        self.spawn_internal(Task {
            spawn: f,
            map: LocalMap::new(),
            exec: self.clone(),
            wake_handle: Arc::new(WakeHandle {
                exec: self.clone(),
                mutex: UnparkMutex::new(),
            }),
        });
        Ok(())
    }
}

struct PoolInner {
    stealers: Vec<Stealer<Task>>,
    producers: Vec<Producer<Task>>,
    size: usize,
}

struct WorkerThread {
    pool: Weak<PoolInner>,
    id: usize,
    internal_queue: Worker<Task>,
    external_queue: Consumer<Task>,
    after_start: Option<Arc<Fn(usize) + Send + Sync>>, 
    before_stop: Option<Arc<Fn(usize) + Send + Sync>>
}

impl WorkerThread {
    fn spawn(pool: &Arc<PoolInner>, 
        id: usize, 
        foreground: bool,
        worker: Worker<Task>, 
        consumer: Consumer<Task>,
        builder: &WorkStealingPoolBuilder)
    {
        let worker = WorkerThread {
            pool: Arc::downgrade(pool),
            id: id,
            internal_queue: worker,
            external_queue: consumer,
            after_start: builder.after_start.clone(),
            before_stop: builder.before_stop.clone(),
        };

        match foreground {
            false => {
                let mut thread_builder = thread::Builder::new();
                if let Some(ref name_prefix) = builder.name_prefix {
                    thread_builder = thread_builder.name(format!("{}{}", name_prefix, id));
                }
                if builder.stack_size > 0 { thread_builder = thread_builder.stack_size(builder.stack_size); }
                
                thread_builder.spawn(move || worker.work()).unwrap();
            },
            true => worker.work()
        }
    }

    fn with_current<F: FnOnce(Option<&WorkerThread>)>(pool: &Arc<PoolInner>, f: F) {
        WORKER_THREAD.with(|c| {
            let worker = c.get();
            unsafe {
                if !worker.is_null() && Arc::ptr_eq(pool, &(&*worker).pool.upgrade().unwrap()) {
                    f(Some(&*worker))
                }
                else {
                    f(None)
                }
            }
        })
    }

    fn work(self) {
        let _scope = futures::executor::enter().expect("Attempted to run a worker thread on a thread that already has an executor!");
        WORKER_THREAD.with(|c| c.set(&self as *const _));
        self.after_start.as_ref().map(|f| f(self.id));
        loop {
            let consistent = self.empty_external_queue();
            if let Some(pool) = self.pool.upgrade() {
                match self.internal_queue.pop().or_else(|| self.steal_task(&pool)) {
                    Some(task) => task.run(),
                    None => { 
                        if !consistent {
                            continue;
                        } else { 
                            self.yield_time(); 
                        }
                    },
                }
            } else { break }
        }
        self.before_stop.as_ref().map(|f| f(self.id));
        WORKER_THREAD.with(|c| c.set(ptr::null()));
    }

    fn steal_task(&self, pool: &Arc<PoolInner>) -> Option<Task> {
        let stealer_id = rand_in_range(0..pool.stealers.len());
        if stealer_id != self.id {
            if let Stolen::Data(task) = pool.stealers[stealer_id].steal() {
                return Some(task)
            }
        }
        None
    }

    fn empty_external_queue(&self) -> bool {
        loop {
            match self.external_queue.pop() {
                PopResult::Data(task) => self.internal_queue.push(task),
                PopResult::Empty => return false,
                PopResult::Inconsistent => return true,
            }
        }
    }

    fn yield_time(&self) {

    }
}

/// Thread pool configuration object.
pub struct WorkStealingPoolBuilder {
    pool_size: usize,
    stack_size: usize,
    name_prefix: Option<String>,
    after_start: Option<Arc<Fn(usize) + Send + Sync>>,
    before_stop: Option<Arc<Fn(usize) + Send + Sync>>,
}

impl WorkStealingPoolBuilder {
    /// Create a default work stealing pool configuration.
    ///
    /// See the other methods on this type for details on the defaults.
    pub fn new() -> WorkStealingPoolBuilder {
        WorkStealingPoolBuilder {
            pool_size: num_cpus::get(),
            stack_size: 0,
            name_prefix: None,
            after_start: None,
            before_stop: None,
        }
    }

    /// Set size of a future WorkStealingPool
    ///
    /// The size of a work stealing pool is the number of worker threads spawned.  By
    /// default, this is equal to the number of CPU cores.
    pub fn pool_size(&mut self, size: usize) -> &mut Self {
        self.pool_size = size;
        self
    }

    /// Set stack size of threads in the pool.
    ///
    /// By default, worker threads use Rust's standard stack size.
    pub fn stack_size(&mut self, stack_size: usize) -> &mut Self {
        self.stack_size = stack_size;
        self
    }

    /// Set thread name prefix of a future WorkStealingPool.
    ///
    /// Thread name prefix is used for generating thread names. For example, if prefix is
    /// `my-pool-`, then threads in the pool will get names like `my-pool-1` etc.
    ///
    /// By default, worker threads are assigned Rust's standard thread name.
    pub fn name_prefix<S: Into<String>>(&mut self, name_prefix: S) -> &mut Self {
        self.name_prefix = Some(name_prefix.into());
        self
    }

    /// Execute the closure `f` immediately after each worker thread is started,
    /// but before running any tasks on it.
    ///
    /// This hook is intended for bookkeeping and monitoring.
    /// The closure `f` will be dropped after the `builder` is dropped
    /// and all worker threads in the pool have executed it.
    ///
    /// The closure provided will receive an index corresponding to the worker
    /// thread it's running on.
    pub fn after_start<F>(&mut self, f: F) -> &mut Self
        where F: Fn(usize) + Send + Sync + 'static
    {
        self.after_start = Some(Arc::new(f));
        self
    }

    /// Execute closure `f` just prior to shutting down each worker thread.
    ///
    /// This hook is intended for bookkeeping and monitoring.
    /// The closure `f` will be dropped after the `builder` is droppped
    /// and all threads in the pool have executed it.
    ///
    /// The closure provided will receive an index corresponding to the worker
    /// thread it's running on.
    pub fn before_stop<F>(&mut self, f: F) -> &mut Self
        where F: Fn(usize) + Send + Sync + 'static
    {
        self.before_stop = Some(Arc::new(f));
        self
    }

    /// Create a [`WorkStealingPool`](::WorkStealingPool) with the given configuration.
    ///
    /// # Panics
    ///
    /// Panics if `pool_size == 0`.
    pub fn create(&mut self) -> WorkStealingPool {
        assert!(self.pool_size > 0);

        let mut workers = Vec::new();
        let mut stealers = Vec::new();
        let mut producers = Vec::new();
        let mut consumers = Vec::new();
        for _ in 0..self.pool_size {
            let (worker, stealer) = deque::new();
            stealers.push(stealer);
            workers.push(worker);

            let (producer, consumer) = mpsc::queue();
            producers.push(producer);
            consumers.push(consumer);
        }

        let pool = WorkStealingPool {
            inner: Arc::new(PoolInner {
                stealers: stealers,
                producers: producers,
                size: self.pool_size,
            })
        };

        let mut id = 0;
        for (worker, consumer) in workers.into_iter().zip(consumers.into_iter()) {
            WorkerThread::spawn(&pool.inner, id, false, worker, consumer, &self);
            id += 1;
        }
        pool
    }

    /// Create a [`WorkStealingPool`](::WorkStealingPool) with the given configuration,
    /// running it on the current thread.
    ///
    /// # Panics
    ///
    /// Panics if `pool_size == 0`.
    pub fn create_foreground<F: Future<Item=(), Error=Never> + Send + 'static>(&mut self, start: Option<F>) -> WorkStealingPool {
        assert!(self.pool_size > 0);

        let mut workers = Vec::new();
        let mut stealers = Vec::new();
        let mut producers = Vec::new();
        let mut consumers = Vec::new();
        for _ in 0..self.pool_size {
            let (worker, stealer) = deque::new();
            stealers.push(stealer);
            workers.push(worker);

            let (producer, consumer) = mpsc::queue();
            producers.push(producer);
            consumers.push(consumer);
        }

        let mut pool = WorkStealingPool {
            inner: Arc::new(PoolInner {
                stealers: stealers,
                producers: producers,
                size: self.pool_size,
            })
        };

        if let Some(start) = start {
            pool.spawn(Box::new(start));
        }

        for (id, (worker, consumer)) in workers.into_iter().zip(consumers.into_iter()).enumerate() {
            WorkerThread::spawn(&pool.inner, id, id == self.pool_size - 1, worker, consumer, &self);
        }
        pool
    }
}
