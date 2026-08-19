# Initialization

The `init::core::init()` function orchestrates the full system bring-up in 17 sequential phases. Each phase depends on the previous ones completing successfully.

## Phase order

| # | Phase | Function | Dependencies |
|---|-------|----------|-------------|
| 1 | Shims | `init_shims()` | None |
| 2 | Config | `init_config()` | Shims |
| 3 | Common | `init_common()` | Shims |
| 4 | Firmware | `init_firmware()` | Shims, MMIO |
| 5 | Memory | `init_memory()` | Firmware |
| 6 | Interrupts | `init_interrupts()` | Memory, Firmware |
| 7 | Bus | `init_bus()` | MMIO, Memory |
| 8 | DMA | `init_dma()` | Memory, Bus |
| 9 | IOMMU | `init_iommu()` | Firmware, Memory |
| 10 | CPU | `init_cpu()` | Shims, CPUID |
| 11 | Security | `init_security()` | CPU, CPUID |
| 12 | Discovery | `init_discovery()` | Bus, MMIO |
| 13 | Timers | `init_timers()` | Firmware, I/O |
| 14 | Accelerators | `init_accelerators()` | Bus, DMA, IOMMU |
| 15 | Topology | `init_topology()` | CPU |
| 16 | Debug | `init_debug()` | Timers |
| 17 | Power | `init_power()` | CPU, Firmware |

## Phase 1 — Shims

`init_shims()` uses `compare_exchange(0, 1)` to ensure single-entry execution. It detects the architecture and registers platform-specific function pointers for CPUID, MSR, MMIO, MIDR, exit, mkdir, and scan_dir. It also auto-registers the native syscall handler via machine code blobs.

## Phase 2 — Config

Probes CPU features (SSE, AVX, NEON) and firmware features (ACPI, UEFI, DeviceTree) and registers them as feature flags.

## Phase 3 — Common

Verifies that common utilities (registers, bitfields, atomics) are functioning.

## Phase 4 — Firmware

Probes firmware interfaces in order:
1. ACPI — scans 0xE0000–0x100000 for RSDP, parses RSDT/XSDT
2. UEFI — checks `/sys/firmware/efi/` or magic at 0x80000000
3. DeviceTree — parses FDT if ACPI is not present
4. SMBIOS — scans for SMBIOS entry point

## Phase 5 — Memory

Detects available physical memory, initializes the frame allocator (8192 frames), sets up virtual memory paging, and configures the cache hierarchy.

## Phase 6 — Interrupts

Initializes the interrupt controller:
- x86_64: programs PIC/APIC, loads IDT with 256 entries
- aarch64: configures GIC (Generic Interrupt Controller)

## Phase 7 — Bus

Enumerates system buses:
- PCI: scans 256 buses × 32 devices × 8 functions
- PCIe: probes ECAM regions from MCFG ACPI table
- AMBA: reads peripheral IDs from ARM memory-mapped regions
- Virtio: detects virtio transport devices

## Phase 8 — DMA

Creates the DMA engine with a 128-entry descriptor ring buffer. Initializes as a global singleton via `Once<DmaEngine>`.

## Phase 9 — IOMMU

Probes for IOMMU hardware:
- Intel VT-d: base address from ACPI DMAR table
- ARM SMMU: base address from DeviceTree

Initializes IOVA allocator. Domain base, size, and IOVA mappings are derived from the created domain parameters at runtime — no hardcoded addresses.

## Phase 10 — CPU

Gathers full CPU information: architecture, vendor, model, frequency, cache sizes, core count, hyperthreading, and per-core details.

## Phase 11 — Security

Checks SGX support, enables speculation mitigations (IBRS, IBPB, STIBP, SSBD, Retpoline as appropriate).

## Phase 12 — Discovery

Populates the unified device registry with all discovered devices (CPU, GPU, TPU, LPU, network, storage, IOMMU).

## Phase 13 — Timers

Registers available clock sources:
- x86_64: TSC (primary), HPET (from ACPI), PIT (fallback)
- aarch64: CNTVCT_EL0 (ARM generic timer)

## Phase 14 — Accelerators

Initializes GPU, TPU, and LPU devices found during bus enumeration. Creates `AccelHandle` instances for each.

## Phase 15 — Topology

Detects system topology: socket count, cores per socket, threads per core, NUMA nodes, interconnect layout.

## Phase 16 — Debug

Sets up performance monitoring counters and trace infrastructure.

## Phase 17 — Power

Configures the power governor (default: OnDemand), reads initial thermal zone temperatures, and registers ACPI power management if available.
