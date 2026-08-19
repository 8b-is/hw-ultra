use core::sync::atomic::{AtomicUsize, Ordering};

static SYSREG_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_sysreg_base(base: usize) {
    SYSREG_BASE.store(base, Ordering::Release);
}

fn read_sysreg_at(offset: usize) -> u64 {
    let base = SYSREG_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe { core::ptr::read_volatile((base + offset) as *const u64) }
    } else {
        0
    }
}

pub fn read_current_el() -> u8 {
    ((read_sysreg_at(0x00) >> 2) & 0x3) as u8
}

pub fn read_sctlr_el1() -> u64 {
    read_sysreg_at(0x08)
}

pub fn read_tcr_el1() -> u64 {
    read_sysreg_at(0x10)
}

pub fn read_mair_el1() -> u64 {
    read_sysreg_at(0x18)
}

pub fn read_vbar_el1() -> u64 {
    read_sysreg_at(0x20)
}

pub fn write_vbar_el1(val: u64) {
    write_sysreg(0x20, val);
}

pub fn read_cntfrq_el0() -> u64 {
    read_sysreg_at(0x28)
}

pub fn read_cntvct_el0() -> u64 {
    read_sysreg_at(0x30)
}

pub fn write_sysreg(offset: usize, val: u64) {
    let base = SYSREG_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            core::ptr::write_volatile((base + offset) as *mut u64, val);
        }
    }
}
