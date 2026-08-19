use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static APPLE_LPU_BASE: AtomicUsize = AtomicUsize::new(0);
static APPLE_LPU_INIT: AtomicBool = AtomicBool::new(false);
static APPLE_LPU_INFER_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct AppleLpu {
    pub base: usize,
}

impl AppleLpu {
    pub fn probe(base: usize) -> Option<Self> {
        let id = crate::hardware_access::mmio_read32(base).unwrap_or(0);
        if id == 0 || id == 0xFFFF_FFFF {
            return None;
        }
        APPLE_LPU_BASE.store(base, Ordering::Release);
        Some(AppleLpu { base })
    }

    pub fn init(&self) -> bool {
        crate::hardware_access::mmio_write32(self.base, 1);
        let status = crate::hardware_access::mmio_read32(self.base + 0x04).unwrap_or(0);
        if status & 1 != 0 {
            APPLE_LPU_INIT.store(true, Ordering::Release);
            true
        } else {
            false
        }
    }

    pub fn submit_tokens(&self, token_buf: usize, count: u32) -> bool {
        if !APPLE_LPU_INIT.load(Ordering::Acquire) {
            return false;
        }
        crate::hardware_access::mmio_write32(self.base + 0x10, token_buf as u32);
        crate::hardware_access::mmio_write32(self.base + 0x14, count);
        crate::hardware_access::mmio_write32(self.base + 0x18, 1);
        APPLE_LPU_INFER_COUNT.fetch_add(1, Ordering::Relaxed);
        true
    }

    pub fn is_initialized(&self) -> bool {
        APPLE_LPU_INIT.load(Ordering::Acquire)
    }

    pub fn inference_count(&self) -> usize {
        APPLE_LPU_INFER_COUNT.load(Ordering::Acquire)
    }
}
