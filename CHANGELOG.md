# Changelog


All notable changes to this project will be documented in this file.

## 0.0.9 — 2026-03-27

### Changed
- **Zero OS-specific values in the crate** — All hardcoded Linux syscall numbers, OS constants, sysfs/devfs paths, DRM ioctl numbers, and driver constants have been removed from library code. The crate now ships with no OS-specific knowledge whatsoever.
- **`sysinfo` syscall moved to `SyscallNrTable`** — `memory/info.rs` no longer hardcodes syscall numbers 99 (x86_64) / 179 (aarch64). Uses `nr_sysinfo()` from the shim like all other syscalls.
- **`mkdirat` syscall added to `SyscallNrTable`** — `mkdir()` fallback in `functions.rs` uses `raw_syscall(nr_mkdirat, ...)` instead of returning `ERR_NOT_IMPLEMENTED`.
- **Sysfs/devfs fallback paths removed** — `read_midr_sysfs()` (aarch64), `read_sysfs_temp()` / `probe_zones()` (thermal), `detect_sockets()` (topology), `read_cpu_freq_sysfs()` (cpu), `read_governor_sysfs()` (power), `scan_video_devices_sysfs()` (PCI), `try_open_drm()` fallback path, `GPU_PROBES` device node list, and `mali_read_gpu_id_native()` ioctl probing — all removed or replaced with injectable callbacks / default returns.
- **DRM/Radeon constants made injectable** — 24 hardcoded DRM ioctl numbers and Radeon driver constants replaced by `DrmConstants` struct with `set_drm_constants()`. All DRM methods return `None`/default if the table is not configured.

### Added
- **`DrmConstants`** struct (25 fields) and **`set_drm_constants()`** in `gpu/drm.rs` — injectable table for DRM ioctl version, GEM close, Radeon info/gem/cs ioctls, info request codes, GEM domains, chunk IDs, and CS flags.
- **`nr_mkdirat()`** and **`nr_sysinfo()`** accessors in `arch/shim/syscall.rs`.
- **`SyscallNrTable`** now has 33 fields (added `mkdirat`, `sysinfo`).
- Test files (`detect_all.rs`, `stress_sequential.rs`) now provide all Linux-specific values via `configure_linux_tables()` — syscall numbers, OS constants, and DRM constants.

### Fixed
- **LICENSE** restored to MIT (was incorrectly changed to a custom restrictive license).

## 0.0.8 — 2026-03-15

### Changed
- **Zero hardcoded MMIO addresses** — All AArch64 device initialization (GPU, TPU, LPU, audio, camera, display, input, modem, NFC, sensor, storage, USB) now queries the Flattened Device Tree via `find_device_by_compatible()` instead of hardcoded base/size/IRQ values.
- **`cpu/api.rs` deduplicated** — Replaced 506 lines of duplicated code (structs, detection, thermal, frequency) with 10 lines of `pub use` re-exports from `info`, `detect`, `cores`, `ram`, `thermal`, `frequency`.
- **Crystal frequency inference** — `estimate_frequency()` no longer falls back to a hardcoded 25 MHz crystal. New `infer_crystal_hz()` derives the crystal clock from the CPU model via CPUID family/model (Intel SDM Table 18-85): 19.2 MHz (Atom Goldmont), 24 MHz (Skylake through Raptor Lake). Returns 0 for unknown models, falling through to brand string parsing or TSC calibration.
- **`runtime/system.rs`** — AArch64 accelerator registration uses DeviceTree lookups instead of hardcoded addresses. GPU fallback uses `probe_bar_size()` for real BAR size instead of hardcoded `0x1_0000`.
- **`init/core.rs`** — Virtual address mapping uses allocator-derived addresses instead of hardcoded `0x4000_0000`. IOMMU exercising uses dynamically created domain parameters instead of hardcoded IOVA addresses.
- **`init/detect_test.rs`** — LPU detection uses DeviceTree lookup instead of hardcoded base address.

### Added
- **`find_device_by_compatible(needle)`** in `firmware/devicetree.rs` — Searches the FDT blob for a device whose `compatible` string contains the given needle. Returns `Option<(reg_base, reg_size, irq)>`. Used by all AArch64 lifecycle modules.
- **`contains_bytes(haystack, needle)`** helper in `firmware/devicetree.rs` — Byte-slice substring search for `compatible` matching.
- **`infer_crystal_hz()`** in `cpu/frequency.rs` — CPUID-based crystal frequency lookup for 15 Intel CPU models (Atom, Skylake, Kaby Lake, Coffee Lake, Ice Lake, Tiger Lake, Alder Lake, Raptor Lake).

## 0.0.7 — 2026-03-13

