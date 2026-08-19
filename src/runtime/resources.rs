use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

static DMA_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DMA_LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);
static DMA_ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

static MEM_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static MEM_LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);
static MEM_ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

static IRQ_REGISTERED: AtomicUsize = AtomicUsize::new(0);
static IRQ_LIMIT: AtomicUsize = AtomicUsize::new(256);

static SWAP_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static SWAP_ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

static CPU_THREADS: AtomicUsize = AtomicUsize::new(0);

static LAST_ACTIVITY_DMA: AtomicU64 = AtomicU64::new(0);
static LAST_ACTIVITY_MEM: AtomicU64 = AtomicU64::new(0);
static LAST_ACTIVITY_IRQ: AtomicU64 = AtomicU64::new(0);
static LAST_ACTIVITY_SWAP: AtomicU64 = AtomicU64::new(0);
static LAST_ACTIVITY_CPU: AtomicU64 = AtomicU64::new(0);

static LIMITS_ENFORCED: AtomicBool = AtomicBool::new(false);

fn touch(ts: &AtomicU64) {
    ts.store(crate::sys::monotonic_ns(), Ordering::Release);
}

pub fn enforce_limits(enable: bool) {
    LIMITS_ENFORCED.store(enable, Ordering::Release);
}

pub fn limits_active() -> bool {
    LIMITS_ENFORCED.load(Ordering::Acquire)
}

pub fn set_dma_limit(bytes: usize) {
    DMA_LIMIT.store(bytes, Ordering::Release);
}

pub fn dma_limit() -> usize {
    DMA_LIMIT.load(Ordering::Acquire)
}

pub fn try_alloc_dma(bytes: usize) -> bool {
    let current = DMA_ALLOCATED.load(Ordering::Acquire);
    if !crate::arch::guardian::gate_dma(current, bytes) {
        return false;
    }
    if !LIMITS_ENFORCED.load(Ordering::Acquire) {
        DMA_ALLOCATED.fetch_add(bytes, Ordering::AcqRel);
        DMA_ALLOC_COUNT.fetch_add(1, Ordering::AcqRel);
        touch(&LAST_ACTIVITY_DMA);
        return true;
    }
    let limit = DMA_LIMIT.load(Ordering::Acquire);
    if current + bytes > limit {
        return false;
    }
    DMA_ALLOCATED.fetch_add(bytes, Ordering::AcqRel);
    DMA_ALLOC_COUNT.fetch_add(1, Ordering::AcqRel);
    touch(&LAST_ACTIVITY_DMA);
    true
}

pub fn free_dma(bytes: usize) {
    let prev = DMA_ALLOCATED.fetch_sub(bytes, Ordering::AcqRel);
    if prev >= bytes {
        DMA_ALLOC_COUNT.fetch_sub(1, Ordering::AcqRel);
    }
    touch(&LAST_ACTIVITY_DMA);
}

pub fn dma_allocated() -> usize {
    DMA_ALLOCATED.load(Ordering::Acquire)
}

pub fn dma_alloc_count() -> usize {
    DMA_ALLOC_COUNT.load(Ordering::Acquire)
}

pub fn set_memory_limit(bytes: usize) {
    MEM_LIMIT.store(bytes, Ordering::Release);
}

pub fn memory_limit() -> usize {
    MEM_LIMIT.load(Ordering::Acquire)
}

pub fn try_alloc_memory(bytes: usize) -> bool {
    let current = MEM_ALLOCATED.load(Ordering::Acquire);
    if !crate::arch::guardian::gate_memory(current, bytes) {
        return false;
    }
    if !LIMITS_ENFORCED.load(Ordering::Acquire) {
        MEM_ALLOCATED.fetch_add(bytes, Ordering::AcqRel);
        MEM_ALLOC_COUNT.fetch_add(1, Ordering::AcqRel);
        touch(&LAST_ACTIVITY_MEM);
        return true;
    }
    let limit = MEM_LIMIT.load(Ordering::Acquire);
    if current + bytes > limit {
        return false;
    }
    MEM_ALLOCATED.fetch_add(bytes, Ordering::AcqRel);
    MEM_ALLOC_COUNT.fetch_add(1, Ordering::AcqRel);
    touch(&LAST_ACTIVITY_MEM);
    true
}

pub fn free_memory(bytes: usize) {
    let prev = MEM_ALLOCATED.fetch_sub(bytes, Ordering::AcqRel);
    if prev >= bytes {
        MEM_ALLOC_COUNT.fetch_sub(1, Ordering::AcqRel);
    }
    touch(&LAST_ACTIVITY_MEM);
}

