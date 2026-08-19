use core::sync::atomic::{AtomicUsize, Ordering};

static REGS_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_regs_base(base: usize) {
    REGS_BASE.store(base, Ordering::Release);
}

pub struct AArch64Regs {
    pub x: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
}

impl AArch64Regs {
    pub const fn zeroed() -> Self {
        Self {
            x: [0u64; 31],
            sp: 0,
            pc: 0,
            pstate: 0,
        }
    }

    pub fn read_sp() -> u64 {
        let base = REGS_BASE.load(Ordering::Acquire);
        if base != 0 {
            unsafe { core::ptr::read_volatile(base as *const u64) }
        } else {
            0
        }
    }

    pub fn read_lr() -> u64 {
        let base = REGS_BASE.load(Ordering::Acquire);
        if base != 0 {
            unsafe { core::ptr::read_volatile((base + 8) as *const u64) }
        } else {
            0
        }
    }
}
