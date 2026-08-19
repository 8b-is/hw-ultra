use core::sync::atomic::{AtomicU64, Ordering};

static CAPACITY_MEMORY: AtomicU64 = AtomicU64::new(0);
static CAPACITY_SWAP: AtomicU64 = AtomicU64::new(0);
static CAPACITY_IRQ: AtomicU64 = AtomicU64::new(256);
static CAPACITY_CPU: AtomicU64 = AtomicU64::new(0);
static CYCLE_MS: AtomicU64 = AtomicU64::new(100);

static MEMORY_READER: AtomicU64 = AtomicU64::new(0);
static SWAP_READER: AtomicU64 = AtomicU64::new(0);
static CPU_READER: AtomicU64 = AtomicU64::new(0);

pub fn set_memory_capacity(bytes: u64) {
    CAPACITY_MEMORY.store(bytes, Ordering::Release);
}

pub fn memory_capacity() -> u64 {
    CAPACITY_MEMORY.load(Ordering::Acquire)
}

pub fn set_swap_capacity(bytes: u64) {
    CAPACITY_SWAP.store(bytes, Ordering::Release);
}

pub fn swap_capacity() -> u64 {
    CAPACITY_SWAP.load(Ordering::Acquire)
}

pub fn set_irq_capacity(count: u64) {
    CAPACITY_IRQ.store(count, Ordering::Release);
}

pub fn irq_capacity() -> u64 {
    CAPACITY_IRQ.load(Ordering::Acquire)
}

pub fn set_cpu_capacity(logical_cores: u64) {
    CAPACITY_CPU.store(logical_cores, Ordering::Release);
}

pub fn cpu_capacity() -> u64 {
    CAPACITY_CPU.load(Ordering::Acquire)
}

pub fn set_cycle_ms(ms: u64) {
    CYCLE_MS.store(ms, Ordering::Release);
}

pub fn cycle_ms() -> u64 {
    CYCLE_MS.load(Ordering::Acquire)
}

pub fn set_memory_reader(f: fn() -> u64) {
    MEMORY_READER.store(f as usize as u64, Ordering::Release);
}

pub fn read_memory_usage() -> Option<u64> {
    let raw = MEMORY_READER.load(Ordering::Acquire);
    if raw == 0 {
        return None;
    }
    let f: fn() -> u64 = unsafe { core::mem::transmute(raw as usize) };
    Some(f())
}

pub fn set_swap_reader(f: fn() -> u64) {
    SWAP_READER.store(f as usize as u64, Ordering::Release);
}

pub fn read_swap_usage() -> Option<u64> {
    let raw = SWAP_READER.load(Ordering::Acquire);
    if raw == 0 {
        return None;
    }
    let f: fn() -> u64 = unsafe { core::mem::transmute(raw as usize) };
    Some(f())
}

pub fn set_cpu_reader(f: fn() -> u64) {
    CPU_READER.store(f as usize as u64, Ordering::Release);
}

pub fn read_cpu_usage() -> Option<u64> {
    let raw = CPU_READER.load(Ordering::Acquire);
    if raw == 0 {
        return None;
    }
    let f: fn() -> u64 = unsafe { core::mem::transmute(raw as usize) };
    Some(f())
}
