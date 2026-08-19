use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static ACCEL_BASE: AtomicUsize = AtomicUsize::new(0);
static ACCEL_INIT: AtomicBool = AtomicBool::new(false);
static ACCEL_WORKLOAD_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Accelerator {
    pub base: usize,
}

impl Accelerator {
    pub fn probe(base: usize) -> Option<Self> {
        let id = crate::hardware_access::mmio_read32(base).unwrap_or(0);
        if id == 0 || id == 0xFFFF_FFFF {
            return None;
        }
        ACCEL_BASE.store(base, Ordering::Release);
        Some(Accelerator { base })
    }

    pub fn init(&self) -> bool {
        crate::hardware_access::mmio_write32(self.base, 0x01);
        let status = crate::hardware_access::mmio_read32(self.base + 0x04).unwrap_or(0);
        if status != 0 {
            ACCEL_INIT.store(true, Ordering::Release);
            true
        } else {
            false
        }
    }

    pub fn submit_workload(&self, desc_addr: usize, desc_count: u32) -> bool {
        if !ACCEL_INIT.load(Ordering::Acquire) {
            return false;
        }
        crate::hardware_access::mmio_write32(self.base + 0x20, desc_addr as u32);
        crate::hardware_access::mmio_write32(self.base + 0x24, desc_count);
        crate::hardware_access::mmio_write32(self.base + 0x28, 1);
        ACCEL_WORKLOAD_COUNT.fetch_add(1, Ordering::Relaxed);
        true
    }

    pub fn is_initialized(&self) -> bool {
        ACCEL_INIT.load(Ordering::Acquire)
    }

    pub fn workload_count(&self) -> usize {
        ACCEL_WORKLOAD_COUNT.load(Ordering::Acquire)
    }

    pub fn read_version(&self) -> u32 {
        crate::hardware_access::mmio_read32(self.base + 0x04).unwrap_or(0)
    }
}
