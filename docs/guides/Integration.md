# Integration Guide

## Adding the crate to your project

Add to `Cargo.toml`:

```toml
[dependencies]
hardware = { path = "../hardware" }
```

The crate is `#![no_std]` — no standard library is needed.

## Basic usage

```rust
use hardware::sys;

// Initialize all hardware
let system = sys::runtime::System::init();

// Check what was detected
let status = system.status();
// status.arch, status.gpu_present, status.total_ram, etc.
```

## Accessing individual subsystems

All access goes through `hardware::sys::*`:

```rust
// CPU info
let cpu = sys::cpu::detect();

// Memory info
let mem = sys::memory::info::detect_memory_info();

// Firmware
if sys::firmware::acpi::is_present() {
    let rev = sys::firmware::acpi::acpi_revision();
}

// Timers
let ticks = sys::timer::clocksource::read_ticks();
```

## Custom shim registration

The crate auto-detects the architecture and auto-registers native CPUID and syscall handlers via machine code blobs. For custom environments, you can override shims before or after init:

```rust
use hardware::sys::arch::shim;

shim::set_cpuid(my_cpuid_fn);
shim::set_mmio_read32(my_mmio_read_fn);
shim::set_mmio_write32(my_mmio_write_fn);
```

## Resource limits

```rust
let system = sys::runtime::System::init();
system.set_dma_limit(1024 * 1024);      // 1 MB DMA
system.set_memory_limit(64 * 1024 * 1024); // 64 MB heap
system.set_irq_limit(32);                 // 32 IRQs
system.enforce_limits(true);
```

## Feature detection

```rust
use hardware::sys::config::feature;

if feature::is_enabled(feature::FEATURE_GPU) {
    // GPU was detected during init
}
```
