use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static QC_BASE: AtomicUsize = AtomicUsize::new(0);
static QC_INIT: AtomicBool = AtomicBool::new(false);
static QC_INFER_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Qualcomm {
    pub base: usize,
}

impl Qualcomm {
    pub fn probe(base: usize) -> Option<Self> {
        let id = crate::hardware_access::mmio_read32(base).unwrap_or(0);
        if id == 0 || id == 0xFFFF_FFFF {
            return None;
        }
        QC_BASE.store(base, Ordering::Release);
        Some(Qualcomm { base })
    }

    pub fn init(&self) -> bool {
        crate::hardware_access::mmio_write32(self.base, 0x01);
        let version = crate::hardware_access::mmio_read32(self.base + 0x04).unwrap_or(0);
        if version != 0 {
            QC_INIT.store(true, Ordering::Release);
            true
        } else {
            false
        }
    }

    pub fn submit_workload(&self, desc_addr: usize, desc_count: u32) -> bool {
        if !QC_INIT.load(Ordering::Acquire) {
            return false;
        }
        crate::hardware_access::mmio_write32(self.base + 0x30, desc_addr as u32);
        crate::hardware_access::mmio_write32(self.base + 0x34, desc_count);
        crate::hardware_access::mmio_write32(self.base + 0x38, 1);
        QC_INFER_COUNT.fetch_add(1, Ordering::Relaxed);
        true
    }

    pub fn is_initialized(&self) -> bool {
        QC_INIT.load(Ordering::Acquire)
    }

    pub fn workload_count(&self) -> usize {
        QC_INFER_COUNT.load(Ordering::Acquire)
    }

    pub fn read_version(&self) -> u32 {
        crate::hardware_access::mmio_read32(self.base + 0x04).unwrap_or(0)
    }
}
