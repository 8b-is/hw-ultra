# Warnings — Read Before Modifying This Crate

This document is for anyone modifying the `hardware` crate internals. The crate interacts directly with hardware registers, I/O ports, DMA engines, GPU command buffers, and raw syscalls. Mistakes can freeze your machine, corrupt memory, or cause kernel panics.

## 1. Infinite recursion in shims

The shim system (`arch/shim/`) uses `OnceCopy<fn(...)>` with a `default_xxx()` fallback that calls `init_shims()`. If `init_shims()` in turn calls a function that falls back into an uninitialized shim, the result is **infinite recursion** leading to an immediate stack overflow.

**Rule**: `init_shims()` must never call `detect_arch()`, `cpuid_count()`, `read_msr()`, or any function that goes through a shim. Initialization must use `compare_exchange` as a reentrancy guard, not a simple `load`/`store`.

**Rule**: The `native_xxx()` functions in `arch/x86_64/` and `arch/aarch64/` must never call `detect_arch()` — use `arch_cached()` instead, which reads the cached value without triggering detection. This breaks the recursion cycle: `detect_arch()` → `cpuid_count()` → `native_cpuid()` → `arch_cached()` (returns `None` on first probe → `(0,0,0,0)`, then real CPUID once cached).

## 2. Port I/O without privilege → SIGSEGV

x86 `in`/`out` instructions require I/O privilege level (iopl). Without `request_hw_privilege()` (which calls `iopl(3)` via the shim’s `nr_iopl()` syscall number), any port read or write causes a **SIGSEGV**.

**Current protection**: `HW_PRIVILEGE` (`AtomicBool`) is checked before every operation. The `inb`/`outb`/`inl`/`outl` functions in `arch/x86_64/io.rs` use `OnceCopy<fn(...)>` — without a registered handler, they return `0xFF`/`0xFFFFFFFF` and silently ignore writes.

**Never**: Remove the privilege check. Never call `core::arch::asm!("in"...)` directly without going through the shim.

## 3. Memory allocation without Guardian → OOM kill

The Guardian (`arch/guardian/`) enforces two layers of protection: (1) capacity ceiling — 80% RAM, 50% swap, 80% CPU, and (2) surge rate limiting — a sliding-window budget per resource that prevents consumption spikes. Gate functions perform both checks atomically via CAS loops.

If you bypass or disable `try_alloc_memory()` / `try_alloc_swap()`, the system will run out of resources.

**Never**: Call `sys_mmap_anon()` in a loop without checking `try_alloc_memory()` first. Stress tests deliberately attempt 100% to verify that the Guardian blocks.

**Never**: Raise thresholds above 80% RAM / 50% swap. These values are calibrated to prevent system freezes.

**Rule**: If you inject a usage reader via `set_memory_reader()` / `set_swap_reader()` / `set_cpu_reader()`, ensure the callback is fast and lock-free — it is called on every gate check.

## 4. fork() without waitpid() → zombie processes

`sys::fork()` creates a real Linux process. If the parent does not call `waitpid()` for each child, zombie processes accumulate. After a few thousand, the system refuses to create new processes.

**Rule**: Every `fork()` must have a matching `waitpid()`. `free_cpu(1)` must be called after each `waitpid()` to release the Guardian counter.

## 5. DMA and IOMMU — memory corruption

DMA buffers (`dma/buffer.rs`) use `sys_mmap_anon()` and expose their physical address via `phys_addr()`. If a hardware driver writes to a wrong physical address, the result is **silent memory corruption**.

**Rule**: Always verify that the IOMMU is configured when available (`iommu/mapping.rs`). The IOMMU translates addresses and prevents devices from writing to arbitrary locations.

**Never**: Use `phys_addr()` as a DMA address without going through `submit_buffer()`, which handles IOMMU mapping.

## 6. GPU — possible system freeze

GPU commands (`DRM_IOCTL_RADEON_CS`) are executed directly by the hardware. A malformed command buffer can freeze the GPU, which on some drivers also freezes the display.

**Current protection**: `probe_cs_packet_size()` tests buffer sizes before mass submission. NOP commands are harmless.

**Never**: Submit arbitrary GPU opcodes without validating them first. Never write directly to mapped VRAM without going through GEM interfaces.

**Never**: Open `/dev/dri/card0` instead of `/dev/dri/renderD128` — `card0` controls the display, `renderD128` is compute-only.

## 7. MSR — unpredictable results

Model-Specific Registers (`read_msr`/`write_msr`) are specific to each CPU model. Reading a nonexistent MSR causes a `#GP` (General Protection fault) → SIGSEGV.

**Current protection**: `read_msr()` goes through the shim which checks the rate-limiting guard (max 1024 calls). The fallback returns `None`.

**Never**: Write to an MSR without knowing exactly what it does. Some MSRs control CPU voltage, frequency, or memory protection mode.

## 8. ACPI — unverified physical addresses

ACPI code (`firmware/acpi.rs`) scans physical memory (0xE0000–0x100000) to find the RSDP. On a standard Linux userspace system, these addresses are not mapped.

**Current protection**: ACPI functions use `mmio_read32()` via the shim, which returns `None` if the address is not accessible.

**Never**: Dereference a raw pointer to an ACPI physical address without verifying that the page is mapped. Using `read_volatile` on an unmapped address = SIGSEGV.

## 9. Interrupts — handler that panics = double fault

The interrupt system (`interrupt/handler.rs`) stores `fn()` in a 256-entry array. If a handler panics, it causes a double fault on bare metal.

