pub mod device;
pub mod platform;
pub mod smmu;

pub use device::diagnostics;
pub use device::init_display;
pub use device::is_initialized;
pub use device::read_display_reg;
pub use device::write_display_reg;
