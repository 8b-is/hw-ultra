use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static VIRT_AVAILABLE: AtomicU8 = AtomicU8::new(0xFF);
static HYP_REG_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_hyp_reg_base(base: usize) {
    HYP_REG_BASE.store(base, Ordering::Release);
}

pub fn is_el2_available() -> bool {
    let cached = VIRT_AVAILABLE.load(Ordering::Acquire);
    if cached != 0xFF {
        return cached != 0;
    }
    let el = crate::arch::aarch64::cpu::system_regs::read_current_el();
    let available = el >= 2;
    VIRT_AVAILABLE.store(available as u8, Ordering::Release);
    available
}

pub fn read_hcr_el2() -> u64 {
    let base = HYP_REG_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe { core::ptr::read_volatile(base as *const u64) }
    } else {
        0
    }
}

pub fn read_vttbr_el2() -> u64 {
    let base = HYP_REG_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe { core::ptr::read_volatile((base + 8) as *const u64) }
    } else {
        0
    }
}

pub fn vmid() -> u16 {
    let vttbr = read_vttbr_el2();
    ((vttbr >> 48) & 0xFFFF) as u16
}
