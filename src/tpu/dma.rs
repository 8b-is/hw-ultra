pub fn chunk_size() -> usize {
    256
}

pub fn max_chunks_for(len: usize) -> usize {
    len.div_ceil(chunk_size())
}

use crate::dma::buffer::DmaBuffer;
use crate::dma::engine::DmaEngine;

pub struct TpuDma;

impl TpuDma {
    pub fn submit(buf: &DmaBuffer, flags: u32, align: usize) -> Result<usize, &'static str> {
        let eng = DmaEngine::get().ok_or("no dma engine")?;
        eng.submit_buffer(buf, flags, align)
    }
}
