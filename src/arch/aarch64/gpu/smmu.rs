use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub const ATTR_CACHEABLE: u32 = 1 << 0;
pub const ATTR_SHAREABLE: u32 = 1 << 1;
pub const ATTR_READ: u32 = 1 << 2;
pub const ATTR_WRITE: u32 = 1 << 3;

const MAX_STREAMS: usize = 16;

static STREAM_COUNT: AtomicUsize = AtomicUsize::new(0);
static STREAM_IDS: [AtomicU32; MAX_STREAMS] = [const { AtomicU32::new(0) }; MAX_STREAMS];
static STREAM_ATTRS: [AtomicU32; MAX_STREAMS] = [const { AtomicU32::new(0) }; MAX_STREAMS];

pub fn configure_stream(device_base: usize, stream_id: u32) -> u32 {
    let idx = STREAM_COUNT.fetch_add(1, Ordering::AcqRel);
    if idx >= MAX_STREAMS {
        STREAM_COUNT.fetch_sub(1, Ordering::Release);
        return 0;
    }

    STREAM_IDS[idx].store(stream_id, Ordering::Release);

    if let Some(ctrl) = crate::iommu::controller::get() {
        if ctrl.is_enabled() {
            if let Some(iova) = ctrl.translate_iova(device_base) {
                STREAM_ATTRS[idx].store(iova as u32, Ordering::Release);
            }
        }
    }

    stream_id
}

pub fn set_attributes(device_base: usize, stream_id: u32, attrs: u32) {
    if device_base == 0 {
        return;
    }
    for i in 0..MAX_STREAMS {
        if STREAM_IDS[i].load(Ordering::Acquire) == stream_id {
            STREAM_ATTRS[i].store(attrs, Ordering::Release);
            return;
        }
    }
}

pub fn map_dma_for_device(phys: usize, size: usize) -> usize {
    if let Some(ctrl) = crate::iommu::controller::get() {
        if ctrl.is_enabled() {
            let buf = crate::dma::buffer::DmaBuffer::new(size, 65536);
            if let Some(ref b) = buf {
                if let Some(iova) = ctrl.map_dma_buffer(b, 65536) {
                    return iova;
                }
            }
        }
    }
    phys
}

pub fn get_stream_attrs(stream_id: u32) -> u32 {
    for i in 0..MAX_STREAMS {
        if STREAM_IDS[i].load(Ordering::Acquire) == stream_id {
            return STREAM_ATTRS[i].load(Ordering::Acquire);
        }
    }
    0
}

pub fn stream_count() -> usize {
    STREAM_COUNT.load(Ordering::Acquire)
}
