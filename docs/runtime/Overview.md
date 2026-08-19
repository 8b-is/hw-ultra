# Runtime Module

The `runtime` module provides the high-level system runtime: the `System` object, HAL (Hardware Abstraction Layer), entry point, platform abstraction, accelerator handles, resource monitoring, and panic handler.

## Submodules

| File | Description |
|------|-------------|
| `system.rs` | `System` struct — full system initialization and management |
| `hal.rs` | Hardware Abstraction Layer — unified API for all hardware |
| `entry.rs` | `hardware_start()` entry point |
| `panic.rs` | Custom panic handler |
| `platform.rs` | `Platform` enum and detection |
| `handle.rs` | `AccelHandle`, `AcceleratorKind` |
| `monitor.rs` | `Component`, `ComponentSnapshot`, `Precision` |
| `resources.rs` | `ReapResult`, `ResourceSnapshot` |
| `worker.rs` | Worker thread management |

## System

```
System {
    arch: Architecture     — detected architecture
    platform: Platform     — detected platform type
}
```

### SystemStatus

```
SystemStatus {
    arch: Architecture
    gpu_present: bool
    tpu_present: bool
    lpu_present: bool
    dma_allocated: usize
    mem_allocated: usize
    irq_registered: usize
    limits_enforced: bool
    total_ram: usize
}
```

### System API

| Method | Description |
|--------|-------------|
| `init()` | Full 17-phase system initialization |
| `is_ready()` | Whether init completed |
| `arch()` | Returns `Architecture` |
| `gpu()` / `tpu()` / `lpu()` | Returns `Option<AccelHandle>` |
| `status()` | Returns `SystemStatus` snapshot |
| `set_dma_limit(n)` | Sets max DMA allocations |
| `set_memory_limit(n)` | Sets max memory allocations |
| `set_irq_limit(n)` | Sets max IRQ registrations |
| `enforce_limits(bool)` | Enables/disables limit enforcement |
| `try_alloc_dma(size)` | Attempts DMA allocation within limits |
| `try_alloc_memory(size)` | Attempts memory allocation within limits |
| `try_register_irq()` | Attempts IRQ registration within limits |
| `set_temp_limit(component, millideg)` | Sets thermal limit |
| `set_freq_bounds(component, min, max)` | Sets frequency bounds |
| `set_precision(component, precision)` | Sets monitoring precision |
| `spawn_workers()` | Starts background worker threads |
| `for_each_accel(f)` | Iterates over all accelerator handles |

## Entry point

`hardware_start() -> !`

The crate's entry point:
1. Creates `System::init()`
2. Configures resource limits
3. Spawns workers
4. Enters infinite loop (event processing)

## HAL

The HAL provides a unified function-level API across all hardware subsystems. See [HAL.md](HAL.md) for details.

## Re-exports

```rust
pub use hal::{...}
pub use handle::{AccelHandle, AcceleratorKind}
pub use monitor::{Component, ComponentSnapshot, Precision}
pub use platform::Platform
pub use resources::{ReapResult, ResourceSnapshot}
pub use system::System
```

`GuardianSnapshot` and `SurgeSnapshot` are exported directly from `arch::guardian` via `hardware::sys::*`.
