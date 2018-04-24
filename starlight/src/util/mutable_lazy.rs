use futures::prelude::*;

use std::cell::UnsafeCell;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// This code shamelessly stolen from the incredible [lazy-init](https://github.com/khuey/lazy-init)
pub struct MutableLazy<T> {
    value: UnsafeCell<Option<T>>,
    initialized: AtomicBool,
    lock: Mutex<()>,
}

impl<T: Sync> MutableLazy<T> {
    pub fn new() -> MutableLazy<T> {
        MutableLazy {
            value: UnsafeCell::new(None),
            initialized: AtomicBool::new(false),
            lock: Mutex::new(()),
        }
    }

    pub fn get<F: FnOnce() -> T>(&self, create: F) -> &T {
        if !self.initialized.load(Ordering::Acquire) {
            // We *may* not be initialized. We have to block to be certain.
            let _lock = self.lock.lock().unwrap();
            if !self.initialized.load(Ordering::Relaxed) {
                // Ok, we're definitely uninitialized.
                // Safe to fiddle with the UnsafeCell now, because we're locked,
                // and there can't be any outstanding references.
                unsafe { *self.value.get() = Some(create()) };
                self.initialized.store(true, Ordering::Release);
            } else {
                // We raced, and someone else initialized us. We can fall
                // through now.
            }
        }

        // We're initialized, our value is immutable, no synchronization needed.
        unsafe { (*self.value.get()).as_ref().unwrap() }
    }

    pub fn get_mut<F: FnOnce() -> T>(&mut self, create: F) -> &mut T {
        // Since self is mutable here, there can't be any other references,
        // so synchronization should not be necessary.
        if !self.initialized.load(Ordering::Relaxed) {
            unsafe { *self.value.get() = Some(create()) };
            self.initialized.store(true, Ordering::Relaxed);
        }

        unsafe { (*self.value.get()).as_mut().unwrap() }
    }
}

unsafe impl<T: Sync> Sync for MutableLazy<T> { }