### Added
- **Surge rate limiter** (`arch/guardian/surge.rs`) — sliding-window rate limiting per resource (memory, swap, DMA, IRQ, CPU). CAS loop (`compare_exchange_weak`) for lock-free atomic correctness. Configure via `set_memory_surge(window_ns, budget)`, etc. Disabled by default (window = 0).
- **Usage reader callbacks** — `set_memory_reader(fn() -> u64)`, `set_swap_reader()`, `set_cpu_reader()` inject real hardware usage into gate checks. When registered, `gate_memory`/`gate_swap`/`gate_cpu` use the reader value instead of the internal allocation counter.
- **`GuardianSnapshot`** — point-in-time capture of all Guardian state (capacities, real usage, surge counters). `guardian_snapshot()` function for observability and time-series graphing.
- **`SurgeSnapshot`** — per-resource surge window/budget/counter snapshot embedded in `GuardianSnapshot`

### Changed
- Gate functions now perform **dual check**: `bounded()` (static capacity ceiling) AND `gate_*_surge()` (sliding-window rate limit). Both must pass.
- `gate_memory`, `gate_swap`, `gate_cpu` use injected usage reader when available, fall back to internal counter otherwise
- `GuardianSnapshot` and `SurgeSnapshot` exported via `hardware::sys::*`

## 0.0.6 — 2026-03-12

### Added
- **Native CPUID blob** — `X86_64_CPUID_BLOB` (27 bytes) in `.text` section executes the real `cpuid` instruction without inline assembly or `#[cfg]`
- **`arch_cached()`** — reads cached architecture without triggering detection, avoids recursion cycle (`detect_arch` → `cpuid_count` → `native_cpuid`)
- Physical vs logical core detection now works natively (Intel CPUID 0x0B, AMD 0x80000008 + 0x8000001E)

### Changed
- `native_cpuid()` no longer returns `(0,0,0,0)` — executes real CPUID via blob when architecture is cached as X86_64
- Documentation updated across 15 `.md` files for native blobs, `arch_cached()`, OS-agnostic wording

## 0.0.5 — 2026-03-11

### Added
- **Native syscall blobs** — `X86_64_SYSCALL_BLOB` (26 bytes) and `AARCH64_SYSCALL_BLOB` (36 bytes) embedded in `.text` section
- `register_native_syscall()` auto-detects architecture and registers the correct blob during `init_shims()`
- `set_raw_syscall_fn()` remains available as optional override
- `iopl` syscall (31st field in `SyscallNrTable`)
- `OsConstants` expanded to 12 fields (`o_creat`, `o_trunc`, `o_nonblock`, `o_excl`, `o_directory`)
- CPU detection via `sched_getaffinity` (128-byte mask, up to 1024 CPUs)
- Guardian `bounded()` uses `.max(1)` to prevent division-by-zero on single-core machines

### Changed
- `RawSyscallFn` changed to `extern "C"` calling convention (ABI guarantee for blob compatibility, not a C dependency)
- All defaults changed from `-38` to `ERR_NOT_IMPLEMENTED` (`-1`)
- Tests no longer need to provide `linux_raw_syscall` or call `set_raw_syscall_fn()`
- Buffered test output with spinlock

## 0.0.4 — 2026-03-10

### Added
- `sys.rs` split into dedicated modules
- All modules made private (`mod` instead of `pub mod`), single public API via `pub mod sys`
- Runtime OS constants (`OsConstants`, `SyscallNrTable`) — no hardcoded platform values

### Changed
- Removed all `#[cfg]`, `extern "C"`, `#[no_mangle]` from library code
- All stubs replaced with shim/MMIO delegation
- Recursive shim calls fixed with `OnceCopy` + CAS guard pattern

## 0.0.3 — 2026-03-09

### Added
- GPU compute pipeline (DRM device access, command submission)
- TPU graph execution abstractions
- Experimental LPU (language processing unit) module
- IOMMU support (Intel VT-d, ARM SMMU)
- Power management (DVFS, C-states, thermal monitoring)

## 0.0.2 — 2026-03-08

### Added
- PCI/PCIe bus enumeration and BAR probing
- DMA engine, buffer allocation, descriptor rings
- Interrupt controller (IDT, APIC, IRQ routing)
- Timer subsystem (HPET, PIT, TSC, ARM generic timer)
- Firmware interfaces (ACPI, UEFI, SMBIOS, DeviceTree)
- Memory subsystem (frame allocator, virtual memory, heap, NUMA)

## 0.0.1 — 2026-03-07

### Added
- Initial release
- Architecture shim layer (`OnceCopy<fn(...)>` function pointers)
- x86_64 and AArch64 backend stubs
- CPUID, MSR, MMIO abstractions
- CPU detection (vendor, model, frequency, features)
- `#![no_std]`, zero dependencies, zero `#[cfg]`
