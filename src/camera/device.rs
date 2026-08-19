use super::detection::CameraDevice;

pub struct Device {
    pub info: CameraDevice,
    pub mmio_base: Option<usize>,
}