pub fn memory_allocated() -> usize {
    MEM_ALLOCATED.load(Ordering::Acquire)
}

pub fn memory_alloc_count() -> usize {
    MEM_ALLOC_COUNT.load(Ordering::Acquire)
}

pub fn set_irq_limit(max: usize) {
    IRQ_LIMIT.store(max, Ordering::Release);
}

pub fn try_register_irq() -> bool {
    let current = IRQ_REGISTERED.load(Ordering::Acquire);
    if !crate::arch::guardian::gate_irq(current, 1) {
        return false;
    }
    if !LIMITS_ENFORCED.load(Ordering::Acquire) {
        IRQ_REGISTERED.fetch_add(1, Ordering::AcqRel);
        touch(&LAST_ACTIVITY_IRQ);
        return true;
    }
    let limit = IRQ_LIMIT.load(Ordering::Acquire);
    if current >= limit {
        return false;
    }
    IRQ_REGISTERED.fetch_add(1, Ordering::AcqRel);
    touch(&LAST_ACTIVITY_IRQ);
    true
}

pub fn unregister_irq() {
    IRQ_REGISTERED.fetch_sub(1, Ordering::AcqRel);
    touch(&LAST_ACTIVITY_IRQ);
}

pub fn irq_registered() -> usize {
    IRQ_REGISTERED.load(Ordering::Acquire)
}

pub fn try_alloc_swap(bytes: usize) -> bool {
    let current = SWAP_ALLOCATED.load(Ordering::Acquire);
    if !crate::arch::guardian::gate_swap(current, bytes) {
        return false;
    }
    SWAP_ALLOCATED.fetch_add(bytes, Ordering::AcqRel);
    SWAP_ALLOC_COUNT.fetch_add(1, Ordering::AcqRel);
    touch(&LAST_ACTIVITY_SWAP);
    true
}

pub fn free_swap(bytes: usize) {
    let prev = SWAP_ALLOCATED.fetch_sub(bytes, Ordering::AcqRel);
    if prev >= bytes {
        SWAP_ALLOC_COUNT.fetch_sub(1, Ordering::AcqRel);
    }
    touch(&LAST_ACTIVITY_SWAP);
}

pub fn swap_allocated() -> usize {
    SWAP_ALLOCATED.load(Ordering::Acquire)
}

pub fn swap_alloc_count() -> usize {
    SWAP_ALLOC_COUNT.load(Ordering::Acquire)
}

pub fn try_alloc_cpu(threads: usize) -> bool {
    let current = CPU_THREADS.load(Ordering::Acquire);
    if !crate::arch::guardian::gate_cpu(current, threads) {
        return false;
    }
    CPU_THREADS.fetch_add(threads, Ordering::AcqRel);
    touch(&LAST_ACTIVITY_CPU);
    true
}

pub fn free_cpu(threads: usize) {
    CPU_THREADS.fetch_sub(threads, Ordering::AcqRel);
    touch(&LAST_ACTIVITY_CPU);
}

pub fn cpu_threads() -> usize {
    CPU_THREADS.load(Ordering::Acquire)
}

pub fn reset_counters() {
    DMA_ALLOCATED.store(0, Ordering::Release);
    DMA_ALLOC_COUNT.store(0, Ordering::Release);
    MEM_ALLOCATED.store(0, Ordering::Release);
    MEM_ALLOC_COUNT.store(0, Ordering::Release);
    IRQ_REGISTERED.store(0, Ordering::Release);
    SWAP_ALLOCATED.store(0, Ordering::Release);
    SWAP_ALLOC_COUNT.store(0, Ordering::Release);
    CPU_THREADS.store(0, Ordering::Release);
    LAST_ACTIVITY_DMA.store(0, Ordering::Release);
    LAST_ACTIVITY_MEM.store(0, Ordering::Release);
    LAST_ACTIVITY_IRQ.store(0, Ordering::Release);
    LAST_ACTIVITY_SWAP.store(0, Ordering::Release);
    LAST_ACTIVITY_CPU.store(0, Ordering::Release);
}

pub fn last_activity_dma() -> u64 {
    LAST_ACTIVITY_DMA.load(Ordering::Acquire)
}

pub fn last_activity_mem() -> u64 {
    LAST_ACTIVITY_MEM.load(Ordering::Acquire)
}

