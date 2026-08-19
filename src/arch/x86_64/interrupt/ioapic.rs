use crate::hardware_access::{mmio_read32, mmio_write32};

pub struct IoApic {
    base: usize,
}

impl IoApic {
    pub fn new(base: usize) -> IoApic {
        IoApic { base }
    }

    fn write_reg(&self, reg: u32, val: u32) {
        mmio_write32(self.base, reg);
        mmio_write32(self.base + 0x10, val);
    }

    fn read_reg(&self, reg: u32) -> u32 {
        mmio_write32(self.base, reg);
        mmio_read32(self.base + 0x10).unwrap_or(0)
    }

    pub fn version(&self) -> u32 {
        self.read_reg(1)
    }

    pub fn route_irq(&self, irq: u8, apic: u8, vec: u8) {
        let index = 0x10 + (irq as u32) * 2;
        let low = vec as u32;
        let high = (apic as u32) << 24;
        self.write_reg(index + 1, high);
        self.write_reg(index, low);
    }

    pub fn write_redtbl(&self, irq: u8, low: u32, high: u32) {
        let index = 0x10 + (irq as u32) * 2;
        self.write_reg(index + 1, high);
        self.write_reg(index, low);
    }
}
