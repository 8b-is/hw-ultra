use crate::memory::heap::bump::BumpAllocator;

pub struct BuddyAllocator {
    _private: (),
}

impl Default for BuddyAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl BuddyAllocator {
    pub fn new() -> Self {
        BuddyAllocator { _private: () }
    }

    pub fn alloc(&self, size: usize) -> *mut u8 {
        let b = BumpAllocator::new();
        b.alloc(size, core::mem::align_of::<usize>())
    }
}
