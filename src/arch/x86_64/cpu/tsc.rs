use core::sync::atomic::{AtomicUsize, Ordering};

static TSC_MMIO: AtomicUsize = AtomicUsize::new(0);

pub fn set_tsc_mmio(addr: usize) {
    TSC_MMIO.store(addr, Ordering::Release);
}

pub fn read_tsc() -> u64 {
    let addr = TSC_MMIO.load(Ordering::Acquire);
    if addr != 0 {
        unsafe { core::ptr::read_volatile(addr as *const u64) }
    } else {
        native_rdtsc()
    }
}

fn native_rdtsc() -> u64 {
    0
}
