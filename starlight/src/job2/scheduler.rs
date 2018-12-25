use super::task::TaskSource;
use mpsc_queue::{Consumer, Producer};

pub struct TaskScheduler {
    inner: Arc<Inner>
    threads: usize,
}

struct Inner {
    producers: Vec<Producer<Box<TaskSource>>>,
    workers: Vec<WorkerThread>
}

struct WakeHandle {
    scheduler: TaskScheduler,
    source: TaskSourceHandle,
}

pub struct TaskSourceHandle {

}

impl TaskScheduler {
    pub fn new<Q: IntoTaskQueue>(threads: usize, start: Q) -> Q::Handle {

        let queues = (0..threads).map(|_| mpsc::queue()).collect<Vec<_>>(); 

        TaskScheduler {
            inner: Arc::new(Inner {
                workers: (0..threads).map(|_| WorkerThread::new()).collect<Vec<_>>()
            })
            threads: threads
        }
    }

    pub fn spawn_queue<Q: IntoTaskQueue>(&mut self, queue: Q) -> Q::Handle {
        let (handle, workers) = queue.with_workers(self.threads);
        assert_eq!(workers.len(), self.threads);
        for producer in self.inner.producers.iter() {
            producer.push(Box::new(workers.pop()));
        }
        handle
    }
}

pub struct WorkerThread {
    consumer: Consumer<Box<TaskSource>>,
}

impl WorkerThread {
    fn new(consumer: Consumer<Box<TaskSource>>) -> WorkerThread
    {
        WorkerThread {
            consumer: consumer
        }
    }

    fn run(self, foreground: bool) {
        loop {
            
        }
    }
}

pub trait IntoTaskQueue {
    type Handle;
    type Worker: TaskSource;
    fn with_workers(self, workers: usize) -> (Self::Handle, Vec<Self::Worker>);
}