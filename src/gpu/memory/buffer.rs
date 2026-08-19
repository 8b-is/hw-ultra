use crate::dma::buffer::DmaBuffer;

pub struct Buffer {
    dma: DmaBuffer,
    iova: Option<usize>,
}

impl Buffer {
    pub fn new(size: usize, align: usize) -> Option<Self> {
        let d = DmaBuffer::new(size, align)?;
        Some(Buffer { dma: d, iova: None })
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.dma.as_ptr()
    }
    pub fn len(&self) -> usize {
        self.dma.len()
    }
    pub fn is_empty(&self) -> bool {
        self.dma.is_empty()
    }

    pub fn phys_addr(&self) -> usize {
        self.dma.phys_addr()
    }

    pub fn set_iova(&mut self, iova: usize) {
        self.iova = Some(iova);
    }
    pub fn iova(&self) -> Option<usize> {
        self.iova
    }
}
