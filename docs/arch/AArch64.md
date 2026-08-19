# AArch64 Backend

Architecture-specific implementations for ARM AArch64 processors.

## Submodules

| File | Description |
|------|-------------|
| `sysreg.rs` | System register access (MIDR_EL1, cache ops, barriers) |
| `mmio.rs` | Memory-mapped I/O |
| `init.rs` | Shim registration for aarch64 |
| `register.rs` | General register file |
| `interrupt.rs` | GIC interrupt controller interface |
| `mmu.rs` | Memory management unit |
| `simd.rs` | NEON SIMD detection |
| `syscall.rs` | aarch64 syscall convention (`svc #0`) — native blob in `shim/syscall.rs` |
| `cpu.rs` | CPU-specific utilities |
| `gpu.rs`, `tpu.rs`, `lpu.rs` | Accelerator stubs |
| `audio.rs`, `camera.rs`, `display.rs`, `input.rs` | Device stubs |
| `modem.rs`, `nfc.rs`, `sensor.rs`, `storage.rs`, `usb.rs` | Device stubs |
| `virtualization.rs` | Virtualization extensions |

## System registers

```rust
pub unsafe fn read_midr_el1() -> u64
pub unsafe fn dc_civac(addr: usize)     // Clean & Invalidate by VA to PoC
pub unsafe fn dc_cvau(addr: usize)      // Clean by VA to PoU
pub unsafe fn dsb_ish()                  // Data Synchronization Barrier, Inner Shareable
pub unsafe fn read_sysreg(id: u32) -> u64
pub fn set_midr_mmio(addr: usize)
pub fn set_dc_civac_fn(f: CacheOpFn)
pub fn set_dc_cvau_fn(f: CacheOpFn)
pub fn set_dsb_ish_fn(f: BarrierFn)
```

Cache maintenance and barrier operations use `OnceCopy<fn(...)>` pointers. On real hardware these must be actual aarch64 instructions. The shim stubs are no-ops.

## MMIO

```rust
pub unsafe fn mmio_read32(addr: usize) -> u32
pub unsafe fn mmio_write32(addr: usize, val: u32)
```

Direct `read_volatile`/`write_volatile`. Address must be valid and 4-byte aligned. Unaligned access causes SIGBUS on aarch64.

## CPU identification

AArch64 CPUs are identified through the MIDR_EL1 system register:
- Bits [31:24]: Implementer (0x41 = ARM, 0x51 = Qualcomm, etc.)
- Bits [23:20]: Variant
- Bits [19:16]: Architecture
- Bits [15:4]: Part number
- Bits [3:0]: Revision

## Register base

`REGS_BASE` is 0 by default. Must be configured via `set_regs_base()` before reading SP/LR/PC. Reading from address 0 is undefined behavior.

## Initialization

`init_shim()` registers aarch64-specific implementations:
- CPUID emulation via MIDR parsing
- MSR emulation via system register access
- MMIO read/write function pointers
