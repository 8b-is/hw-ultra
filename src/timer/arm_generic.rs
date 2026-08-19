use core::sync::atomic::{AtomicUsize, Ordering};

pub struct ArmGenericTimer;

static TIMER_FREQ: AtomicUsize = AtomicUsize::new(0);

impl ArmGenericTimer {
    pub fn frequency() -> u64 {
        let cached = TIMER_FREQ.load(Ordering::Acquire);
        if cached != 0 {
            return cached as u64;
        }
        let freq = crate::arch::aarch64::cpu::system_regs::read_cntfrq_el0();
        TIMER_FREQ.store(freq as usize, Ordering::Release);
        freq
    }

    pub fn counter() -> u64 {
        crate::arch::aarch64::cpu::system_regs::read_cntvct_el0()
    }

    pub fn elapsed_ns(start: u64, end: u64) -> u64 {
        let freq = Self::frequency();
        if freq == 0 {
            return 0;
        }
        let ticks = end.wrapping_sub(start);
        (ticks * 1_000_000_000) / freq
    }

    pub fn set_timer_value(ticks: u64) {
        crate::arch::aarch64::cpu::system_regs::write_sysreg(0x38, ticks);
    }

    pub fn enable_timer() {
        crate::arch::aarch64::cpu::system_regs::write_sysreg(0x40, 1u64);
    }
}
