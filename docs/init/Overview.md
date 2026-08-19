# Init Module

The `init` module orchestrates system initialization by calling each subsystem's init function in dependency order.

## Submodules

| File | Description |
|------|-------------|
| `core.rs` | `init()` and `init_shims()` — main init sequence |

## init()

The main initialization function runs 17 phases in order:

| # | Phase | Function | Dependencies |
|---|-------|----------|-------------|
| 1 | Shims | `init_shims()` | None |
| 2 | Config | `init_config()` | Shims |
| 3 | Common | `init_common()` | Shims |
| 4 | Firmware | `init_firmware()` | Shims, MMIO |
| 5 | Memory | `init_memory()` | Firmware (for memory map) |
| 6 | Interrupts | `init_interrupts()` | Memory |
| 7 | Bus | `init_bus()` | Memory, I/O |
| 8 | DMA | `init_dma()` | Memory |
| 9 | IOMMU | `init_iommu()` | DMA, Firmware |
| 10 | CPU | `init_cpu()` | Shims, CPUID |
| 11 | Security | `init_security()` | CPU |
| 12 | Discovery | `init_discovery()` | Bus |
| 13 | Timers | `init_timers()` | Firmware (HPET), I/O (PIT) |
| 14 | Accelerators | `init_accelerators()` | Bus, DMA, IOMMU |
| 15 | Topology | `init_topology()` | CPU |
| 16 | Debug | `init_debug()` | Timers |
| 17 | Power | `init_power()` | CPU, Firmware |

## init_shims()

Registers architecture-specific function pointers:
- CPUID shim (x86_64: native machine code blob for real `cpuid` instruction)
- MSR read shim
- MMIO read/write shims
- AArch64 MIDR shim
- Exit, Mkdir, ScanDir shims
- Raw syscall handler (auto-detected via `X86_64_SYSCALL_BLOB` or `AARCH64_SYSCALL_BLOB`)

## Re-exports

```rust
pub use core::{init, init_shims};
```

## Safety

Init must run exactly once before any other hardware access. The `System::init()` in the runtime module calls `init::init()` as its first action.
