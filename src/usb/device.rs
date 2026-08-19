use super::detection::UsbController;

pub struct Device {
    pub info: UsbController,
    pub mmio_base: Option<usize>,
}
