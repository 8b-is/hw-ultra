use core::sync::atomic::{AtomicUsize, Ordering};

static PERF_START: AtomicUsize = AtomicUsize::new(0);
static PERF_END: AtomicUsize = AtomicUsize::new(0);
static PERF_SAMPLE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct Perf {
    pub start_ticks: u64,
    pub end_ticks: u64,
}

pub fn read_timestamp() -> u64 {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => crate::arch::x86_64::cpu::tsc::read_tsc(),
        crate::arch::Architecture::AArch64 => {
            crate::arch::aarch64::cpu::system_regs::read_cntvct_el0()
        }
        _ => 0,
    }
}

pub fn start() -> Perf {
    let ts = read_timestamp();
    PERF_START.store(ts as usize, Ordering::Release);
    Perf {
        start_ticks: ts,
        end_ticks: 0,
    }
}

pub fn stop(p: &Perf) -> Perf {
    let ts = read_timestamp();
    PERF_END.store(ts as usize, Ordering::Release);
    PERF_SAMPLE_COUNT.fetch_add(1, Ordering::Relaxed);
    Perf {
        start_ticks: p.start_ticks,
        end_ticks: ts,
    }
}

pub fn elapsed(p: &Perf) -> u64 {
    p.end_ticks.wrapping_sub(p.start_ticks)
}

pub fn sample_count() -> usize {
    PERF_SAMPLE_COUNT.load(Ordering::Acquire)
}

pub fn last_start() -> usize {
    PERF_START.load(Ordering::Acquire)
}

pub fn last_end() -> usize {
    PERF_END.load(Ordering::Acquire)
}
