use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static AVX_ENABLED: AtomicU8 = AtomicU8::new(0);
static CR4_AVX_SHADOW: AtomicUsize = AtomicUsize::new(0);
static XCR0_SHADOW: AtomicUsize = AtomicUsize::new(0);

pub fn is_supported() -> bool {
    crate::arch::x86_64::cpu::cpuid::has_feature_ecx(28)
}

pub fn enable() -> bool {
    if !is_supported() {
        return false;
    }
    let mut cr4 = CR4_AVX_SHADOW.load(Ordering::Acquire) as u64;
    cr4 |= 1u64 << 18;
    CR4_AVX_SHADOW.store(cr4 as usize, Ordering::Release);
    XCR0_SHADOW.store(0x7, Ordering::Release);
    AVX_ENABLED.store(1, Ordering::Release);
    true
}

pub fn is_enabled() -> bool {
    AVX_ENABLED.load(Ordering::Acquire) != 0
}
