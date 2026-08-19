pub mod aarch64;
pub mod guardian;
pub mod shim;
pub mod x86_64;

pub use shim::{
    cpuid_count, detect_arch, mmio_read32, mmio_write32, read_aarch64_midr, read_msr, set_arch,
};
pub mod architecture;
pub use architecture::Architecture;
pub use shim::init_shims;
