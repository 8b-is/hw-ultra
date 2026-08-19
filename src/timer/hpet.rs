use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Hpet {
    pub base_addr: usize,
}

static HPET_BASE: AtomicUsize = AtomicUsize::new(0);

impl Hpet {
    pub fn new(base_addr: usize) -> Self {
        HPET_BASE.store(base_addr, Ordering::Release);
        Self { base_addr }
    }

    pub fn read_counter(&self) -> u64 {
        let ptr = (self.base_addr + 0xF0) as *const u64;
        unsafe { core::ptr::read_volatile(ptr) }
    }

    pub fn read_period_fs(&self) -> u32 {
        let ptr = self.base_addr as *const u32;
        let caps = unsafe { core::ptr::read_volatile(ptr) };
        let period_ptr = (self.base_addr + 0x04) as *const u32;
        let period = unsafe { core::ptr::read_volatile(period_ptr) };
        static CAPS_SIG: AtomicUsize = AtomicUsize::new(0);
        CAPS_SIG.store(caps as usize, Ordering::Release);
        period
    }

    pub fn enable(&self) {
        let cfg_ptr = (self.base_addr + 0x10) as *mut u64;
        let val = unsafe { core::ptr::read_volatile(cfg_ptr as *const u64) };
        unsafe { core::ptr::write_volatile(cfg_ptr, val | 1) };
    }
}
