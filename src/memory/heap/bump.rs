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
        // Store an offset: zero is a valid initial cursor, not a racy sentinel.
        let mut offset = BUMP_PTR.load(Ordering::Acquire);
        loop {
            let Some(aligned) = heap_ptr
                .checked_add(offset)
                .and_then(|ptr| ptr.checked_add(align - 1))
                .map(|ptr| ptr & !(align - 1))
            else {
                return core::ptr::null_mut();
            };
            let current_used = aligned - heap_ptr;
            if !crate::arch::guardian::gate_heap(current_used, size, BUMP_HEAP_SIZE) {
                return core::ptr::null_mut();
            }
            let Some(next) = current_used.checked_add(size) else {
                return core::ptr::null_mut();
            };
            if next > BUMP_HEAP_SIZE {
                return core::ptr::null_mut();
            }
            match BUMP_PTR.compare_exchange_weak(offset, next, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return aligned as *mut u8,
                Err(actual) => offset = actual,
            }
        }
    }

    /// Rewinds the shared heap used by every allocator instance.
    /// Callers must stop allocations and stop using previously returned pointers
    /// before resetting; this is not a per-instance deallocation operation.

    pub fn reset(&self) {
        BUMP_PTR.store(0, Ordering::Release);
    }
}
