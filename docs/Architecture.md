# Architecture

This document describes the high-level architecture of the `hardware` crate.

## Layered design

```
┌─────────────────────────────────┐
│         pub mod sys             │  ← sole public API
├─────────────────────────────────┤
│       runtime / init            │  ← orchestration, HAL
├──────┬──────┬──────┬────────────┤
│ cpu  │memory│ gpu  │ firmware   │  ← subsystem modules
│ dma  │iommu │ tpu  │ interrupt  │
│ bus  │power │ lpu  │ timer      │
│ net  │debug │ ...  │ security   │
├──────┴──────┴──────┴────────────┤
│     hardware_access / common    │  ← guarded I/O, utilities
├─────────────────────────────────┤
│        arch (shim layer)        │  ← runtime arch dispatch
├──────────┬──────────────────────┤
│  x86_64  │      aarch64        │  ← arch-specific impls
└──────────┴──────────────────────┘
```

## Module visibility

All 36 modules in `lib.rs` are declared with `mod` (private). Only `pub mod sys` is exported. This means:

- External code accesses everything through `hardware::sys::*`
- Internal modules can freely restructure without breaking downstream
- The `sys` module re-exports selected types and functions from internal modules

## Runtime architecture dispatch

Instead of `#[cfg(target_arch = "x86_64")]` / `#[cfg(target_arch = "aarch64")]`, the crate uses a shim layer with `OnceCopy<fn(...)>` function pointers:

1. At startup, `init_shims()` detects the architecture
2. It registers architecture-specific callback functions into `OnceCopy` statics
3. All subsequent calls go through the registered functions
4. If a function is not registered, a safe default is returned

This allows the same binary to contain both x86_64 and aarch64 code paths, selected at runtime.

## Initialization sequence

`init::core::init()` orchestrates 17 phases in order:

1. `init_shims` — register architecture-specific function pointers
2. `init_config` — feature detection, capability registration
3. `init_common` — common utilities verification
4. `init_firmware` — probe ACPI, UEFI, DeviceTree, SMBIOS
5. `init_memory` — detect and configure memory subsystem
6. `init_interrupts` — set up interrupt controller (PIC/APIC or GIC)
7. `init_bus` — enumerate PCI, PCIe, AMBA, Virtio devices
8. `init_dma` — initialize DMA engine with 128-entry ring buffer
9. `init_iommu` — probe and configure IOMMU (Intel VT-d or ARM SMMU)
10. `init_cpu` — gather CPU information, topology, features
11. `init_security` — enable SGX detection, speculation mitigations
12. `init_discovery` — populate device registry
13. `init_timers` — register clock sources (TSC, HPET, PIT, ARM generic timer)
14. `init_accelerators` — initialize GPU, TPU, LPU if present
15. `init_topology` — detect system topology (sockets, cores, NUMA)
16. `init_debug` — set up performance counters and tracing
17. `init_power` — configure power governor and thermal zones

## Data flow patterns

### Hardware access path
```
User code → sys::* → hardware_access::api → arch::shim → arch::x86_64 or arch::aarch64
```

### DMA path
```
DmaBuffer::new() → sys_mmap_anon() → phys_addr() → IOMMU mapping → DmaEngine::submit()
```

### Interrupt path
```
Hardware IRQ → Controller::dispatch() → registered fn() handler → Controller::eoi()
```

## Singleton pattern

Global state uses `Once<T>` or `OnceCopy<T>` for thread-safe initialization:
- `DMA_ENGINE: Once<DmaEngine>` — single DMA engine instance
- `IOMMU_ONCE: Once<IommuController>` — single IOMMU controller
- `TPU_DEVICE: Once<TpuDevice>` — single TPU device
- `LPU_DEVICE: Once<LpuDevice>` — single LPU device
- `GLOBAL_CONTROLLER: Once<Controller>` — interrupt controller
- `IDT_ONCE: Once<Idt>` — interrupt descriptor table

## Guardian resource gating

The Guardian (`arch/guardian/`) enforces two layers of resource protection:

1. **Capacity ceiling** (`bounded()`) — static percentage limit per resource
2. **Surge rate limiter** (`check_surge()`) — sliding-window budget to prevent consumption spikes

| Resource | Default limit | Surge |
|----------|--------------|-------|
| RAM | 80% of total | configurable window/budget |
| Swap | 50% of total | configurable window/budget |
| CPU threads | 80% of available | configurable window/budget |
| IRQ vectors | configurable | configurable window/budget |
| DMA buffers | configurable | configurable window/budget |
| Heap allocations | configurable | N/A |

Usage can come from injected reader callbacks (`set_memory_reader()`, `set_swap_reader()`, `set_cpu_reader()`) or internal atomic counters. `guardian_snapshot()` captures all state for time-series monitoring.