**Rule**: Interrupt handlers must never panic, allocate memory, or make blocking calls. They should only increment atomic counters or write to registers.

## 10. OnceCopy — set() is called exactly once

`OnceCopy<T>` uses a CAS (Compare-And-Swap) with 3 states: `EMPTY(0) → WRITING(1) → READY(2)`. `set()` returns `false` if the value was already written. If you ignore this return value and proceed as if your value was accepted, you will use the wrong implementation.

**Rule**: If `set()` returns `false`, another thread/init won the race. Your implementation is not active — do not use it as if it were.

## 11. Syscall numbers — wrong number = undefined behavior

Syscall numbers differ between x86_64 and aarch64. If `set_syscall_nrs()` is called with wrong numbers, every syscall will do something unexpected. For example, `write` (nr=1 on x86_64) maps to `exit` (nr=1 on aarch64 without translation).

**Rule**: Always use the complete `SyscallNrTable` with all 33 fields. Never hardcode a syscall number.

## 12. raw_syscall — native blobs handle this automatically

The crate auto-registers a native syscall handler via machine code blobs (`X86_64_SYSCALL_BLOB`, `AARCH64_SYSCALL_BLOB`) during `init_shims()`. The blobs use `extern "C"` calling convention to receive arguments, shuffle registers to the architecture's syscall ABI, and execute `syscall` (x86_64) or `svc #0` (aarch64).

`set_raw_syscall_fn()` remains available as an optional override — if called, it replaces the native blob. If overriding, the handler must use `extern "C"` calling convention and the target architecture's syscall ABI. On x86_64: `rax`=nr, `rdi`/`rsi`/`rdx`/`r10`/`r8`/`r9`=args, return in `rax`, `rcx` and `r11` clobbered.

## 13. Memory alignment — MMIO and volatile

MMIO accesses (`mmio_read32`/`mmio_write32`) and volatile accesses (`read_volatile`/`write_volatile`) require correct alignment. An unaligned access on aarch64 causes a `Bus Error` (SIGBUS).

**Rule**: All 32-bit accesses must be aligned to 4 bytes. Use `align_up()` from `common/alignment.rs` if needed.

## 14. Stress tests — do not run in production

The `stress_sequential` tests deliberately attempt to exhaust resources (100% CPU, 100% RAM, 100% swap, 10,000 GPU submissions). The Guardian stops them at 80%/50%, but during execution the system will be under heavy load.

**Never**: Run stress tests on a production server, a machine with unsaved data, or a VM with little RAM.

## 15. Thermal — temperature readings may be inaccurate

CPU temperature reading (`cpu/thermal.rs`) uses MSR 0x19C (Intel only). On AMD or CPUs without this MSR, the reading returns 0 with no error.

**Never**: Rely on the returned temperature for critical decisions (hardware throttling, emergency shutdown) without verifying that the MSR is valid for the CPU.

## 16. PCI BAR probing — can disrupt active devices

BAR probing (`bus/pci/`) writes `0xFFFFFFFF` to the BAR register, reads the size, then restores the original value. During those few cycles, the device is inaccessible.

**Never**: Perform BAR probing on a device in active use (GPU with active display, NIC with network traffic).

## 17. reboot() and shutdown() — irreversible

`power::reboot()` writes `0xFE` to port 0x64 (keyboard controller). `power::shutdown()` writes `0x2000` to port 0x604 (ACPI). These operations are **irreversible and immediate** if the system has I/O privileges.

**Never**: Call these functions in a test or by accident. They actually reboot/power off the machine.

## 18. DeviceTree — parsing untrusted data

The FDT parser (`firmware/devicetree.rs`) reads binary blobs. A malformed blob can cause out-of-bounds reads.

**Current protection**: The magic `0xD00DFEED` is verified, and offsets are bounded by `totalsize`.

**Rule**: Never trust offsets in an FDT without verifying they are within the blob bounds.

## 19. UEFI runtime services — dangerous calls

UEFI table pointers (`RuntimeServicesTable`) can point to invalid addresses if the firmware is not UEFI or if runtime services are unavailable.

**Current protection**: `parse_uefi()` checks for the magic at 0x80000000.

**Never**: Call a UEFI function pointer without verifying that `UefiInfo` was correctly initialized.

## 20. The crate has zero dependencies — by design

There is no libc, no global allocator, no `std`. Any function that assumes the existence of `malloc`, `printf`, `pthread`, or `errno` will not work.

**Rule**: All buffers must be on the stack or allocated via `sys_mmap_anon()`. All text output goes through `write_stderr()`. All synchronization uses atomics.

## Design invariants — do not break these

1. **All modules are `mod` (private)** in `lib.rs`. Only `pub mod sys` is exported. Do not make internal modules public.
2. **Zero `#[cfg]`** anywhere in the crate. Architecture dispatch is runtime-only via shims.
3. **`extern "C"` used only for calling convention** on machine code blobs (syscall, CPUID) — no C library dependency, no foreign function linkage.
4. **Every `unsafe` function has a `/// # Safety` doc comment.** Clippy enforces this. Do not remove them.
5. **`OnceCopy::set()` returns `bool`** — the first caller wins. Do not retry or force-set.
6. **Atomic ordering**: `Acquire` for loads, `Release` for stores, `AcqRel` for CAS. Do not use `Relaxed` for synchronization variables.
7. **No `static mut`** in library code. All mutable state uses atomics or `OnceCopy`.
8. **No `unwrap()` or `expect()`** in library code. Every fallible path returns `Option` or a safe default.
