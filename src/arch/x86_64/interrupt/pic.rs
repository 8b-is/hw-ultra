use crate::arch::x86_64::io::{inb, outb};
use crate::interrupt::IrqController;
use core::sync::atomic::{AtomicU16, Ordering};

pub struct Pic8259;

impl Default for Pic8259 {
    fn default() -> Self {
        Self::new()
    }
}

impl Pic8259 {
    pub const fn new() -> Self {
        Pic8259
    }

    pub fn read_mask(&self) -> u16 {
        let a = unsafe { inb(0x21) } as u16;
        let b = unsafe { inb(0xA1) } as u16;
        (b << 8) | a
    }

    fn write_mask(&self, mask: u16) {
        unsafe {
            outb(0x21, (mask & 0xff) as u8);
        }
        unsafe {
            outb(0xA1, ((mask >> 8) & 0xff) as u8);
        }
    }
}

static PIC_MASK: AtomicU16 = AtomicU16::new(0xffff);

impl IrqController for Pic8259 {
    fn init(&self) -> bool {
        unsafe {
            outb(0x20, 0x11);
            outb(0xA0, 0x11);
            outb(0x21, 0x20);
            outb(0xA1, 0x28);
            outb(0x21, 0x04);
            outb(0xA1, 0x02);
            outb(0x21, 0x01);
            outb(0xA1, 0x01);
            outb(0x21, 0xff);
            outb(0xA1, 0xff);
        }
        PIC_MASK.store(0xffff, Ordering::Release);
        true
    }

    fn enable_irq(&self, irq: u8) {
        let mut mask = PIC_MASK.load(Ordering::Acquire);
        mask &= !(1u16 << (irq as u16));
        PIC_MASK.store(mask, Ordering::Release);
        self.write_mask(mask);
    }

    fn disable_irq(&self, irq: u8) {
        let mut mask = PIC_MASK.load(Ordering::Acquire);
        mask |= 1u16 << (irq as u16);
        PIC_MASK.store(mask, Ordering::Release);
        self.write_mask(mask);
    }

    fn eoi(&self, irq: u8) {
        let cmd: u8 = 0x20;
        unsafe {
            outb(0x20, cmd);
        }
        if irq >= 8 {
            unsafe {
                outb(0xA0, cmd);
            }
        }
    }
}
