use futures::prelude::*;
use futures::task::{Context, Waker};
use futures::channel::mpsc;

use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard, PoisonError, TryLockError};

pub struct AsyncMutex<T> {
    inner: Mutex<(T, mpsc::UnboundedReceiver<Waker>)>,
    send: mpsc::UnboundedSender<Waker>,
}

impl<T> AsyncMutex<T> {
    pub fn new(item: T) -> AsyncMutex<T> {
        let (send, receive) = mpsc::unbounded();
        AsyncMutex {
            inner: Mutex::new((item, receive)),
            send: send,
        }
    }

    pub fn acquire(&self) -> MutexFuture<T> {
        MutexFuture { mutex: self }
    }
}

pub struct MutexFuture<'a, T: 'a> {
    mutex: &'a AsyncMutex<T>,
}

impl<'a, T: 'a> Future for MutexFuture<'a, T> {
    type Item = AsyncGuard<'a, T>;
    type Error = PoisonError<AsyncGuard<'a, T>>;

    fn poll(&mut self, context: &mut Context) -> Result<Async<Self::Item>, Self::Error> { 
        match self.mutex.inner.try_lock() {
            Ok(guard) => Ok(Async::Ready(AsyncGuard(Some(guard)))),
            Err(TryLockError::Poisoned(e)) => Err(PoisonError::new(AsyncGuard(Some(e.into_inner())))),
            Err(TryLockError::WouldBlock) => {
                self.mutex.send.unbounded_send(context.waker().clone());
                Ok(Async::Pending)
            }
        }
    }
}

pub struct AsyncGuard<'a, T: 'a>(Option<MutexGuard<'a, (T, mpsc::UnboundedReceiver<Waker>)>>);

impl<'a, T> Drop for AsyncGuard<'a, T> {
    fn drop(&mut self) {
        // use `take` here to ensure the mutex guard is dropped before the waker is called.
        let waker = match (self.0.take().unwrap()).1.try_next() {
            Ok(Some(waker)) => Some(waker),
            Err(e) => None,
            _ => unreachable!(),
        };
        waker.map(|w| w.wake());
    }
}

impl<'a, T: 'a> Deref for AsyncGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &(self.0.unwrap()).0
    }
}

impl<'a, T: 'a> DerefMut for AsyncGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut(self.0.unwrap()).0
    }
}