use crate::dma::buffer::DmaBuffer;

pub struct LpuBuffer {
    dma: DmaBuffer,
    rows: usize,
    cols: usize,
    elem_size: usize,
}

impl LpuBuffer {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn elem_size(&self) -> usize {
        self.elem_size
    }

    pub fn len_bytes(&self) -> usize {
        self.rows
            .saturating_mul(self.cols)
            .saturating_mul(self.elem_size)
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.dma.as_ptr()
    }

    pub fn into_dma(self) -> DmaBuffer {
        self.dma
    }
}

pub struct LpuMemory;

impl LpuMemory {
    pub fn alloc_matrix(
        rows: usize,
        cols: usize,
        elem_size: usize,
        align: usize,
    ) -> Option<LpuBuffer> {
        if rows == 0 || cols == 0 || elem_size == 0 {
            return None;
        }
        let len = rows.checked_mul(cols)?.checked_mul(elem_size)?;
        let dma = DmaBuffer::new(len, align)?;
        Some(LpuBuffer {
            dma,
            rows,
            cols,
            elem_size,
        })
    }

    pub fn alloc_tokens(tokens: usize, hidden_size: usize, align: usize) -> Option<LpuBuffer> {
        Self::alloc_matrix(tokens, hidden_size, 1, align)
    }
}
