use crate::hardware_access::{mmio_read32, mmio_write32, read_msr};

pub struct Apic {
    base: usize,
}

impl Apic {
    pub fn probe() -> Apic {
        let base = if let Some(msr) = read_msr(0x1B) {
            ((msr >> 12) as usize) << 12
        } else {
            0xFEE0_0000usize
        };

        let ver = mmio_read32(base + 0x30).unwrap_or(0xffff_ffff);
        if ver == 0 || ver == 0xffff_ffff {
            Apic { base: 0 }
        } else {
            Apic { base }
        }
    }

    pub fn init(&self) -> bool {
        if self.base == 0 {
            return false;
        }
        let spurious = mmio_read32(self.base + 0xF0).unwrap_or(0) | (1 << 8);
        mmio_write32(self.base + 0xF0, spurious);
        true
    }

    pub fn id(&self) -> u32 {
        if self.base == 0 {
            return 0;
        }
        let id = mmio_read32(self.base + 0x20).unwrap_or(0);
        (id >> 24) & 0xff
    }

    pub fn eoi(&self) {
        if self.base == 0 {
            return;
        }
        mmio_write32(self.base + 0xB0, 0);
    }
}
