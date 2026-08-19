# Constants and Limits

Fixed-size limits used throughout the crate. Since there is no heap allocator, all data structures use statically-sized arrays.

## Core limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| Register bank size | 32 | `common/registers` | General-purpose register file |
| Max cores | 64 | `cpu/core` | Maximum tracked CPU cores |
| Model name length | 48 | `cpu/info` | CPU model name buffer |
| Context registers | 16 | `cpu/context` | Saved register context |

## Bus limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| Max PCI devices | 64 | `bus/discovery` | Discovered device slots |
| PCI buses | 256 | `bus/pci/device` | Full BDF scan range |
| PCI devices/bus | 32 | `bus/pci/device` | Devices per bus |
| PCI functions/device | 8 | `bus/pci/device` | Functions per device |
| PCIe links | 8 | `bus/pcie/topology` | PCIe topology links |
| Max capabilities | 16 | `bus/pci/capability` | PCI capability entries |

## DMA limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| Ring buffer size | 128 | `dma/engine` | DMA descriptor ring entries |
| DMA mappings | 64 | `dma/mapping` | Virtual-to-physical mappings |

## IOMMU limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| Max domains | 16 | `iommu/domain` | IOMMU protection domains |
| Max mappings | 64 | `iommu/mapping` | IOVA-to-physical mappings |
| IOVA base | 0x1_0000_0000 | `iommu/controller` | IOVA address space start |
| IOVA limit | 0x2_0000_0000 | `iommu/controller` | IOVA address space end |

## Interrupt limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| Handler table | 256 | `interrupt/controller` | IRQ handler slots |
| IDT entries | 256 | `interrupt/idt` | Interrupt descriptor table |

## Memory limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| Free frames | 8192 | `memory/phys/allocator` | Physical frame pool |
| Page size | 4096 | `memory/phys/frame` | Standard page frame |

## Firmware limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| FDT nodes | 128 | `firmware/devicetree` | DeviceTree parsed nodes |
| FDT devices | 64 | `firmware/devicetree` | DeviceTree device entries |
| FDT compatible length | 128 | `firmware/devicetree` | Compatible string buffer |
| FDT node name | 64 | `firmware/devicetree` | Node name buffer |

## Accelerator limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| GPU command queue | 64 | `gpu/queue` | GPU command ring entries |
| GPU compute kernels | 16 | `gpu/compute/kernel` | Registered compute kernels |
| TPU compiled graph | 16 | `tpu/compiler` | Graph node limit |
| Tensor dimensions | 4 | `tpu/tensor` | Max tensor rank |

## Guardian limits (defaults)

| Resource | Default limit | Description |
|----------|--------------|-------------|
| RAM | 80% | Maximum RAM usage (capacity ceiling) |
| Swap | 50% | Maximum swap usage (capacity ceiling) |
| CPU threads | 80% | Maximum CPU utilization (capacity ceiling) |

In addition to the capacity ceiling, each resource can have a **surge budget** — a sliding-window rate limit that caps how much can be allocated per time window. Surge is disabled by default (window = 0). Configure via `set_memory_surge(window_ns, budget)` etc.

## Other limits

| Constant | Value | Module | Description |
|----------|-------|--------|-------------|
| Thermal zones | 8 | `thermal/api` | Temperature monitoring zones |
| Config capabilities | 16 | `config/capability` | Capability registry slots |
| Feature flags | 16 bits | `config/feature` | Hardware feature bitmask |
| Discovery devices | 32 | `discovery/registry` | Discovered device registry |
| Syscall handlers | 256 | `syscall/api` | Syscall dispatch table |
| Security enclaves | 8 | `security/enclaves` | SGX enclave slots |
| Isolation domains | 16 | `security/isolation` | Isolation domain slots |
| Guard CPUID ops | 1024 | `hardware_access/api` | CPUID rate limit |
| Guard MMIO ops | 10000 | `hardware_access/api` | MMIO rate limit |
