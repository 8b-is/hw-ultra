use core::sync::atomic::{AtomicUsize, Ordering};

const DMA_BUF_REG: usize = 0x200;
const DMA_SIZE_REG: usize = 0x208;
const DMA_CTRL_REG: usize = 0x210;
const DMA_STATUS_REG: usize = 0x214;
const DMA_REGION_SIZE: usize = 16 * 1024 * 1024;
const DMA_GRANULE: usize = 65536;

static DMA_REGION_BASE: AtomicUsize = AtomicUsize::new(0);
static DMA_OFFSET: AtomicUsize = AtomicUsize::new(0);

pub fn setup_dma_region(mmio_base: usize) -> usize {
    let buf = crate::dma::buffer::DmaBuffer::new(DMA_REGION_SIZE, DMA_GRANULE);
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

    DMA_REGION_BASE.store(phys, Ordering::Release);

    crate::hardware_access::mmio_write32(mmio_base + DMA_BUF_REG, phys as u32);
    crate::hardware_access::mmio_write32(mmio_base + DMA_BUF_REG + 4, (phys >> 32) as u32);
    crate::hardware_access::mmio_write32(mmio_base + DMA_SIZE_REG, DMA_REGION_SIZE as u32);
    crate::hardware_access::mmio_write32(mmio_base + DMA_CTRL_REG, 1);

    phys
}

pub fn alloc_dma_buffer(size: usize) -> Option<usize> {
    let base = DMA_REGION_BASE.load(Ordering::Acquire);
    if base == 0 {
        return None;
    }
    let aligned = (size + DMA_GRANULE - 1) & !(DMA_GRANULE - 1);
    let off = DMA_OFFSET.fetch_add(aligned, Ordering::AcqRel);
    if off + aligned > DMA_REGION_SIZE {
        return None;
    }
    Some(base + off)
}

pub fn clean_and_invalidate(va_start: usize, size: usize) {
    let line_size = 64usize;
    let mut addr = va_start & !(line_size - 1);
    let end = va_start + size;
    if crate::arch::detect_arch() == crate::arch::architecture::Architecture::AArch64 {
        while addr < end {
            unsafe { super::super::sysreg::dc_civac(addr) }
            addr += line_size;
        }
        unsafe { super::super::sysreg::dsb_ish() }
    } else {
        while addr < end {
            unsafe {
                core::ptr::write_volatile(
                    addr as *mut u8,
                    core::ptr::read_volatile(addr as *const u8),
                );
            }
            addr += line_size;
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    }
}

pub fn submit_dma_transfer(src_phys: u64, size: u32) -> bool {
    let base = super::device::mmio_base();
    if base == 0 {
        return false;
    }
    crate::hardware_access::mmio_write32(base + DMA_BUF_REG, src_phys as u32);
    crate::hardware_access::mmio_write32(base + DMA_BUF_REG + 4, (src_phys >> 32) as u32);
    crate::hardware_access::mmio_write32(base + DMA_SIZE_REG, size);
    crate::hardware_access::mmio_write32(base + DMA_CTRL_REG, 0x03);
    true
}

pub fn is_dma_complete() -> bool {
    let base = super::device::mmio_base();
    if base == 0 {
        return false;
    }
    let status = crate::hardware_access::mmio_read32(base + DMA_STATUS_REG).unwrap_or(0);
    status & 0x01 != 0
}

pub fn dma_region_base() -> usize {
    DMA_REGION_BASE.load(Ordering::Acquire)
}
