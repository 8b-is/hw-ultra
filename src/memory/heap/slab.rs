use crate::memory::heap::bump::BumpAllocator;

pub struct Slab {
    _private: (),
}

impl Default for Slab {
    fn default() -> Self {
        Self::new()
    }
}

impl Slab {
    pub fn new() -> Self {
        Slab { _private: () }
    }

    pub fn alloc(&self, size: usize) -> *mut u8 {
        let b = BumpAllocator::new();
        b.alloc(size, core::mem::align_of::<usize>())
    }
}
