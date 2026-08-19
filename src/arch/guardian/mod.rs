mod capacity;
mod gate;
pub mod reaper;
mod surge;

pub use capacity::{
    cycle_ms, set_cpu_capacity, set_cpu_reader, set_cycle_ms, set_irq_capacity,
    set_memory_capacity, set_memory_reader, set_swap_capacity, set_swap_reader,
};
pub use gate::{gate_cpu, gate_dma, gate_heap, gate_irq, gate_memory, gate_swap};
pub use reaper::{idle_timeout_ns, reap_idle, set_idle_timeout_ns};
pub use surge::{
    gate_cpu_surge, gate_dma_surge, gate_irq_surge, gate_memory_surge, gate_swap_surge,
    set_cpu_surge, set_dma_surge, set_irq_surge, set_memory_surge, set_swap_surge, SurgeSnapshot,
};

#[derive(Clone, Copy)]
pub struct GuardianSnapshot {
    pub timestamp_ns: u64,
    pub mem_capacity: u64,
    pub swap_capacity: u64,
    pub irq_capacity: u64,
    pub cpu_capacity: u64,
    pub mem_usage: u64,
    pub swap_usage: u64,
    pub cpu_usage: u64,
    pub surge: SurgeSnapshot,
}

pub fn guardian_snapshot() -> GuardianSnapshot {
    GuardianSnapshot {
        timestamp_ns: crate::sys::monotonic_ns(),
        mem_capacity: capacity::memory_capacity(),
        swap_capacity: capacity::swap_capacity(),
        irq_capacity: capacity::irq_capacity(),
        cpu_capacity: capacity::cpu_capacity(),
        mem_usage: capacity::read_memory_usage().unwrap_or(0),
        swap_usage: capacity::read_swap_usage().unwrap_or(0),
        cpu_usage: capacity::read_cpu_usage().unwrap_or(0),
        surge: surge::surge_snapshot(),
    }
}
