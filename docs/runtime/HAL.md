# HAL — Hardware Abstraction Layer

## Overview

The HAL (`runtime/hal.rs`) provides a unified function-level interface to all hardware subsystems. It re-exports functions from lower-level modules into a flat API surface.

## API categories

### Memory

| Function | Description |
|----------|-------------|
| `phys_region()` | Returns physical memory region info |
| `init_phys()` | Initializes physical allocator |
| `alloc_frame()` | Allocates a physical frame |
| `free_frame(frame)` | Frees a physical frame |
| `map_page(virt, phys)` | Maps a virtual page to physical |
| `unmap_page(virt)` | Unmaps a virtual page |
| `map(virt, phys, size)` | Maps a range |
| `unmap(virt, size)` | Unmaps a range |
| `detect_cache()` | Detects cache hierarchy |
| `ensure_cache_coherence()` | Flushes caches |
| `detect_numa_nodes()` | Probes NUMA topology |
| `new_bump_allocator()` | Creates a bump allocator |
| `new_buddy_allocator()` | Creates a buddy allocator |
| `new_slab()` | Creates a slab allocator |

### Interrupts

| Function | Description |
|----------|-------------|
| `register(irq, handler)` | Registers interrupt handler |
| `unregister(irq)` | Removes interrupt handler |
| `dispatch(irq)` | Manually dispatches |
| `enable_irq(irq)` | Enables an IRQ |
| `disable_irq(irq)` | Disables an IRQ |
| `eoi(irq)` | Sends end-of-interrupt |
| `init_idt()` | Initializes IDT (x86_64) |

### Bus

| Function | Description |
|----------|-------------|
| `register_device(vendor, device, bus)` | Registers a bus device |
| `device_count()` | Number of bus devices |

### DMA

| Function | Description |
|----------|-------------|
| `prepare(buf, flags)` | Creates DMA descriptor |
| `enqueue(desc)` | Submits descriptor |
| `dequeue()` | Retrieves completed descriptor |
| `reap()` | Reaps completed transfers |

### IOMMU

| Function | Description |
|----------|-------------|
| `create_domain(base, size, flags)` | Creates IOMMU domain |
| `unmap_iova(iova)` | Unmaps an IOVA |
| `translate_iova(iova)` | Translates IOVA to physical |

### Firmware

| Function | Description |
|----------|-------------|
| `detect()` | Detects firmware type |
| `query_acpi()` | Queries ACPI |
| `query_uefi()` | Queries UEFI |
| `query_devicetree()` | Queries DeviceTree |
| `query_smbios()` | Queries SMBIOS |

### Security

| Function | Description |
|----------|-------------|
| `detect()` | Detects security features |
| `enable_isolation()` | Enables isolation domains |
| `enable_enclave()` | Enables SGX enclaves |

### Hardware access

| Function | Description |
|----------|-------------|
| `cpuid(leaf, subleaf)` | Reads CPUID |
| `msr(msr)` | Reads MSR |
| `mmio(addr)` | Reads MMIO |
