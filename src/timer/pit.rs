use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Pit;

const PIT_FREQ: u32 = 1193182;
const PIT_CMD: u16 = 0x43;
const PIT_CH0: u16 = 0x40;

static PIT_DIVISOR: AtomicUsize = AtomicUsize::new(0);

impl Pit {
    pub fn set_frequency(hz: u32) -> u32 {
        if hz == 0 {
            return 0;
        }
        let divisor = PIT_FREQ / hz;
        let divisor16 = if divisor > 0xFFFF {
            0xFFFF
        } else if divisor == 0 {
            1
        } else {
            divisor as u16
        };
        PIT_DIVISOR.store(divisor16 as usize, Ordering::Release);
        unsafe {
            crate::arch::x86_64::io::outb(PIT_CMD, 0x36);
            crate::arch::x86_64::io::outb(PIT_CH0, (divisor16 & 0xFF) as u8);
            crate::arch::x86_64::io::outb(PIT_CH0, ((divisor16 >> 8) & 0xFF) as u8);
        }
        PIT_FREQ / (divisor16 as u32)
    }

    pub fn read_count() -> u16 {
        unsafe {
            crate::arch::x86_64::io::outb(PIT_CMD, 0x00);
            let lo = crate::arch::x86_64::io::inb(PIT_CH0) as u16;
            let hi = crate::arch::x86_64::io::inb(PIT_CH0) as u16;
            (hi << 8) | lo
        }
    }
}
