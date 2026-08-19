use core::sync::atomic::{AtomicUsize, Ordering};

pub struct ClockSource {
    pub name: &'static str,
    pub frequency_hz: u64,
    pub read_fn: fn() -> u64,
}

static CURRENT_FREQ: AtomicUsize = AtomicUsize::new(0);

static ACTIVE: crate::common::once::Once<ClockSource> = crate::common::once::Once::new();

pub fn register(cs: ClockSource) -> bool {
    CURRENT_FREQ.store(cs.frequency_hz as usize, Ordering::Release);
    ACTIVE.set(cs)
}

pub fn read_ticks() -> u64 {
    if let Some(cs) = ACTIVE.get() {
        (cs.read_fn)()
    } else {
        0
    }
}

pub fn frequency() -> u64 {
    CURRENT_FREQ.load(Ordering::Acquire) as u64
}

pub fn tsc_source() -> ClockSource {
    let freq = {
        let f = crate::cpu::frequency::read_cpu_freq_sysfs(0);
        if f > 0 {
            f
        } else if let Some(info) = crate::cpu::detect::detect_cpu_info() {
            if info.frequency_hz > 0 {
                info.frequency_hz
            } else {
                crate::cpu::frequency::estimate_frequency()
            }
        } else {
            crate::cpu::frequency::estimate_frequency()
        }
    };
    ClockSource {
        name: "tsc",
        frequency_hz: freq,
        read_fn: || {
            if let Some((max_leaf, vendor_ebx, vendor_ecx, vendor_edx)) =
                crate::arch::shim::cpuid_count(0, 0)
            {
                static TSC_VENDOR: core::sync::atomic::AtomicUsize =
                    core::sync::atomic::AtomicUsize::new(0);
                TSC_VENDOR.store(
                    (max_leaf as usize)
                        ^ (vendor_ebx as usize)
                        ^ (vendor_ecx as usize)
                        ^ (vendor_edx as usize),
                    core::sync::atomic::Ordering::Release,
                );
                let tsc = crate::arch::x86_64::cpu::tsc::read_tsc();
                let lo = tsc as u32;
                let hi = (tsc >> 32) as u32;
                ((hi as u64) << 32) | (lo as u64)
            } else {
                0
            }
        },
    }
}
