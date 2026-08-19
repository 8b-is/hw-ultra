use crate::dma::buffer::DmaBuffer;
use crate::iommu::controller;

pub struct GpuAllocator;

impl GpuAllocator {
    pub fn alloc_framebuffer(size: usize, align: usize) -> Option<DmaBuffer> {
        DmaBuffer::new(size, align)
    }

    pub fn map_for_device(buf: &DmaBuffer, align: usize) -> Option<usize> {
        if let Some(ctrl) = controller::get() {
            ctrl.map_dma_buffer(buf, align)
        } else {
            Some(buf.phys_addr())
        }
    }

    pub fn unmap_iova(iova: usize) {
        let ok = crate::dma::engine::DmaEngine::unmap_iova(iova);
        debug_assert!(ok);
    }
}
