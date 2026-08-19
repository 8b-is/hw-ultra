use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static EDGE_LPU_BASE: AtomicUsize = AtomicUsize::new(0);
static EDGE_LPU_INIT: AtomicBool = AtomicBool::new(false);
static EDGE_INFER_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct EdgeLpu {
    pub base: usize,
}

impl EdgeLpu {
    pub fn probe(base: usize) -> Option<Self> {
        let id = crate::hardware_access::mmio_read32(base).unwrap_or(0);
        if id == 0 || id == 0xFFFF_FFFF {
            return None;
        }
        EDGE_LPU_BASE.store(base, Ordering::Release);
        Some(EdgeLpu { base })
    }

    pub fn init(&self) -> bool {
        crate::hardware_access::mmio_write32(self.base, 0x01);
        let status = crate::hardware_access::mmio_read32(self.base + 0x08).unwrap_or(0);
        if status & 0x01 != 0 {
            EDGE_LPU_INIT.store(true, Ordering::Release);
            true
        } else {
            false
        }
    }

    pub fn run_inference(&self, model_addr: usize, input_addr: usize, output_addr: usize) -> bool {
        if !EDGE_LPU_INIT.load(Ordering::Acquire) {
            return false;
        }
        crate::hardware_access::mmio_write32(self.base + 0x20, model_addr as u32);
        crate::hardware_access::mmio_write32(self.base + 0x24, input_addr as u32);
        crate::hardware_access::mmio_write32(self.base + 0x28, output_addr as u32);
        crate::hardware_access::mmio_write32(self.base + 0x2C, 1);
        EDGE_INFER_COUNT.fetch_add(1, Ordering::Relaxed);
        true
    }

    pub fn is_initialized(&self) -> bool {
        EDGE_LPU_INIT.load(Ordering::Acquire)
    }

    pub fn inference_count(&self) -> usize {
        EDGE_INFER_COUNT.load(Ordering::Acquire)
    }
}
