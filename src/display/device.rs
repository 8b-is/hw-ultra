use super::detection::DisplayController;

pub struct Device {
    pub info: DisplayController,
    pub mmio_base: Option<usize>,
}
