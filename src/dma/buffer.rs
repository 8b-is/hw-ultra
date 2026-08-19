use crate::memory::heap::bump::BumpAllocator;
use crate::memory::phys::frame::Frame;

pub struct DmaBuffer {
    ptr: *mut u8,
    len: usize,
}

impl DmaBuffer {
    pub fn new(size: usize, align: usize) -> Option<Self> {
        let b = BumpAllocator::new();
        let p = b.alloc(size, align);
        if p.is_null() {
            return None;
        }
        Some(DmaBuffer { ptr: p, len: size })
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn phys_addr(&self) -> usize {
        self.ptr as usize
    }

    pub fn frame(&self) -> Frame {
        Frame::new(self.phys_addr())
    }
}

impl Drop for DmaBuffer {
    fn drop(&mut self) { /* bump allocator handles lifetime; nothing to free */
    }
}