pub fn last_activity_irq() -> u64 {
    LAST_ACTIVITY_IRQ.load(Ordering::Acquire)
}

pub fn last_activity_swap() -> u64 {
    LAST_ACTIVITY_SWAP.load(Ordering::Acquire)
}

pub fn last_activity_cpu() -> u64 {
    LAST_ACTIVITY_CPU.load(Ordering::Acquire)
}

pub fn force_reset_dma() {
    DMA_ALLOCATED.store(0, Ordering::Release);
    DMA_ALLOC_COUNT.store(0, Ordering::Release);
    LAST_ACTIVITY_DMA.store(0, Ordering::Release);
}

pub fn force_reset_mem() {
    MEM_ALLOCATED.store(0, Ordering::Release);
    MEM_ALLOC_COUNT.store(0, Ordering::Release);
    LAST_ACTIVITY_MEM.store(0, Ordering::Release);
}

pub fn force_reset_irq() {
    IRQ_REGISTERED.store(0, Ordering::Release);
    LAST_ACTIVITY_IRQ.store(0, Ordering::Release);
}

pub fn force_reset_swap() {
    SWAP_ALLOCATED.store(0, Ordering::Release);
    SWAP_ALLOC_COUNT.store(0, Ordering::Release);
    LAST_ACTIVITY_SWAP.store(0, Ordering::Release);
}

pub fn force_reset_cpu() {
    CPU_THREADS.store(0, Ordering::Release);
    LAST_ACTIVITY_CPU.store(0, Ordering::Release);
}

pub fn set_memory_capacity(bytes: u64) {
    crate::arch::guardian::set_memory_capacity(bytes);
}

pub fn set_swap_capacity(bytes: u64) {
    crate::arch::guardian::set_swap_capacity(bytes);
}

pub fn set_irq_capacity(count: u64) {
    crate::arch::guardian::set_irq_capacity(count);
}

pub fn set_cpu_capacity(logical_cores: u64) {
    crate::arch::guardian::set_cpu_capacity(logical_cores);
}

pub fn set_cycle_ms(ms: u64) {
    crate::arch::guardian::set_cycle_ms(ms);
}

pub fn cycle_ms() -> u64 {
    crate::arch::guardian::cycle_ms()
}

pub fn set_idle_timeout_ns(ns: u64) {
    crate::arch::guardian::set_idle_timeout_ns(ns);
}

pub fn idle_timeout_ns() -> u64 {
    crate::arch::guardian::idle_timeout_ns()
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
    let r = crate::arch::guardian::reap_idle();
    ReapResult {
        dma_reaped: r.dma_reaped,
        mem_reaped: r.mem_reaped,
        irq_reaped: r.irq_reaped,
        swap_reaped: r.swap_reaped,
        cpu_reaped: r.cpu_reaped,
        workers_killed: r.workers_killed,
    }
}

pub struct ResourceSnapshot {
    pub dma_allocated: usize,
    pub dma_limit: usize,
    pub dma_alloc_count: usize,
    pub mem_allocated: usize,
    pub mem_limit: usize,
    pub mem_alloc_count: usize,
    pub irq_registered: usize,
    pub irq_limit: usize,
    pub limits_enforced: bool,
    pub swap_allocated: usize,
    pub swap_alloc_count: usize,
    pub cpu_threads: usize,
}

pub fn snapshot() -> ResourceSnapshot {
    ResourceSnapshot {
        dma_allocated: DMA_ALLOCATED.load(Ordering::Acquire),
        dma_limit: DMA_LIMIT.load(Ordering::Acquire),
        dma_alloc_count: DMA_ALLOC_COUNT.load(Ordering::Acquire),
        mem_allocated: MEM_ALLOCATED.load(Ordering::Acquire),
        mem_limit: MEM_LIMIT.load(Ordering::Acquire),
        mem_alloc_count: MEM_ALLOC_COUNT.load(Ordering::Acquire),
        irq_registered: IRQ_REGISTERED.load(Ordering::Acquire),
        irq_limit: IRQ_LIMIT.load(Ordering::Acquire),
        limits_enforced: LIMITS_ENFORCED.load(Ordering::Acquire),
        swap_allocated: SWAP_ALLOCATED.load(Ordering::Acquire),
        swap_alloc_count: SWAP_ALLOC_COUNT.load(Ordering::Acquire),
        cpu_threads: CPU_THREADS.load(Ordering::Acquire),
    }
}
