use core::sync::atomic::{AtomicUsize, Ordering};

static GIC_CPU_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_gic_cpu_base(base: usize) {
    GIC_CPU_BASE.store(base, Ordering::Release);
}

pub fn init() {
    let base = GIC_CPU_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            core::ptr::write_volatile(base as *mut u64, 1u64);
            core::ptr::write_volatile((base + 8) as *mut u64, 0xffu64);
        }
    }
}

pub fn enable_irq(irq: u8) {
    static IRQ_AA_LAST: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    IRQ_AA_LAST.store(irq as usize, core::sync::atomic::Ordering::Release);
}

pub fn disable_irq(irq: u8) {
    static IRQ_AA_LAST2: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    IRQ_AA_LAST2.store(irq as usize, core::sync::atomic::Ordering::Release);
}

pub fn eoi(irq: u8) {
    let base = GIC_CPU_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            core::ptr::write_volatile((base + 16) as *mut u64, irq as u64);
        }
    }
}
