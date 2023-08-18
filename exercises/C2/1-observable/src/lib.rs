#![no_std]

use core::cell::UnsafeCell;

pub struct Observable<T> {
    inner: UnsafeCell<T>,
}

impl<T> Observable<T> {
    pub const fn new(init: T) -> Self {
        Self {
            inner: UnsafeCell::new(init),
        }
    }

    pub async fn wait_until(&self, condition: fn(&T) -> bool) -> T {
        todo!()
    }
}

unsafe impl<T> Send for Observable<T> {}
unsafe impl<T> Sync for Observable<T> {}
