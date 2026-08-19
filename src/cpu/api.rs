// Common CPU API — re-exports from internal modules.
// ARM + x86 unified public interface for external consumers (via sys/).

pub use super::cores::{detect_cores, CoreInfo};
pub use super::detect::detect_cpu_info;
pub use super::frequency::calibrate_tsc;
pub use super::info::{fill_cpu_component, model_name_str};
pub use super::info::{ComponentStatus, CpuInfo};
pub use super::ram::{detect_ram, RamInfo};
pub use super::thermal::read_core_temperatures;
