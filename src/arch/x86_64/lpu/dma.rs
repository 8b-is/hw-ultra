use core::sync::atomic::{AtomicUsize, Ordering};

const DMA_BUF_REG: usize = 0x300;
const DMA_SIZE_REG: usize = 0x308;
const DMA_CTRL_REG: usize = 0x310;
const DMA_STATUS_REG: usize = 0x314;

const COHERENT_REGION_SIZE: usize = 4 * 1024 * 1024;
const COHERENT_ALIGN: usize = 4096;

static COHERENT_BASE: AtomicUsize = AtomicUsize::new(0);
static COHERENT_OFFSET: AtomicUsize = AtomicUsize::new(0);

pub fn setup_coherent_region(mmio_base: usize) -> usize {
    let buf = crate::dma::buffer::DmaBuffer::new(COHERENT_REGION_SIZE, COHERENT_ALIGN);
    let phys = match buf {
        Some(ref b) => {
            let p = b.phys_addr();
            if let Some(ctrl) = crate::iommu::controller::get() {
                if ctrl.is_enabled() {
                    ctrl.translate_iova(p).unwrap_or(p)
                } else {
                    p
                }
            } else {
                p
            }
        }
        None => 0,
    };

    COHERENT_BASE.store(phys, Ordering::Release);

    unsafe {
        super::super::mmio::mmio_write32(mmio_base + DMA_BUF_REG, phys as u32);
        super::super::mmio::mmio_write32(mmio_base + DMA_BUF_REG + 4, (phys >> 32) as u32);
        super::super::mmio::mmio_write32(mmio_base + DMA_SIZE_REG, COHERENT_REGION_SIZE as u32);
        super::super::mmio::mmio_write32(mmio_base + DMA_CTRL_REG, 1);
    }

    phys
}

pub fn alloc_coherent(size: usize) -> Option<usize> {
    let base = COHERENT_BASE.load(Ordering::Acquire);
    if base == 0 {
        return None;
    }
    let aligned_size = (size + COHERENT_ALIGN - 1) & !(COHERENT_ALIGN - 1);
    let off = COHERENT_OFFSET.fetch_add(aligned_size, Ordering::AcqRel);
    if off + aligned_size > COHERENT_REGION_SIZE {
        return None;
    }
    Some(base + off)
}

pub fn submit_inference_dma(token_phys: u64, token_count: u32) -> bool {
    let base = super::device::mmio_base();
    if base == 0 {
        return false;
    }
    unsafe {
        super::super::mmio::mmio_write32(base + DMA_BUF_REG, token_phys as u32);
        super::super::mmio::mmio_write32(base + DMA_BUF_REG + 4, (token_phys >> 32) as u32);
        super::super::mmio::mmio_write32(base + DMA_SIZE_REG, token_count);
        super::super::mmio::mmio_write32(base + DMA_CTRL_REG, 0x03);
    }
    true
}

pub fn is_dma_complete() -> bool {
    let base = super::device::mmio_base();
    if base == 0 {
        return false;
    }
    let status = unsafe { super::super::mmio::mmio_read32(base + DMA_STATUS_REG) };
    status & 0x01 != 0
}

pub fn coherent_base() -> usize {
    COHERENT_BASE.load(Ordering::Acquire)
}
