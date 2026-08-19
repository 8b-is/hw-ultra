use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};

pub struct Once<T> {
    state: AtomicU8,
    storage: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send + Sync> Sync for Once<T> {}

impl<T> Default for Once<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Once<T> {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(0),
            storage: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn set(&self, val: T) -> bool {
        if self
            .state
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        unsafe {
            (*self.storage.get()).write(val);
        }
        self.state.store(2, Ordering::Release);
        true
    }

    pub fn get(&self) -> Option<&T> {
        if self.state.load(Ordering::Acquire) != 2 {
            return None;
        }
        Some(unsafe { &*((*self.storage.get()).as_ptr()) })
    }
}

pub struct OnceCopy<T: Copy> {
    state: AtomicU8,
    storage: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Copy + Send + Sync> Sync for OnceCopy<T> {}

impl<T: Copy> Default for OnceCopy<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> OnceCopy<T> {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(0),
            storage: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn set(&self, val: T) -> bool {
        if self
            .state
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        unsafe {
            (*self.storage.get()).write(val);
        }
        self.state.store(2, Ordering::Release);
        true
    }

    pub fn get(&self) -> Option<T> {
        if self.state.load(Ordering::Acquire) != 2 {
            return None;
        }
        Some(unsafe { (*self.storage.get()).assume_init() })
    }
}
