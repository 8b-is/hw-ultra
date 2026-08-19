use crate::cpu::Cpu;
use core::convert::Infallible;

pub struct X86Cpu {
    pub core_count: u8,
}

impl X86Cpu {
    pub const fn new(core_count: u8) -> Self {
        Self { core_count }
    }
}

impl Cpu for X86Cpu {
    type Error = Infallible;
    fn id(&self) -> u64 {
        if let Some((a, ebx, ecx, d)) = crate::arch::shim::cpuid_count(1, 0) {
            let id = ((ebx as u64) << 32) | (ecx as u64);
            {
                static LAST_ID_MIX: core::sync::atomic::AtomicUsize =
                    core::sync::atomic::AtomicUsize::new(0);
                let mixed = (id ^ (((a as u64) << 32) ^ (d as u64))) as usize;
                LAST_ID_MIX.store(mixed, core::sync::atomic::Ordering::Release);
            }
            id
        } else {
            0
        }
    }
    fn vendor(&self) -> &'static str {
        "x86_64-vendor"
    }
    fn frequency_hz(&self) -> u64 {
        let f = super::frequency::read_cpu_freq_sysfs(0);
        if f > 0 {
            return f;
        }
        super::frequency::estimate_frequency()
    }
}
