use futures::prelude::*;
use futures::task::{Context, Waker};

use std::collections::VecDeque;
use std::iter;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard, PoisonError, TryLockError};

pub struct ResourcePool<T> {
    items: Box<[Mutex<T>]>,
    wakers: Mutex<VecDeque<Waker>>,
}

impl<T> ResourcePool<T> {
    pub fn new<F: Fn() -> T>(capacity: usize, make_item: F) -> ResourcePool<T> {
        ResourcePool {
            items: iter::repeat(()).map(|_| Mutex::new(make_item())).take(capacity).collect::<Vec<_>>().into_boxed_slice(),
            wakers: Mutex::new(VecDeque::new())
        }
    }

    pub fn acquire(&self) -> ResourceFuture<T> {
        ResourceFuture { pool: self }
    }
}

pub struct ResourceFuture<'a, T: 'a> {
    pool: &'a ResourcePool<T>,
}

impl<'a, T: 'a> Future for ResourceFuture<'a, T> {
    type Item = PoolGuard<'a, T>;
    type Error = PoisonError<Option<PoolGuard<'a, T>>>;

    fn poll(&mut self, cx: &mut Context) -> Result<Async<Self::Item>, Self::Error> {
        for mutex in &*self.pool.items {
            match mutex.try_lock() {
                Ok(guard) => return Ok(Async::Ready(PoolGuard(guard, &self.pool.wakers))),
                Err(TryLockError::Poisoned(e)) => return Err(PoisonError::new(Some(PoolGuard(e.into_inner(), &self.pool.wakers)))),
                Err(TryLockError::WouldBlock) => continue,
            }
        }
        match self.pool.wakers.lock() {
            Ok(mut wakers) => wakers.push_back(cx.waker().clone()),
            Err(_) => return Err(PoisonError::new(None)),
        }
        Ok(Async::Pending)
    }
}

pub struct PoolGuard<'a, T: 'a>(MutexGuard<'a, T>, &'a Mutex<VecDeque<Waker>>);

impl<'a, T> Drop for PoolGuard<'a, T> {
    fn drop(&mut self) {
        self.1.lock().unwrap().pop_front().map(|w| w.wake());
    }
}

impl<'a, T: 'a> Deref for PoolGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<'a, T: 'a> DerefMut for PoolGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}