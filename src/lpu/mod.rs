pub mod device;
pub mod drivers;
pub mod inference;
pub mod memory;
pub mod pipeline;
pub mod quantization;
pub mod scheduler;

pub trait Lpu {
    type Error;
}

pub mod lifecycle;
