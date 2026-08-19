use crate::cpu::Cpu;
use core::convert::Infallible;

pub struct Aarch64Cpu {
    pub cluster_id: u8,
}

impl Aarch64Cpu {
    pub const fn new(cluster_id: u8) -> Self {
        Self { cluster_id }
    }
}

impl Cpu for Aarch64Cpu {
    type Error = Infallible;
    fn id(&self) -> u64 {
        crate::arch::shim::read_aarch64_midr().unwrap_or_default()
    }
    fn vendor(&self) -> &'static str {
        "aarch64-vendor"
    }
    fn frequency_hz(&self) -> u64 {
        let f = super::frequency::read_cpu_freq_sysfs(0);
        if f > 0 {
            f
        } else {
            0
        }
    }
}
