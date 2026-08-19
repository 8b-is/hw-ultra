pub mod compiler;
pub mod device;
pub mod dma;
pub mod drivers;
pub mod executor;
pub mod graph;
pub mod memory;
pub mod runtime;
pub mod tensor;

pub trait Tpu {
    type Error;
}

pub mod lifecycle;
