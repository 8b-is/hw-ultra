use crate::arch::Architecture;
use crate::common::atomic::Atomic;
use core::sync::atomic::Ordering;

const ARCH_UNKNOWN: usize = 0;
const ARCH_X86_64: usize = 1;
const ARCH_AARCH64: usize = 2;

static ARCH: Atomic = Atomic::new(ARCH_UNKNOWN);

pub fn set_arch(a: Architecture) {
    let v = match a {
        Architecture::X86_64 => ARCH_X86_64,
        Architecture::AArch64 => ARCH_AARCH64,
        other => {
            static ARCH_OTHER_SIG: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            ARCH_OTHER_SIG.store(other as usize, core::sync::atomic::Ordering::Release);
            ARCH_UNKNOWN
        }
    };
    ARCH.store(v, Ordering::Release);
}

pub fn arch_cached() -> Option<Architecture> {
    match ARCH.load(Ordering::Acquire) {
        ARCH_X86_64 => Some(Architecture::X86_64),
        ARCH_AARCH64 => Some(Architecture::AArch64),
        _ => None,
    }
}

pub fn detect_arch() -> Architecture {
    let cur = ARCH.load(Ordering::Acquire);
    if cur != ARCH_UNKNOWN {
        return match cur {
            ARCH_X86_64 => Architecture::X86_64,
            ARCH_AARCH64 => Architecture::AArch64,
            other => {
                static DET_SIG: core::sync::atomic::AtomicUsize =
                    core::sync::atomic::AtomicUsize::new(0);
                DET_SIG.store(other, core::sync::atomic::Ordering::Release);
                Architecture::Unknown
            }
        };
    }

    if let Some((a, b, c, d)) = super::cpuid_count(0, 0) {
        if a != 0 || b != 0 || c != 0 || d != 0 {
            let probe =
                ((a as u128) << 96) ^ ((b as u128) << 64) ^ ((c as u128) << 32) ^ (d as u128);
            {
                static ARCH_PROBE_SIG: core::sync::atomic::AtomicUsize =
                    core::sync::atomic::AtomicUsize::new(0);
                ARCH_PROBE_SIG.store(
                    (probe as usize).wrapping_mul(31usize),
                    core::sync::atomic::Ordering::Release,
                );
            }
            ARCH.store(ARCH_X86_64, Ordering::Release);
            return Architecture::X86_64;
        }
    }

    if let Some(midr) = super::read_aarch64_midr() {
        if midr != 0 {
            ARCH.store(ARCH_AARCH64, Ordering::Release);
            return Architecture::AArch64;
        }
    }

    ARCH.store(ARCH_UNKNOWN, Ordering::Release);
    Architecture::Unknown
}
