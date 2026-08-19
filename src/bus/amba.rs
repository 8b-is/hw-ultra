use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Amba {
    pub base_addr: usize,
    pub periph_id: u32,
}

static AMBA_BASE: AtomicUsize = AtomicUsize::new(0);

impl Amba {
    pub fn new(base_addr: usize) -> Self {
        AMBA_BASE.store(base_addr, Ordering::Release);
        let pid = Self::read_periph_id(base_addr);
        Self {
            base_addr,
            periph_id: pid,
        }
    }

    fn read_periph_id(base: usize) -> u32 {
        let p0 = unsafe { core::ptr::read_volatile((base + 0xFE0) as *const u32) } & 0xFF;
        let p1 = unsafe { core::ptr::read_volatile((base + 0xFE4) as *const u32) } & 0xFF;
        let p2 = unsafe { core::ptr::read_volatile((base + 0xFE8) as *const u32) } & 0xFF;
        let p3 = unsafe { core::ptr::read_volatile((base + 0xFEC) as *const u32) } & 0xFF;
        (p3 << 24) | (p2 << 16) | (p1 << 8) | p0
    }

    pub fn read_reg(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.base_addr + offset) as *const u32) }
    }

    pub fn write_reg(&self, offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.base_addr + offset) as *mut u32, val) }
    }
}
