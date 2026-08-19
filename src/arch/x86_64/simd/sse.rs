use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static SSE_ENABLED: AtomicU8 = AtomicU8::new(0);
static CR0_SHADOW: AtomicUsize = AtomicUsize::new(0);
static CR4_SSE_SHADOW: AtomicUsize = AtomicUsize::new(0);

pub fn is_supported() -> bool {
    crate::arch::x86_64::cpu::cpuid::has_feature_edx(25)
}

pub fn enable() -> bool {
    if !is_supported() {
        return false;
    }
    let mut cr0 = CR0_SHADOW.load(Ordering::Acquire) as u64;
    cr0 &= !(1u64 << 2);
    cr0 |= 1u64 << 1;
    CR0_SHADOW.store(cr0 as usize, Ordering::Release);
    let mut cr4 = CR4_SSE_SHADOW.load(Ordering::Acquire) as u64;
    cr4 |= 1u64 << 9;
    cr4 |= 1u64 << 10;
    CR4_SSE_SHADOW.store(cr4 as usize, Ordering::Release);
    SSE_ENABLED.store(1, Ordering::Release);
    true
}

pub fn is_enabled() -> bool {
    SSE_ENABLED.load(Ordering::Acquire) != 0
}
