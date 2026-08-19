use crate::dma::buffer::DmaBuffer;

pub struct Tensor {
    pub dims: [usize; 4],
    buf: Option<DmaBuffer>,
    pub len: usize,
}

impl Tensor {
    pub fn new_from_buffer(dims: [usize; 4], buf: DmaBuffer, len: usize) -> Self {
        Tensor {
            dims,
            buf: Some(buf),
            len,
        }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.buf
            .as_ref()
            .map(|b| b.as_ptr())
            .unwrap_or(core::ptr::null_mut())
    }

    pub fn numel(&self) -> usize {
        self.dims.iter().product()
    }

    pub fn take_buffer(&mut self) -> Option<DmaBuffer> {
        core::mem::take(&mut self.buf)
    }
}
