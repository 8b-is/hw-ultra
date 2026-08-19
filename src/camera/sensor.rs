use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum SensorType {
    Cmos,
    Ccd,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BayerPattern {
    Rggb,
    Bggr,
    Grbg,
    Gbrg,
}

#[derive(Clone, Copy)]
pub struct SensorInfo {
    pub sensor_type: SensorType,
    pub bayer: BayerPattern,
    pub max_width: u16,
    pub max_height: u16,
    pub pixel_depth: u8,
    pub sensor_id: u16,
}

static ACTIVE_WIDTH: AtomicU16 = AtomicU16::new(0);
static ACTIVE_HEIGHT: AtomicU16 = AtomicU16::new(0);
static SENSOR_ID: AtomicU32 = AtomicU32::new(0);

pub fn set_active_resolution(width: u16, height: u16) {
    ACTIVE_WIDTH.store(width, Ordering::Release);
    ACTIVE_HEIGHT.store(height, Ordering::Release);
}

pub fn active_width() -> u16 {
    ACTIVE_WIDTH.load(Ordering::Acquire)
}

pub fn active_height() -> u16 {
    ACTIVE_HEIGHT.load(Ordering::Acquire)
}

pub fn register_sensor(id: u16) {
    SENSOR_ID.store(id as u32, Ordering::Release);
}

pub fn current_sensor_id() -> u16 {
    SENSOR_ID.load(Ordering::Acquire) as u16
}

pub fn megapixels(info: &SensorInfo) -> u32 {
    (info.max_width as u32 * info.max_height as u32) / 1_000_000
}

pub fn raw_line_bytes(width: u16, depth: u8) -> usize {
    let bits = width as usize * depth as usize;
    bits.div_ceil(8)
}
