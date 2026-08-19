# Getting Started

## Requirements

- Rust nightly toolchain (for `#![no_std]` and inline assembly)
- Linux x86_64 or AArch64 host (for testing)
- No external dependencies needed

## Building

```bash
cargo build
```

The crate compiles as a library with `#![no_std]`. It produces no binary output by default.

## Running tests

```bash
cargo test
```

Tests require a Linux host with:
- At least 2 GB of free RAM (stress tests allocate up to 80%)
- Available swap space (stress tests use up to 50%)
- Optional: `/dev/dri/renderD128` for GPU tests (AMD Radeon)

## Using the crate

Add to your `Cargo.toml`:
```toml
[dependencies]
hardware = { path = "../hardware" }
```

All public API is under `hardware::sys`:
```rust
use hardware::sys;
```

## Initialization

Before using any hardware function, initialize the system:
```rust
let system = hardware::sys::System::init();
```

This runs all 17 initialization phases: shim registration, firmware probing, memory detection, interrupt setup, bus enumeration, DMA engine init, IOMMU setup, CPU detection, security features, device discovery, timer registration, accelerator init, topology detection, debug setup, and power configuration.

## Reading CPU information

```rust
if let Some(info) = hardware::sys::detect_cpu_info() {
    // info.arch, info.vendor, info.frequency_hz, info.cores, etc.
}
```

## Detecting memory

```rust
if let Some(mem) = hardware::sys::detect_memory_info() {
    // mem.total_bytes, mem.free_bytes, mem.swap_total_bytes
}
```

## Checking features

```rust
let has_sse = hardware::sys::has_feature("sse");
```
