# Guardian

The Guardian is the resource gating subsystem that prevents the crate from exhausting system resources. It enforces two layers of protection:

1. **Capacity gating** (`bounded()`) — static ceiling check (80% RAM, 50% swap, 80% CPU)
2. **Surge rate limiting** (`check_surge()`) — sliding-window rate limiter that prevents consumption spikes

Both checks must pass for an allocation to proceed.

## Submodules

| File | Description |
|------|-------------|
| `capacity.rs` | Capacity setters, cycle timing, usage reader callbacks |
| `gate.rs` | Gate functions — dual check (bounded + surge) per resource |
| `reaper.rs` | Idle timeout and resource reclamation |
| `surge.rs` | Sliding-window rate limiter with CAS-based atomics |

## Gate functions

Each resource type has a gate function that performs a **dual check** — capacity ceiling AND surge rate:

```rust
pub fn gate_cpu(current: usize, requested: usize) -> bool
pub fn gate_memory(current: usize, requested: usize) -> bool
pub fn gate_swap(current: usize, requested: usize) -> bool
pub fn gate_dma(current: usize, requested: usize) -> bool
pub fn gate_heap(current: usize, requested: usize, cap: usize) -> bool
pub fn gate_irq(current: usize, requested: usize) -> bool
```

Each returns `true` if the allocation is within limits **and** below the surge budget, `false` otherwise.

## Capacity configuration

```rust
pub fn set_cpu_capacity(max: u64)
pub fn set_memory_capacity(max: u64)
pub fn set_swap_capacity(max: u64)
pub fn set_irq_capacity(max: u64)
```

Default limits:
- RAM: 80% of detected total
- Swap: 50% of detected total
- CPU: 80% of logical cores

All limits use `.max(1)` to prevent integer division from rounding the limit down to 0 (e.g. `1 * 80 / 100 = 0` on a single-core machine). This guarantees at least 1 unit is always permitted.

## Usage readers

By default, gate functions receive the internal allocation counter as `current`. For real hardware usage, inject a reader callback:

```rust
pub fn set_memory_reader(f: fn() -> u64)
pub fn set_swap_reader(f: fn() -> u64)
pub fn set_cpu_reader(f: fn() -> u64)
```

When a reader is registered, `gate_memory`/`gate_swap`/`gate_cpu` use its return value instead of the internal counter. This lets the consumer (e.g. an OS kernel) provide real usage from page tables, swap maps, or scheduler stats.

## Surge rate limiting

Configures a per-resource sliding window — max bytes/count allowed per time window:

```rust
pub fn set_memory_surge(window_ns: u64, budget: u64)
pub fn set_swap_surge(window_ns: u64, budget: u64)
pub fn set_dma_surge(window_ns: u64, budget: u64)
pub fn set_irq_surge(window_ns: u64, budget: u64)
pub fn set_cpu_surge(window_ns: u64, budget: u64)
```

Set `window_ns` to 0 to disable surge limiting (default). The check uses a CAS loop (`compare_exchange_weak`) for lock-free atomic correctness.

## Snapshot (observability)

```rust
pub fn guardian_snapshot() -> GuardianSnapshot
```

Returns a point-in-time capture of all Guardian state:

```rust
pub struct GuardianSnapshot {
    pub timestamp_ns: u64,
    pub mem_capacity: u64,
    pub swap_capacity: u64,
    pub irq_capacity: u64,
    pub cpu_capacity: u64,
    pub mem_usage: u64,     // from reader, or 0
    pub swap_usage: u64,
    pub cpu_usage: u64,
    pub surge: SurgeSnapshot,
}

pub struct SurgeSnapshot {
    pub mem_window_ns: u64, pub mem_budget: u64, pub mem_counter: u64,
    pub swap_window_ns: u64, pub swap_budget: u64, pub swap_counter: u64,
    pub dma_window_ns: u64, pub dma_budget: u64, pub dma_counter: u64,
    pub irq_window_ns: u64, pub irq_budget: u64, pub irq_counter: u64,
    pub cpu_window_ns: u64, pub cpu_budget: u64, pub cpu_counter: u64,
}
```

Call `guardian_snapshot()` at regular intervals to build time-series data for graphing.

## Reaper

The reaper handles idle timeout and resource reclamation:

```rust
pub fn set_idle_timeout_ns(ns: u64)
pub fn idle_timeout_ns() -> u64
pub fn reap_idle() -> ReapResult
pub fn cycle_ms() -> u64             // Guardian cycle interval
```

## Usage pattern

```rust
// Before allocating memory
if !gate_memory(current, requested_bytes) {
    return None; // would exceed 80% limit or surge budget
}
// proceed with mmap

// Before forking
if !gate_cpu(current, 1) {
    return None; // would exceed CPU limit or surge budget
}
// proceed with fork

// Inject real usage from your OS kernel
set_memory_reader(|| read_from_page_tables());
set_cpu_reader(|| scheduler_cpu_usage());

// Configure surge: max 64 MB per 100ms window
set_memory_surge(100_000_000, 64 * 1024 * 1024);

// Snapshot for monitoring
let snap = guardian_snapshot();
```
