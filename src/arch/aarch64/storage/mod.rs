pub mod device;
pub mod platform;
pub mod smmu;

pub use device::diagnostics;
pub use device::init_storage;
pub use device::is_initialized;
pub use device::read_storage_reg;
pub use device::write_storage_reg;
