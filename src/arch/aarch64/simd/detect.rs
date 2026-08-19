use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static NEON_AVAILABLE: AtomicU8 = AtomicU8::new(0xFF);
static SVE_VL_MMIO: AtomicUsize = AtomicUsize::new(0);

pub fn set_sve_vl_mmio(addr: usize) {
    SVE_VL_MMIO.store(addr, Ordering::Release);
}

pub fn is_neon_supported() -> bool {
    let cached = NEON_AVAILABLE.load(Ordering::Acquire);
    if cached != 0xFF {
        return cached != 0;
    }
    let features = crate::arch::aarch64::cpu::features::detect();
    let result = features.neon;
    NEON_AVAILABLE.store(result as u8, Ordering::Release);
    result
}

pub fn is_sve_supported() -> bool {
    let features = crate::arch::aarch64::cpu::features::detect();
    features.sve
}

pub fn sve_vector_length() -> usize {
    if !is_sve_supported() {
        return 0;
    }
    let addr = SVE_VL_MMIO.load(Ordering::Acquire);
    if addr != 0 {
        unsafe { core::ptr::read_volatile(addr as *const u64) as usize }
    } else {
        0
    }
}
