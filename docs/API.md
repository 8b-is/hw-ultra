# API Reference

This document provides a summary of the public API exposed through `hardware::sys`.

## System initialization

```rust
System::init() -> System
```
Runs 17 initialization phases and returns a `System` handle. Call this before using any other API.

## CPU

| Function | Signature | Description |
|----------|-----------|-------------|
| `detect_cpu_info` | `() -> Option<CpuInfo>` | Full CPU detection via CPUID/MIDR |
| `has_feature` | `(name: &str) -> bool` | Check for CPU feature (sse, avx, neon, etc.) |
| `read_core_temperatures` | `(out: &mut [Option<i32>]) -> usize` | Read per-core temperatures in Celsius |
| `set_affinity` | `(core_mask: usize)` | Set CPU affinity mask |
| `get_affinity` | `() -> usize` | Get current CPU affinity mask |
| `pin_to_core` | `(core_id: usize)` | Pin current thread to a specific core |
| `estimate_frequency` | `() -> u64` | Estimate CPU frequency in Hz |
| `detect_cores` | `(out: &mut [CoreInfo]) -> usize` | Detect individual core information |

## Memory

| Function | Signature | Description |
|----------|-----------|-------------|
| `detect_memory_info` | `() -> Option<MemoryInfo>` | Detect total/free/swap memory |
| `alloc_frame` | `() -> Option<Frame>` | Allocate a physical page frame |
| `free_frame` | `(f: Frame)` | Free a physical page frame |
| `total_usable_ram` | `() -> usize` | Total usable RAM from boot memory map |

## Interrupts

| Function | Signature | Description |
|----------|-----------|-------------|
| `register` | `(irq: usize, handler: fn()) -> bool` | Register an interrupt handler |
| `unregister` | `(irq: usize) -> bool` | Remove an interrupt handler |
| `dispatch` | `(irq: usize)` | Manually dispatch an interrupt |

## DMA

| Function | Signature | Description |
|----------|-----------|-------------|
| `DmaBuffer::new` | `(size: usize, align: usize) -> Option<Self>` | Allocate DMA buffer |
| `DmaEngine::submit` | `(&self, descs: &[Descriptor]) -> usize` | Submit descriptors to ring |
| `DmaEngine::drain` | `(&self, out: &mut [Descriptor]) -> usize` | Drain completed descriptors |

## Hardware access

| Function | Signature | Description |
|----------|-----------|-------------|
| `read_cpuid` | `(leaf: u32, subleaf: u32) -> Option<(u32,u32,u32,u32)>` | Rate-limited CPUID read |
| `read_msr` | `(msr: u32) -> Option<u64>` | Rate-limited MSR read |
| `mmio_read32` | `(addr: usize) -> Option<u32>` | Rate-limited MMIO read |
| `mmio_write32` | `(addr: usize, val: u32) -> bool` | Rate-limited MMIO write |

## Firmware

| Function | Signature | Description |
|----------|-----------|-------------|
| `acpi_revision` | `() -> usize` | ACPI revision (1 or 2+) |
| `find_ioapic_base` | `() -> Option<usize>` | Find I/O APIC base address |
| `parse_fdt_header` | `(blob: &[u8]) -> Option<FdtHeader>` | Parse Flattened DeviceTree header |
| `parse_uefi` | `()` | Detect and parse UEFI firmware |
| `parse_smbios` | `()` | Detect and parse SMBIOS tables |

## Power

| Function | Signature | Description |
|----------|-----------|-------------|
| `reboot` | `()` | Reboot the machine (requires I/O privilege) |
| `shutdown` | `()` | Power off the machine (requires I/O privilege) |
| `set_frequency` | `(freq_hz: u64)` | Set CPU frequency via DVFS |
| `enter_idle` | `()` | Enter low-power idle state |

## Security

| Function | Signature | Description |
|----------|-----------|-------------|
| `sgx_supported` | `() -> bool` | Check SGX support via CPUID |
| `create_enclave` | `(base: usize, size: usize) -> Option<Enclave>` | Create an SGX enclave |
| `mitigate` | `()` | Apply speculation mitigations |

## Configuration

| Function | Signature | Description |
|----------|-----------|-------------|
| `enable` | `(feature: u32)` | Enable a feature flag |
| `disable` | `(feature: u32)` | Disable a feature flag |
| `is_enabled` | `(feature: u32) -> bool` | Query a feature flag |
| `register_capability` | `(id: usize, value: usize) -> bool` | Register a capability |
