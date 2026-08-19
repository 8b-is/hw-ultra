pub mod controller;
pub mod domain;
pub mod mapping;

pub fn init() {
    crate::iommu::mapping::map_iommu();
}

pub fn init_all() {
    crate::iommu::controller::init();
}
