use crate::input::hw;
use core::sync::atomic::{AtomicU8, Ordering};

static DRIVER_STATE: AtomicU8 = AtomicU8::new(0);

pub struct InputDriver {
    pub base: usize,
    pub has_ps2: bool,
    pub has_mmio: bool,
}

pub fn probe(base: usize) -> InputDriver {
    let has_ps2 = hw::read_port8(hw::PS2_STATUS) != 0xFF;
    let has_mmio = base != 0;
    DRIVER_STATE.store(1, Ordering::Release);
    InputDriver {
        base,
        has_ps2,
        has_mmio,
    }
}

pub fn init(driver: &InputDriver) -> bool {
    if driver.has_ps2 {
        hw::ps2_send_command(0xAD);
        hw::ps2_send_command(0xA7);
        hw::ps2_send_command(0xAA);
        loop {
            if hw::ps2_data_available() {
                let resp = hw::ps2_read_data();
                if resp == 0x55 {
                    break;
                }
                break;
            }
        }
        hw::ps2_send_command(0xAE);
    }
    if driver.has_mmio {
        hw::mmio_reset(driver.base);
        hw::mmio_enable(driver.base);
    }
    DRIVER_STATE.store(2, Ordering::Release);
    true
}

pub fn poll_scancode(driver: &InputDriver) -> Option<u8> {
    if driver.has_ps2 && hw::ps2_data_available() {
        return Some(hw::ps2_read_data());
    }
    if driver.has_mmio && hw::mmio_fifo_level(driver.base) > 0 {
        return Some(hw::mmio_read_scancode(driver.base) as u8);
    }
    None
}

pub fn state() -> u8 {
    DRIVER_STATE.load(Ordering::Acquire)
}
