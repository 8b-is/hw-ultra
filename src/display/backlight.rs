use core::sync::atomic::{AtomicU8, Ordering};

static BRIGHTNESS: AtomicU8 = AtomicU8::new(255);

pub fn set_brightness(base: usize, level: u8) {
    BRIGHTNESS.store(level, Ordering::Release);
    super::hw::write_reg(base, 0x40, level as u32);
}

pub fn get_brightness() -> u8 {
    BRIGHTNESS.load(Ordering::Acquire)
}

pub fn enable(base: usize) {
    super::hw::write_reg(base, 0x44, 1);
}

pub fn disable(base: usize) {
    super::hw::write_reg(base, 0x44, 0);
}
