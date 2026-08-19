pub mod device;
pub mod msi;
pub mod pci;
pub mod registers;

pub use device::diagnostics;
pub use device::init_nfc;
pub use device::is_initialized;
pub use device::nfc_mmio_base;
pub use device::nfc_mmio_size;
pub use device::read_nfc_reg;
pub use device::write_nfc_reg;
