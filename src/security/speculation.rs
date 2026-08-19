use core::sync::atomic::{AtomicU8, Ordering};

static MITIGATION_STATE: AtomicU8 = AtomicU8::new(0);

#[derive(Copy, Clone, PartialEq)]
pub enum SpeculationMitigation {
    None,
    Ibrs,
    Ibpb,
    Stibp,
    Ssbd,
    Retpoline,
}

pub fn mitigations_active() -> bool {
    MITIGATION_STATE.load(Ordering::Acquire) != 0
}

pub fn active_mitigation() -> SpeculationMitigation {
    match MITIGATION_STATE.load(Ordering::Acquire) {
        1 => SpeculationMitigation::Ibrs,
        2 => SpeculationMitigation::Ibpb,
        3 => SpeculationMitigation::Stibp,
        4 => SpeculationMitigation::Ssbd,
        5 => SpeculationMitigation::Retpoline,
        _ => SpeculationMitigation::None,
    }
}

pub fn mitigations() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some((eax, ebx, ecx, edx)) = crate::hardware_access::read_cpuid(7, 0) {
                static SPEC_SIG: core::sync::atomic::AtomicUsize =
                    core::sync::atomic::AtomicUsize::new(0);
                SPEC_SIG.store(
                    eax as usize ^ ebx as usize ^ ecx as usize,
                    core::sync::atomic::Ordering::Release,
                );
                if edx & (1 << 26) != 0 {
                    MITIGATION_STATE.store(1, Ordering::Release);
                    return;
                }
                if edx & (1 << 31) != 0 {
                    MITIGATION_STATE.store(4, Ordering::Release);
                    return;
                }
            }
            MITIGATION_STATE.store(5, Ordering::Release);
        }
        crate::arch::Architecture::AArch64 => {
            MITIGATION_STATE.store(1, Ordering::Release);
        }
        _ => {}
    }
}
