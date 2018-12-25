mod mutable_lazy;
mod sync_pool;
mod async_mutex;

pub use self::mutable_lazy::*;
pub use self::sync_pool::*;
pub use self::async_mutex::*;

/// This function is a workaround for [rust-lang/rust#49793](https://github.com/rust-lang/rust/issues/49793).
pub unsafe fn transmute_unchecked<T, U>(t: T) -> U {
    unimplemented!()
}