# Sys Module

`pub mod sys` is the sole public module exported from `lib.rs`. It serves as the API gateway to all internal modules.

## Purpose

External code only sees `hardware::sys::*`. Internal modules are all private (`mod`). The `sys` module re-exports selected types, functions, and constants from internal modules.

## What is exported

The `sys` module provides access to:

- **CPU**: `CpuInfo`, `CoreInfo`, `ComponentStatus`, `detect_cpu_info()`, `has_feature()`, `estimate_frequency()`, `detect_cores()`, `set_affinity()`, `get_affinity()`, `pin_to_core()`, `read_core_temperatures()`
- **Memory**: `MemoryInfo`, `detect_memory_info()`, `Frame`, `alloc_frame()`, `free_frame()`, `total_usable_ram()`
- **Architecture**: `Architecture`, `detect_arch()`, `arch_cached()`, `init_shims()`
- **Firmware**: ACPI, UEFI, DeviceTree, SMBIOS types and functions
- **Bus**: PCI device types and scanning functions
- **DMA**: `DmaBuffer`, `DmaEngine`, `Descriptor`
- **IOMMU**: `IommuController`, `Domain`
- **Interrupts**: `Controller`, `Idt`, `Vector`, `register()`, `unregister()`, `dispatch()`
- **GPU**: `GpuDevice`, `RawGpuId`, `set_detect_gpu_fn()`, `parse_mali_gpu_id()`, `mali_product_name()`, `parse_adreno_chip_id()`, `adreno_product_name()`, `Command`, `Shader`, `Queue`
- **TPU**: `TpuDevice`, `Tensor`, `Graph`, `CompiledGraph`
- **LPU**: `LpuDevice`
- **Power**: `reboot()`, `shutdown()`, governor control
- **Thermal**: zone management
- **Timer**: `ClockSource`, `Hpet`, `Pit`
- **Security**: `Enclave`, `IsolationDomain`, speculation mitigations
- **Syscall**: `SyscallResult`, syscall registration
- **Runtime**: `System`, `SystemStatus`, `Platform`, `AccelHandle`
- **Config**: feature flags, capabilities, target platform
- **Common**: `Atomic`, `BitField`, `Guard`, `OnceCopy`, `Once`, barriers, alignment, endianness

## Usage

```rust
use hardware::sys;

let system = sys::System::init();
let arch = sys::detect_arch();
```

All test code uses `hardware::sys::*` paths.

## Constants and runtime functions

The `sys` module re-exports both universal constants and platform-varying runtime functions from `sys/consts.rs`:

**Constants** (identical on all platforms):
- `O_RDONLY`, `O_WRONLY`, `O_RDWR`, `AF_UNIX`, `SOCK_STREAM`, `F_SETFL`

**Runtime functions** (resolved via shim, values differ across platforms):
- `o_creat() -> i32`, `o_trunc() -> i32`, `o_nonblock() -> i32`, `o_excl() -> i32`, `o_directory() -> i32`

Also re-exports `SyscallNrTable` (33 fields), `OsConstants` (12 fields), `DrmConstants` (25 fields), `set_syscall_nrs()`, `set_os_constants()`, `set_drm_constants()`.
