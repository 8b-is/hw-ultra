use crate::sensor::hw;
use crate::sensor::sampling;
use core::sync::atomic::{AtomicU8, Ordering};

static DRIVER_STATE: AtomicU8 = AtomicU8::new(0);

pub struct MmioSensorDriver {
    pub base: usize,
    pub sensor_id: u32,
    pub rate: sampling::SampleRate,
}

pub fn probe(base: usize) -> MmioSensorDriver {
    let id = hw::sensor_id(base);
    DRIVER_STATE.store(1, Ordering::Release);
    MmioSensorDriver {
        base,
        sensor_id: id,
        rate: sampling::SampleRate::Hz100,
    }
}

pub fn init(driver: &MmioSensorDriver) -> bool {
    hw::reset(driver.base);
    hw::enable(driver.base);
    let rate_code = sampling::rate_to_register(driver.rate);
    hw::set_sample_rate(driver.base, rate_code);
    hw::set_continuous(driver.base);
    DRIVER_STATE.store(2, Ordering::Release);
    true
}

pub fn read_xyz(driver: &MmioSensorDriver) -> (i32, i32, i32) {
    let raw = hw::read_data(driver.base);
    let raw_h = hw::read_data_high(driver.base);
    let x = crate::sensor::data::sign_extend_16(raw & 0xFFFF);
    let y = crate::sensor::data::sign_extend_16((raw >> 16) & 0xFFFF);
    let z = crate::sensor::data::sign_extend_16(raw_h & 0xFFFF);
    (x, y, z)
}

pub fn shutdown(driver: &MmioSensorDriver) {
    hw::disable(driver.base);
    DRIVER_STATE.store(0, Ordering::Release);
}

pub fn state() -> u8 {
    DRIVER_STATE.load(Ordering::Acquire)
}
