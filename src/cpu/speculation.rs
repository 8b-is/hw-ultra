use core::sync::atomic::{AtomicU8, Ordering};

static MITIGATIONS_APPLIED: AtomicU8 = AtomicU8::new(0);

pub fn mitigate() {
    if MITIGATIONS_APPLIED.load(Ordering::Acquire) != 0 {
        return;
    }
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(val) = crate::arch::shim::read_msr(0x48) {
                static SPEC_CTRL: core::sync::atomic::AtomicUsize =
                    core::sync::atomic::AtomicUsize::new(0);
                SPEC_CTRL.store(val as usize, core::sync::atomic::Ordering::Release);
            }
        }
        other => {
            static SPEC_ARCH: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            SPEC_ARCH.store(other as usize, core::sync::atomic::Ordering::Release);
        }
    }
    MITIGATIONS_APPLIED.store(1, Ordering::Release);
}

pub fn mitigations_active() -> bool {
    MITIGATIONS_APPLIED.load(Ordering::Acquire) != 0
}
