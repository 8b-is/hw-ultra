# Porting Guide

## Overview

This guide covers porting the hardware crate to a new platform or architecture.

## Architecture support

The crate currently supports:
- **x86_64** — full support (CPUID, MSR, I/O ports, APIC, PIT, HPET)
- **AArch64** — full support (system registers, MMIO, GIC, ARM Generic Timer)

## Adding a new architecture

### 1. Create architecture module

Create `src/arch/<arch_name>/` with:
- `mod.rs` — submodule declarations
- `cpu.rs` — CPU identification and feature detection
- `mmio.rs` — MMIO access functions
- Platform-specific modules as needed

### 2. Extend the Architecture enum

In `src/arch/architecture.rs`, add the new variant:

```rust
pub enum Architecture {
    X86_64,
    AArch64,
    NewArch,   // Add here
    Unknown,
}
```

### 3. Register shims

In `src/arch/shim.rs`, ensure the new architecture can provide:
- CPUID equivalent (or no-op)
- MSR equivalent (or no-op)
- MMIO read/write
- Architecture-specific identification

### 4. Update init

In `src/init/core.rs`, add architecture-specific initialization paths.

### 5. Update interrupt controller

In `src/interrupt/controller.rs`, add the new architecture's interrupt controller (e.g., PLIC for RISC-V).

## Adding a new device type

### 1. Create device module

Create `src/<device>/` with:
- `mod.rs` — submodule declarations
- `device.rs` — device struct with `Once<>` singleton
- `detection.rs` — discovery logic

### 2. Add to init sequence

Add `init_<device>()` call in `src/init/core.rs`.

### 3. Register in discovery

Add the device type to `DeviceType` enum in `src/discovery/registry.rs`.

### 4. Add feature flag

Add constant to `src/config/feature.rs`.

## Testing on new platforms

The test suite uses inline assembly for stress testing. To port tests:
1. Ensure `RDTSC` / `CNTVCT_EL0` equivalent is available
2. Update `src/init/detect_test.rs` for the new platform
3. Run `cargo test` and verify all 12 tests pass
