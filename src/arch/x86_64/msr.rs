use core::sync::atomic::{AtomicUsize, Ordering};

static MSR_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_msr_mmio_base(base: usize) {
    MSR_MMIO_BASE.store(base, Ordering::Release);
}

/// # Safety
/// `msr` must be a valid MSR index and the MMIO base must be configured.
pub unsafe fn read_msr(msr: u32) -> u64 {
    let base = MSR_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        core::ptr::read_volatile((base + msr as usize * 8) as *const u64)
    } else {
        0
    }
}

/// # Safety
/// `msr` must be a valid MSR index and the MMIO base must be configured.
pub unsafe fn write_msr(msr: u32, val: u64) {
    let base = MSR_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        core::ptr::write_volatile((base + msr as usize * 8) as *mut u64, val);
    }
}
