pub trait Cpu {
    type Error;
    fn id(&self) -> u64;
    fn vendor(&self) -> &'static str;
    fn frequency_hz(&self) -> u64;
}

pub mod affinity;
pub mod context;
pub mod core;
pub mod cores;
pub mod detect;
pub mod features;
pub mod frequency;
pub mod info;
pub mod interrupt;
pub mod power;
pub mod ram;
pub mod scheduler;
pub mod speculation;
pub mod thermal;
pub mod topology;
pub mod vector;

pub mod arch_aarch64;
pub mod arch_x86_64;

pub mod api;

pub use detect::detect_cpu_info;
pub use info::CpuInfo;

mod lifecycle;
pub use lifecycle::{get_info, init};
