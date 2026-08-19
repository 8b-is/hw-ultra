use core::sync::atomic::{AtomicUsize, Ordering};

const DESC_RING_SIZE: usize = 256;
const DESC_ENTRY_SIZE: usize = 16;
const DMA_RING_REG: usize = 0x200;
const DMA_HEAD_REG: usize = 0x208;
const DMA_TAIL_REG: usize = 0x210;
const DMA_CTRL_REG: usize = 0x218;

static RING_IOVA: AtomicUsize = AtomicUsize::new(0);
static RING_HEAD: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TpuDmaDescriptor {
    pub src_phys: u64,
    pub dst_phys: u64,
    pub length: u32,
    pub flags: u32,
}

pub fn setup_descriptor_ring(mmio_base: usize) -> usize {
    let ring_bytes = DESC_RING_SIZE * DESC_ENTRY_SIZE;
    let buf = crate::dma::buffer::DmaBuffer::new(ring_bytes, 4096);
    let iova = match buf {
        Some(ref b) => {
            let phys = b.phys_addr();
            if let Some(ctrl) = crate::iommu::controller::get() {
                if ctrl.is_enabled() {
                    ctrl.translate_iova(phys).unwrap_or(phys)
                } else {
                    phys
                }
            } else {
                phys
            }
        }
        None => 0,
    };

    RING_IOVA.store(iova, Ordering::Release);

    unsafe {
        super::super::mmio::mmio_write32(mmio_base + DMA_RING_REG, iova as u32);
        super::super::mmio::mmio_write32(mmio_base + DMA_RING_REG + 4, (iova >> 32) as u32);
        super::super::mmio::mmio_write32(mmio_base + DMA_HEAD_REG, 0);
        super::super::mmio::mmio_write32(mmio_base + DMA_TAIL_REG, 0);
        super::super::mmio::mmio_write32(mmio_base + DMA_CTRL_REG, 1);
    }

    iova
}

pub fn submit_transfer(src_phys: u64, dst_phys: u64, length: u32, flags: u32) -> bool {
    let base = super::device::mmio_base();
    if base == 0 {
        return false;
    }

    let head = RING_HEAD.fetch_add(1, Ordering::AcqRel) % DESC_RING_SIZE;
    let ring_base = RING_IOVA.load(Ordering::Acquire);
    if ring_base == 0 {
        return false;
    }

    let desc_offset = head * DESC_ENTRY_SIZE;
    let desc_addr = ring_base + desc_offset;
    unsafe {
        super::super::mmio::mmio_write32(desc_addr, src_phys as u32);
        super::super::mmio::mmio_write32(desc_addr + 4, dst_phys as u32);
        super::super::mmio::mmio_write32(desc_addr + 8, length);
        super::super::mmio::mmio_write32(desc_addr + 12, flags);
    }

    unsafe {
        super::super::mmio::mmio_write32(base + DMA_TAIL_REG, (head + 1) as u32);
    }
    true
}

pub fn ring_iova() -> usize {
    RING_IOVA.load(Ordering::Acquire)
}
