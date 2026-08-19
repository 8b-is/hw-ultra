use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Ept {
    pub root_phys: usize,
    pub level: u8,
}

static EPT_BASE: AtomicUsize = AtomicUsize::new(0);

impl Ept {
    pub fn new(root_phys: usize) -> Self {
        EPT_BASE.store(root_phys, Ordering::Release);
        Self {
            root_phys,
            level: 4,
        }
    }

    pub fn map(&self, guest_phys: usize, host_phys: usize, flags: u64) -> bool {
        let entry = host_phys as u64 | (flags & 0xFFF) | 0x7;
        let ptr = (self.root_phys + ((guest_phys >> 12) & 0x1FF) * 8) as *mut u64;
        unsafe { core::ptr::write_volatile(ptr, entry) };
        true
    }

    pub fn base(&self) -> usize {
        EPT_BASE.load(Ordering::Acquire)
    }
}
