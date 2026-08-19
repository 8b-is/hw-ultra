use core::sync::atomic::{AtomicU64, Ordering};

static SURGE_MEM_WINDOW_NS: AtomicU64 = AtomicU64::new(0);
static SURGE_MEM_BUDGET: AtomicU64 = AtomicU64::new(0);
static SURGE_MEM_COUNTER: AtomicU64 = AtomicU64::new(0);
static SURGE_MEM_WINDOW_START: AtomicU64 = AtomicU64::new(0);
static SURGE_SWAP_WINDOW_NS: AtomicU64 = AtomicU64::new(0);
static SURGE_SWAP_BUDGET: AtomicU64 = AtomicU64::new(0);
static SURGE_SWAP_COUNTER: AtomicU64 = AtomicU64::new(0);
static SURGE_SWAP_WINDOW_START: AtomicU64 = AtomicU64::new(0);
static SURGE_DMA_WINDOW_NS: AtomicU64 = AtomicU64::new(0);
static SURGE_DMA_BUDGET: AtomicU64 = AtomicU64::new(0);
static SURGE_DMA_COUNTER: AtomicU64 = AtomicU64::new(0);
static SURGE_DMA_WINDOW_START: AtomicU64 = AtomicU64::new(0);
static SURGE_IRQ_WINDOW_NS: AtomicU64 = AtomicU64::new(0);
static SURGE_IRQ_BUDGET: AtomicU64 = AtomicU64::new(0);
static SURGE_IRQ_COUNTER: AtomicU64 = AtomicU64::new(0);
static SURGE_IRQ_WINDOW_START: AtomicU64 = AtomicU64::new(0);
static SURGE_CPU_WINDOW_NS: AtomicU64 = AtomicU64::new(0);
static SURGE_CPU_BUDGET: AtomicU64 = AtomicU64::new(0);
static SURGE_CPU_COUNTER: AtomicU64 = AtomicU64::new(0);
static SURGE_CPU_WINDOW_START: AtomicU64 = AtomicU64::new(0);
pub fn set_memory_surge(window_ns: u64, budget: u64) {
    SURGE_MEM_WINDOW_NS.store(window_ns, Ordering::Release);
    SURGE_MEM_BUDGET.store(budget, Ordering::Release);
    SURGE_MEM_COUNTER.store(0, Ordering::Release);
    SURGE_MEM_WINDOW_START.store(0, Ordering::Release);
}

pub fn set_swap_surge(window_ns: u64, budget: u64) {
    SURGE_SWAP_WINDOW_NS.store(window_ns, Ordering::Release);
    SURGE_SWAP_BUDGET.store(budget, Ordering::Release);
    SURGE_SWAP_COUNTER.store(0, Ordering::Release);
    SURGE_SWAP_WINDOW_START.store(0, Ordering::Release);
}

pub fn set_dma_surge(window_ns: u64, budget: u64) {
    SURGE_DMA_WINDOW_NS.store(window_ns, Ordering::Release);
    SURGE_DMA_BUDGET.store(budget, Ordering::Release);
    SURGE_DMA_COUNTER.store(0, Ordering::Release);
    SURGE_DMA_WINDOW_START.store(0, Ordering::Release);
}

pub fn set_irq_surge(window_ns: u64, budget: u64) {
    SURGE_IRQ_WINDOW_NS.store(window_ns, Ordering::Release);
    SURGE_IRQ_BUDGET.store(budget, Ordering::Release);
    SURGE_IRQ_COUNTER.store(0, Ordering::Release);
    SURGE_IRQ_WINDOW_START.store(0, Ordering::Release);
}

pub fn set_cpu_surge(window_ns: u64, budget: u64) {
    SURGE_CPU_WINDOW_NS.store(window_ns, Ordering::Release);
    SURGE_CPU_BUDGET.store(budget, Ordering::Release);
    SURGE_CPU_COUNTER.store(0, Ordering::Release);
    SURGE_CPU_WINDOW_START.store(0, Ordering::Release);
}

