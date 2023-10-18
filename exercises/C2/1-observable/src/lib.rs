// #![no_std] // <-- Uncomment for doing the embedded challenge

use core::marker::PhantomData;

pub struct Observable<T> {
    _phantom: PhantomData<T>,
}

impl<T: Clone> Observable<T> {
    /// Construct a new observable
    pub const fn new(init: T) -> Self {
        todo!()
    }

    /// Wait for any change of the variable
    pub async fn wait(&self) {
        todo!()
    }

    /// Wait until the variable passes the condition check
    pub async fn wait_until(&self, condition: fn(&T) -> bool) -> T {
        todo!()
    }

    /// Set the value
    pub async fn set(&self, value: T) {
        todo!()
    }

    /// Get the value
    pub async fn get(&self) -> T {
        todo!()
    }
}

unsafe impl<T> Send for Observable<T> {}
unsafe impl<T> Sync for Observable<T> {}
