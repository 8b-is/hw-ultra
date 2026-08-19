# Module Index

Complete listing of all 36 modules declared in `lib.rs`.

## Core infrastructure

| Module | Description |
|--------|-------------|
| `arch` | Architecture detection, shim layer, x86_64/aarch64 backends, Guardian resource gating |
| `common` | Atomic wrappers, volatile I/O, bitfields, registers, barriers, endianness, alignment, once-init, guard |
| `hardware_access` | Rate-limited CPUID, MSR, and MMIO access API |
| `boot` | Boot-time memory map and usable RAM detection |

## CPU & memory

| Module | Description |
|--------|-------------|
| `cpu` | CPU detection (CPUID/MIDR), features, frequency, topology, affinity, context, scheduler, thermal, SIMD |
| `memory` | Physical frame allocator, virtual memory mapping, cache coherence, heap allocators (bump/buddy/slab), NUMA |

## Firmware

| Module | Description |
|--------|-------------|
| `firmware` | ACPI table discovery (RSDP/RSDT/XSDT/FADT/HPET/MADT), UEFI (GOP, runtime services), DeviceTree (FDT parser), SMBIOS |

## Bus & I/O

| Module | Description |
|--------|-------------|
| `bus` | PCI/PCIe enumeration, BAR probing, capability parsing, AMBA bus, Virtio device detection |
| `dma` | DMA buffer allocation, 128-entry descriptor ring, IOMMU-aware submission |
| `iommu` | IOMMU controller (Intel VT-d / ARM SMMU), domain management, IOVA allocation and mapping |

## Interrupts & timers

| Module | Description |
|--------|-------------|
| `interrupt` | 256-vector interrupt controller, IDT management, IRQ enable/disable/EOI |
| `timer` | PIT, HPET, ARM generic timer, clocksource/clockevent abstraction |

## Accelerators

| Module | Description |
|--------|-------------|
| `gpu` | GPU detection, command queue (64 entries), shader management, compute kernels, VRAM allocation, DRM interface |
| `tpu` | TPU device management, tensor operations, graph compilation, DMA transfers, IRQ handling |
| `lpu` | LPU (Language Processing Unit) device, inference pipeline, quantization, task scheduling |

## Device subsystems

| Module | Description |
|--------|-------------|
| `audio` | Audio codec, mixer, stream management, format handling |
| `camera` | Camera sensor, frame capture, processing pipeline |
| `display` | Display detection, framebuffer, backlight, timing |
| `input` | Keyboard, touchscreen, button, event handling |
| `modem` | Cellular modem, SIM, signal, network, protocol |
| `nfc` | NFC device, tag reading, protocol, polling |
| `sensor` | Sensor detection, calibration, data sampling |
| `storage` | Block device, partition, command queue |
| `usb` | USB device/hub, descriptors, endpoints, transfers |

## System management

| Module | Description |
|--------|-------------|
| `power` | Reboot, shutdown, DVFS, sleep states (S0/S1/S3/S5), governor policies |
| `thermal` | Thermal zone management (8 zones), temperature probing |
| `topology` | System topology detection (sockets, cores, interconnect) |
| `security` | SGX enclaves, isolation domains, speculation mitigations |
| `net` | Ethernet frame parsing, IPv4, TCP |

## Runtime & configuration

| Module | Description |
|--------|-------------|
| `runtime` | System initialization, HAL abstraction, component monitoring, resource tracking, worker management |
| `init` | 17-phase initialization orchestration |
| `config` | Feature flags (16 features), capability registry (16 slots), target platform selection |
| `discovery` | Device discovery registry (32 devices) |
| `syscall` | Syscall handler registration and dispatch (256 slots) |
| `debug` | Performance counters, trace points, hardware event sampling |

## Public API

| Module | Description |
|--------|-------------|
| `sys` | Sole public module, re-exports everything accessible to external code |
