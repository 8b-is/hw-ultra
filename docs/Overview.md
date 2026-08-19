# Overview

`hardware` is a `#![no_std]` Rust crate that provides a low-level hardware abstraction layer (HAL) for CPU, memory, I/O, accelerators (GPU/TPU/LPU), firmware, and system management. It targets both x86_64 and AArch64 architectures.

## Purpose

The crate gives bare-metal and hosted environments a unified API to detect, configure, and manage hardware components without relying on `std`, libc, or any external dependency. All system calls are issued through inline assembly, and all architecture-specific behavior is dispatched at runtime through a shim layer — no `#[cfg]` conditional compilation is used anywhere.

## Key characteristics

- **Zero dependencies**: no libc, no allocator, no `std`. Buffers are stack-allocated or mapped via raw `mmap` syscalls.
- **Zero `#[cfg]`**: architecture selection happens at runtime through `OnceCopy` function pointers.
- **`extern "C"` for calling convention only**: used on machine code blobs (syscall, CPUID) to guarantee register placement — no C library dependency, no foreign function linkage.
- **Single public module**: only `pub mod sys` is exported from `lib.rs`. All internal modules are private.
- **Atomic-only state**: all mutable global state uses `AtomicUsize`/`AtomicBool`/`AtomicI64` or `OnceCopy<T>`. No `static mut`.
- **Guardian-gated resources**: CPU, memory, swap, IRQ, DMA, and heap allocations are protected by a dual-check system — capacity ceiling (`bounded()`) and sliding-window surge rate limiter. `guardian_snapshot()` exposes all state for monitoring.

## Module categories

| Category | Modules |
|----------|---------|
| Architecture | `arch` (shim, x86_64, aarch64, guardian) |
| Core hardware | `cpu`, `memory`, `interrupt`, `timer` |
| Firmware | `firmware` (ACPI, UEFI, DeviceTree, SMBIOS) |
| Bus & I/O | `bus` (PCI, PCIe, AMBA, Virtio), `dma`, `iommu` |
| Accelerators | `gpu`, `tpu`, `lpu` |
| Devices | `audio`, `camera`, `display`, `input`, `modem`, `nfc`, `sensor`, `storage`, `usb` |
| System | `power`, `thermal`, `topology`, `security`, `net` |
| Runtime | `runtime`, `init`, `config`, `discovery`, `syscall` |
| Utilities | `common`, `debug`, `hardware_access`, `boot` |

## Supported platforms

| Architecture | Detection | I/O ports | MSR | MMIO | Syscalls |
|--------------|-----------|-----------|-----|------|----------|
| x86_64 | CPUID | Yes | Yes | Yes | `syscall` |
| AArch64 | MIDR_EL1 | N/A | System regs | Yes | `svc #0` |
