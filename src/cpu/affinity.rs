use core::sync::atomic::{AtomicUsize, Ordering};

static CURRENT_AFFINITY: AtomicUsize = AtomicUsize::new(0);

pub fn set_affinity(core_mask: usize) {
    CURRENT_AFFINITY.store(core_mask, Ordering::Release);
    let mask: u64 = core_mask as u64;
    unsafe {
        crate::sys::raw_syscall(
            crate::arch::shim::nr_sched_setaffinity(),
            0,
            8,
            &mask as *const u64 as u64,
            0,
            0,
            0,
        );
    }
}

pub fn get_affinity() -> usize {
    let mut mask: u64 = 0;
    let ret = unsafe {
        crate::sys::raw_syscall(
            crate::arch::shim::nr_sched_getaffinity(),
            0,
            8,
            &mut mask as *mut u64 as u64,
            0,
            0,
            0,
        )
    };
    if ret >= 0 {
        CURRENT_AFFINITY.store(mask as usize, Ordering::Release);
        mask as usize
    } else {
        CURRENT_AFFINITY.load(Ordering::Acquire)
    }
}

pub fn pin_to_core(core_id: usize) {
    let mask = 1usize << core_id;
    set_affinity(mask);
}

pub fn is_pinned_to(core_id: usize) -> bool {
    let mask = get_affinity();
    mask & (1 << core_id) != 0
}
