pub mod device;
pub mod platform;
pub mod smmu;

pub use device::diagnostics;
pub use device::init_nfc;
pub use device::is_initialized;
pub use device::read_nfc_reg;
pub use device::write_nfc_reg;
