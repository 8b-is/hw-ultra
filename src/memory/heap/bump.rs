use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct BumpAllocator(());

const BUMP_HEAP_SIZE: usize = 64 * 1024;

struct HeapStorage(UnsafeCell<[u8; BUMP_HEAP_SIZE]>);

unsafe impl Sync for HeapStorage {}

static BUMP_HEAP: HeapStorage = HeapStorage(UnsafeCell::new([0; BUMP_HEAP_SIZE]));
static BUMP_PTR: AtomicUsize = AtomicUsize::new(0);

impl Default for BumpAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl BumpAllocator {
    pub fn new() -> Self {
        BumpAllocator(())
    }

    pub fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        if align == 0 || !align.is_power_of_two() {
            return core::ptr::null_mut();
        }

        let heap_ptr = BUMP_HEAP.0.get() as *mut u8 as usize;
        let heap_end = heap_ptr.saturating_add(BUMP_HEAP_SIZE);
        let mut ptr = BUMP_PTR.load(Ordering::Acquire);
        if ptr == 0 {
            ptr = heap_ptr;
            BUMP_PTR.store(ptr, Ordering::Release);
        }
        let aligned = (ptr + (align - 1)) & !(align - 1);
        let current_used = aligned.saturating_sub(heap_ptr);
        if !crate::arch::guardian::gate_heap(current_used, size, BUMP_HEAP_SIZE) {
            return core::ptr::null_mut();
        }
        let next = aligned.saturating_add(size);
        if next > heap_end {
            core::ptr::null_mut()
        } else {
            BUMP_PTR.store(next, Ordering::Release);
            aligned as *mut u8
        }
    }

    pub fn reset(&self) {
        let heap_ptr = BUMP_HEAP.0.get() as *mut u8 as usize;
        BUMP_PTR.store(heap_ptr, Ordering::Release);
    }
}
