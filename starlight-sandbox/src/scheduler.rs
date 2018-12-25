use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

const EMPTY: usize = 0;
const FULL: usize = 1;
const IN_USE: usize = 2;

pub struct ResourceCell<T: Send> {
    status: AtomicUsize,
    resource: UnsafeCell<Option<T>>,
}

impl<T: Send> ResourceCell<T> {
    pub fn new(data: Option<T>) -> ResourceCell<T> {
        ResourceCell {
            status: AtomicUsize::new(if data.is_none() { EMPTY } else { FULL }),
            resource: UnsafeCell::new(data)
        }
    }

    pub fn try_acquire(&self) -> Result<Guard<T>, ()> {
        if self.status.compare_and_swap(FULL, IN_USE, Ordering::Acquire) == IN_USE {
            return Ok(Guard(self, FULL)) 
        }
        Err(())
    }

    pub fn try_take(&self) -> Result<T, ()> {
        if self.status.compare_and_swap(FULL, IN_USE, Ordering::Acquire) == IN_USE {
            let value = unsafe { self.resource.get().as_mut().unwrap().take().unwrap() };
            self.status.store(EMPTY, Ordering::Release);
            return Ok(value);
        }
        Err(())
    }

    pub fn try_reserve(&self) -> Result<Reserve<T>, ()> {
        if self.status.compare_and_swap(EMPTY, IN_USE, Ordering::Acquire) == IN_USE {
            return Ok(Reserve(self))
        }
        Err(())
    }
}

unsafe impl<T: Send> Sync for ResourceCell<T> { }

pub struct Guard<'a, T: Send + 'a>(&'a ResourceCell<T>, usize);

impl<'a, T: Send + 'a> Guard<'a, T> {
    pub fn take(mut self) -> T {
        let value = unsafe { self.0.resource.get().as_mut().unwrap().take().unwrap() };
        self.1 = EMPTY;
        return value;
    }
}

impl<'a, T: Send + 'a> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.0.status.store(self.1, Ordering::Release);
    }
}

impl<'a, T: Send + 'a> Deref for Guard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        return unsafe { self.0.resource.get().as_ref().unwrap().as_ref().unwrap() };
    }
}

impl<'a, T: Send + 'a> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        return unsafe { self.0.resource.get().as_mut().unwrap().as_mut().unwrap() };
    }
}

pub struct Reserve<'a, T: Send + 'a>(&'a ResourceCell<T>); 

impl<'a, T: Send + 'a> Reserve<'a, T> {
    pub fn fill(self, value: T) {
        unsafe { *self.0.resource.get().as_mut().unwrap() = Some(value) };
        self.0.status.store(FULL, Ordering::Release);
    }
}

macro_rules! producer {
    {
        $name:ident ($($input:ident: $input_ty:ty),*) {
            $body:block
        } => ($($output:ident: $output_ty:ty),*)
    } => {
        pub struct $name<'a> {
            $($input: &'a ResourceCell<$input_ty>),*
            $($output: &'a ResourceCell<$output_ty>),*
        }

        impl<'a> $name<'a> {
            pub fn new($($input: &'a ResourceCell<$input_ty>),* $($output: &'a ResourceCell<$output_ty>),*) -> Self {
                $name {
                    $($input: $input),*
                    $($output: $output),*
                }
            }
        }

        impl<'a> Producer for $name<'a> {
            pub fn 
        }

        move || -> Result<(), ()> {
            struct OutputCells { 
                $($output:
            )
            $(
                let mut $output = ($out_cell).try_reserve()?;
            )*
            $(
                let mut $input = ($in_cell).try_acquire()?;
            )*
            (|$($input),*| $body)($($input.take()),*);
            
        }
    }
}

macro_rules! worker {
    {
        $name {
            $($resource:ident: $resource_ty:ty)*
        }
    } => {
        struct $name {
            
        }
    }
}