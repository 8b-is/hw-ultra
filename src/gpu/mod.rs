pub mod command;
pub mod compute;
mod detection;
pub mod device;
pub mod drivers;
pub mod drm;
pub mod hw;
pub mod memory;
pub mod pipeline;
pub mod queue;
pub mod scheduler;
pub mod shader;

pub use command::Command;
pub use detection::adreno_product_name;
pub use detection::detect_gpus_impl as detect_gpus;
pub use detection::mali_gpu_version;
pub use detection::mali_product_name;
pub use detection::parse_adreno_chip_id;
pub use detection::parse_mali_gpu_id;
pub use detection::set_detect_gpu_fn;
pub use detection::GpuDevice;
pub use detection::RawGpuId;
pub use drm::set_drm_constants;
pub use drm::DrmConstants;
pub use shader::Shader;

mod lifecycle;
pub use lifecycle::init;
