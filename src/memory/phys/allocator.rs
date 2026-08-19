use crate::memory::phys::frame::Frame;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct PhysAllocator;

impl PhysAllocator {
    pub fn region() -> Option<(usize, usize)> {
        Some((0x1000usize, 0x8000_0000usize))
    }
}

const MAX_FREE_FRAMES: usize = 8192;
use crate::common::once::Once;
use core::cell::UnsafeCell;
static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
static ALLOC_INIT: AtomicUsize = AtomicUsize::new(0);

struct FreeListStorage(UnsafeCell<[usize; MAX_FREE_FRAMES]>);

unsafe impl Sync for FreeListStorage {}

static FREE_LIST_ONCE: Once<FreeListStorage> = Once::new();

impl PhysAllocator {
    pub fn init() -> bool {
        if ALLOC_INIT
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        if let Some((start, end)) = Self::region() {
            let mut cur = (start + 0xfff) & !0xfffusize;
            let mut idx = 0usize;
            if FREE_LIST_ONCE.get().is_none() {
                let ok =
                    FREE_LIST_ONCE.set(FreeListStorage(UnsafeCell::new([0usize; MAX_FREE_FRAMES])));
                debug_assert!(ok);
            }
            let fl = match FREE_LIST_ONCE.get() {
                Some(f) => f,
                None => return false,
            };
            unsafe {
                while cur + 0x1000 <= end && idx < MAX_FREE_FRAMES {
                    (*fl.0.get())[idx] = cur;
                    idx += 1;
                    cur = cur.saturating_add(0x1000);
                }
            }
            FREE_COUNT.store(idx, Ordering::Release);
            true
        } else {
            ALLOC_INIT.store(0, Ordering::Release);
            false
        }
    }

    pub fn alloc_frame() -> Option<Frame> {
        if ALLOC_INIT.load(Ordering::Acquire) == 0 {
            let ok = Self::init();
            debug_assert!(ok);
        }
        loop {
            let cnt = FREE_COUNT.load(Ordering::Acquire);
            if cnt == 0 {
                return None;
            }
            if FREE_COUNT
                .compare_exchange(cnt, cnt - 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                let idx = cnt - 1;
                let fl = FREE_LIST_ONCE.get()?;
                let addr = unsafe { (*fl.0.get())[idx] };
                return Some(Frame::new(addr));
            }
        }
    }

    pub fn free_frame(f: Frame) {
        let mut cnt = FREE_COUNT.load(Ordering::Acquire);
        loop {
            if cnt >= MAX_FREE_FRAMES {
                return;
            }
            if FREE_COUNT
                .compare_exchange(cnt, cnt + 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                let idx = cnt;
                let fl = match FREE_LIST_ONCE.get() {
                    Some(v) => v,
                    None => return,
                };
                unsafe {
                    (*fl.0.get())[idx] = f.as_usize();
                }
                return;
            }
            cnt = FREE_COUNT.load(Ordering::Acquire);
        }
    }
}
