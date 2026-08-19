# System

## Overview

`System` is the top-level object that orchestrates all hardware initialization and provides runtime management.

## Initialization

`System::init() -> Self` runs the full 17-phase init sequence:

1. `init_shims()` — architecture-specific function pointer setup
2. `init_config()` — feature flags and capabilities
3. `init_common()` — common utilities
4. `init_firmware()` — ACPI, UEFI, DeviceTree, SMBIOS
5. `init_memory()` — physical/virtual memory, heap
6. `init_interrupts()` — interrupt controller and IDT
7. `init_bus()` — PCI/PCIe/AMBA discovery
8. `init_dma()` — DMA engine
9. `init_iommu()` — IOMMU setup
10. `init_cpu()` — CPU detection and features
11. `init_security()` — SGX, speculation mitigations
12. `init_discovery()` — device registry
13. `init_timers()` — clock sources
14. `init_accelerators()` — GPU, TPU, LPU
15. `init_topology()` — system topology
16. `init_debug()` — performance counters, tracing
17. `init_power()` — power management, thermal

## Resource limits

`System` tracks resource consumption and can enforce limits:

| Resource | Limit method | Check method |
|----------|-------------|--------------|
| DMA allocations | `set_dma_limit(n)` | `try_alloc_dma(size)` |
| Memory allocations | `set_memory_limit(n)` | `try_alloc_memory(size)` |
| IRQ registrations | `set_irq_limit(n)` | `try_register_irq()` |

Enable enforcement with `enforce_limits(true)`. When enabled, allocation attempts that exceed limits return `false`.

## Accelerator access

```
system.gpu() -> Option<AccelHandle>
system.tpu() -> Option<AccelHandle>
system.lpu() -> Option<AccelHandle>
```

Returns a handle if the accelerator was detected during init. `for_each_accel(f)` iterates all detected accelerators.

## Monitoring

| Method | Description |
|--------|-------------|
| `set_temp_limit(component, millideg)` | Thermal shutdown threshold |
| `set_freq_bounds(component, min, max)` | DVFS bounds |
| `set_precision(component, precision)` | Monitoring granularity |
