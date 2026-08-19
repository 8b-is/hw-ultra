use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static VMX_ENABLED: AtomicU8 = AtomicU8::new(0);
static CR4_VMX_SHADOW: AtomicUsize = AtomicUsize::new(0);

pub fn is_supported() -> bool {
    crate::arch::x86_64::cpu::cpuid::has_feature_ecx(5)
}

pub fn enable_vmx() -> bool {
    if !is_supported() {
        return false;
    }
    let mut cr4 = CR4_VMX_SHADOW.load(Ordering::Acquire) as u64;
    cr4 |= 1u64 << 13;
    CR4_VMX_SHADOW.store(cr4 as usize, Ordering::Release);
    VMX_ENABLED.store(1, Ordering::Release);
    true
}

pub fn is_enabled() -> bool {
    VMX_ENABLED.load(Ordering::Acquire) != 0
}

pub fn read_vmx_basic() -> u64 {
    unsafe { crate::arch::x86_64::msr::read_msr(0x480) }
}
