# DMA Module

The `dma` module manages Direct Memory Access operations: buffer allocation, descriptor ring management, address mapping, and IOMMU integration.

## Submodules

| File | Description |
|------|-------------|
| `engine.rs` | `DmaEngine` with 128-entry descriptor ring |
| `buffer.rs` | `DmaBuffer` for contiguous DMA-capable memory |
| `descriptor.rs` | `Descriptor` struct (phys addr + length + flags) |
| `mapping.rs` | Virtual-to-physical address mappings for DMA |

## DmaEngine

```
DmaEngine {
    ring: [Descriptor; 128]
    head: AtomicUsize     — producer index
    tail: AtomicUsize     — consumer index
}
```

Singleton via `Once<DmaEngine>`.

### API

| Method | Description |
|--------|-------------|
| `init()` | Creates a new engine with empty ring |
| `prepare_descriptor(buf, flags)` | Creates descriptor from a `DmaBuffer` |
| `prepare_descriptor_with_phys(phys, len, flags)` | Creates descriptor from raw address |
| `submit(descs)` | Submits descriptors to the ring, returns count |
| `submit_buffer(buf, flags, align)` | Submits a buffer with IOMMU mapping |
| `drain(out)` | Drains completed descriptors, returns count |
| `unmap_descriptor(desc)` | Unmaps a descriptor's IOVA |
| `unmap_iova(iova)` | Unmaps an IOVA directly |
| `get()` | Returns `Option<&'static DmaEngine>` |

### Ring operation

The ring is a simple producer-consumer queue:
- `submit()` advances `head`
- `drain()` advances `tail`
- Ring is full when `(head + 1) % 128 == tail`

## DmaBuffer

```
DmaBuffer {
    ptr: *mut u8     — virtual address
    len: usize       — buffer size
}
```

| Method | Description |
|--------|-------------|
| `new(size, align)` | Allocates a contiguous DMA buffer |
| `as_ptr()` | Returns the virtual address |
| `len()` | Buffer size |
| `phys_addr()` | Physical address (via frame allocator) |
| `frame()` | Associated physical `Frame` |

## Descriptor

```
Descriptor {
    phys: usize     — physical address
    len: usize      — transfer length
    flags: u32      — transfer flags
}
```

## Mapping

Maintains up to 64 virtual-to-physical mappings for DMA regions.

| Function | Description |
|----------|-------------|
| `map(virt, phys, size)` | Adds a mapping |
| `translate(virt)` | Returns physical address for a virtual address |
| `mapping_count()` | Number of active mappings |

## IOMMU integration

When an IOMMU is present, `submit_buffer()` automatically:
1. Allocates an IOVA via `IommuController::alloc_iova()`
2. Maps the DMA buffer through the IOMMU
3. Uses the IOVA (not physical address) in the descriptor

See [Warnings.md](../Warnings.md) warning 5 for DMA safety rules.
