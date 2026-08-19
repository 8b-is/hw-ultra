use super::capacity;
use super::surge;

const LIMIT_PERCENT: u64 = 80;
const SWAP_LIMIT_PERCENT: u64 = 50;
const CPU_LIMIT_PERCENT: u64 = 80;

fn bounded(capacity: u64, current: u64, requested: u64, percent: u64) -> bool {
    if capacity == 0 {
        return true;
    }
    let limit = (capacity * percent / 100).max(1);
    let after = current.saturating_add(requested);
    after <= limit
}

pub fn gate_memory(current: usize, requested: usize) -> bool {
    let real = capacity::read_memory_usage().unwrap_or(current as u64);
    bounded(
        capacity::memory_capacity(),
        real,
        requested as u64,
        LIMIT_PERCENT,
    ) && surge::gate_memory_surge(requested as u64)
}

pub fn gate_swap(current: usize, requested: usize) -> bool {
    let real = capacity::read_swap_usage().unwrap_or(current as u64);
    bounded(
        capacity::swap_capacity(),
        real,
        requested as u64,
        SWAP_LIMIT_PERCENT,
    ) && surge::gate_swap_surge(requested as u64)
}

pub fn gate_dma(current: usize, requested: usize) -> bool {
    bounded(
        capacity::memory_capacity(),
        current as u64,
        requested as u64,
        LIMIT_PERCENT,
    ) && surge::gate_dma_surge(requested as u64)
}

pub fn gate_irq(current: usize, requested: usize) -> bool {
    bounded(
        capacity::irq_capacity(),
        current as u64,
        requested as u64,
        LIMIT_PERCENT,
    ) && surge::gate_irq_surge(requested as u64)
}

pub fn gate_heap(current: usize, requested: usize, cap: usize) -> bool {
    bounded(cap as u64, current as u64, requested as u64, LIMIT_PERCENT)
}

pub fn gate_cpu(current: usize, requested: usize) -> bool {
    let real = capacity::read_cpu_usage().unwrap_or(current as u64);
    bounded(
        capacity::cpu_capacity(),
        real,
        requested as u64,
        CPU_LIMIT_PERCENT,
    ) && surge::gate_cpu_surge(requested as u64)
}
