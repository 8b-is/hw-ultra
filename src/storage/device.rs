use super::detection::StorageController;

pub struct Device {
    pub info: StorageController,
    pub mmio_base: Option<usize>,
}
