use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Atomic(AtomicUsize);

impl Atomic {
    pub const fn new(v: usize) -> Self {
        Self(AtomicUsize::new(v))
    }

    pub fn load(&self, order: Ordering) -> usize {
        self.0.load(order)
    }

    pub fn store(&self, val: usize, order: Ordering) {
        self.0.store(val, order)
    }

    pub fn swap(&self, val: usize, order: Ordering) -> usize {
        self.0.swap(val, order)
    }

    pub fn compare_exchange(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<usize, usize> {
        self.0.compare_exchange(current, new, success, failure)
    }

    pub fn compare_exchange_weak(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering,
    ) -> Result<usize, usize> {
        self.0.compare_exchange_weak(current, new, success, failure)
    }
}

impl Default for Atomic {
    fn default() -> Self {
        Self::new(0)
    }
}
