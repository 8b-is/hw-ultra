pub mod device;
pub mod msi;
pub mod pci;
pub mod registers;

pub use device::audio_mmio_base;
pub use device::audio_mmio_size;
pub use device::diagnostics;
pub use device::init_audio;
pub use device::is_initialized;
pub use device::read_audio_reg;
pub use device::write_audio_reg;
