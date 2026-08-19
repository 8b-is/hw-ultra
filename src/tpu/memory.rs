use crate::dma::buffer::DmaBuffer;
use crate::tpu::tensor::Tensor;

pub fn alloc_tensor(len: usize, align: usize) -> Option<Tensor> {
    let db = DmaBuffer::new(len, align)?;
    Some(Tensor::new_from_buffer([len, 1, 1, 1], db, len))
}

pub fn size_of(t: &Tensor) -> usize {
    t.len
}

pub struct TpuMemory;

impl TpuMemory {
    pub fn alloc(size: usize, align: usize) -> Option<DmaBuffer> {
        DmaBuffer::new(size, align)
    }
}