fn check_surge(
    window_ns: &AtomicU64,
    budget: &AtomicU64,
    counter: &AtomicU64,
    window_start: &AtomicU64,
    amount: u64,
) -> bool {
    let win = window_ns.load(Ordering::Acquire);
    if win == 0 {
        return true;
    }
    let cap = budget.load(Ordering::Acquire);
    if cap == 0 {
        return true;
    }

    let now = crate::sys::monotonic_ns();
    let start = window_start.load(Ordering::Acquire);

    if now.wrapping_sub(start) >= win
        && window_start
            .compare_exchange(start, now, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
    {
        counter.store(0, Ordering::Release);
    }

    loop {
        let current = counter.load(Ordering::Acquire);
        let next = current.saturating_add(amount);
        if next > cap {
            return false;
        }
        if counter
            .compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            return true;
        }
    }
}

pub fn gate_memory_surge(requested: u64) -> bool {
    check_surge(
        &SURGE_MEM_WINDOW_NS,
        &SURGE_MEM_BUDGET,
        &SURGE_MEM_COUNTER,
        &SURGE_MEM_WINDOW_START,
        requested,
    )
}

pub fn gate_swap_surge(requested: u64) -> bool {
    check_surge(
        &SURGE_SWAP_WINDOW_NS,
        &SURGE_SWAP_BUDGET,
        &SURGE_SWAP_COUNTER,
        &SURGE_SWAP_WINDOW_START,
        requested,
    )
}

pub fn gate_dma_surge(requested: u64) -> bool {
    check_surge(
        &SURGE_DMA_WINDOW_NS,
        &SURGE_DMA_BUDGET,
        &SURGE_DMA_COUNTER,
        &SURGE_DMA_WINDOW_START,
        requested,
    )
}

pub fn gate_irq_surge(requested: u64) -> bool {
    check_surge(
        &SURGE_IRQ_WINDOW_NS,
        &SURGE_IRQ_BUDGET,
        &SURGE_IRQ_COUNTER,
        &SURGE_IRQ_WINDOW_START,
        requested,
    )
}

pub fn gate_cpu_surge(requested: u64) -> bool {
    check_surge(
        &SURGE_CPU_WINDOW_NS,
        &SURGE_CPU_BUDGET,
        &SURGE_CPU_COUNTER,
        &SURGE_CPU_WINDOW_START,
        requested,
    )
}

#[derive(Clone, Copy)]
pub struct SurgeSnapshot {
    pub mem_window_ns: u64,
    pub mem_budget: u64,
    pub mem_counter: u64,
    pub swap_window_ns: u64,
    pub swap_budget: u64,
    pub swap_counter: u64,
    pub dma_window_ns: u64,
    pub dma_budget: u64,
    pub dma_counter: u64,
    pub irq_window_ns: u64,
    pub irq_budget: u64,
    pub irq_counter: u64,
    pub cpu_window_ns: u64,
    pub cpu_budget: u64,
    pub cpu_counter: u64,
}

pub fn surge_snapshot() -> SurgeSnapshot {
    SurgeSnapshot {
        mem_window_ns: SURGE_MEM_WINDOW_NS.load(Ordering::Acquire),
        mem_budget: SURGE_MEM_BUDGET.load(Ordering::Acquire),
        mem_counter: SURGE_MEM_COUNTER.load(Ordering::Acquire),
        swap_window_ns: SURGE_SWAP_WINDOW_NS.load(Ordering::Acquire),
        swap_budget: SURGE_SWAP_BUDGET.load(Ordering::Acquire),
        swap_counter: SURGE_SWAP_COUNTER.load(Ordering::Acquire),
        dma_window_ns: SURGE_DMA_WINDOW_NS.load(Ordering::Acquire),
        dma_budget: SURGE_DMA_BUDGET.load(Ordering::Acquire),
        dma_counter: SURGE_DMA_COUNTER.load(Ordering::Acquire),
        irq_window_ns: SURGE_IRQ_WINDOW_NS.load(Ordering::Acquire),
        irq_budget: SURGE_IRQ_BUDGET.load(Ordering::Acquire),
        irq_counter: SURGE_IRQ_COUNTER.load(Ordering::Acquire),
        cpu_window_ns: SURGE_CPU_WINDOW_NS.load(Ordering::Acquire),
        cpu_budget: SURGE_CPU_BUDGET.load(Ordering::Acquire),
        cpu_counter: SURGE_CPU_COUNTER.load(Ordering::Acquire),
    }
}
