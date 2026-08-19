use core::sync::atomic::{AtomicU64, Ordering};

use crate::runtime::resources;

static IDLE_TIMEOUT_NS: AtomicU64 = AtomicU64::new(30_000_000_000);

pub fn set_idle_timeout_ns(ns: u64) {
    IDLE_TIMEOUT_NS.store(ns, Ordering::Release);
}

pub fn idle_timeout_ns() -> u64 {
    IDLE_TIMEOUT_NS.load(Ordering::Acquire)
}

pub struct ReapResult {
    pub dma_reaped: bool,
    pub mem_reaped: bool,
    pub irq_reaped: bool,
    pub swap_reaped: bool,
    pub cpu_reaped: bool,
    pub workers_killed: usize,
}

pub fn reap_idle() -> ReapResult {
    let timeout = IDLE_TIMEOUT_NS.load(Ordering::Acquire);
    let now = crate::sys::monotonic_ns();

    let mut result = ReapResult {
        dma_reaped: false,
        mem_reaped: false,
        irq_reaped: false,
        swap_reaped: false,
        cpu_reaped: false,
        workers_killed: 0,
    };

    if timeout == 0 {
        return result;
    }

    if resources::dma_allocated() > 0 && is_stale(resources::last_activity_dma(), now, timeout) {
        resources::force_reset_dma();
        result.dma_reaped = true;
    }

    if resources::memory_allocated() > 0 && is_stale(resources::last_activity_mem(), now, timeout) {
        resources::force_reset_mem();
        result.mem_reaped = true;
    }

    if resources::irq_registered() > 0 && is_stale(resources::last_activity_irq(), now, timeout) {
        resources::force_reset_irq();
        result.irq_reaped = true;
    }

    if resources::swap_allocated() > 0 && is_stale(resources::last_activity_swap(), now, timeout) {
        resources::force_reset_swap();
        result.swap_reaped = true;
    }

    if resources::cpu_threads() > 0 && is_stale(resources::last_activity_cpu(), now, timeout) {
        let workers = crate::runtime::worker::worker_count();
        crate::runtime::worker::stop_workers();
        resources::force_reset_cpu();
        result.cpu_reaped = true;
        result.workers_killed = workers;
    }

    result
}

fn is_stale(last: u64, now: u64, timeout: u64) -> bool {
    if last == 0 {
        return false;
    }
    now.saturating_sub(last) >= timeout
}
