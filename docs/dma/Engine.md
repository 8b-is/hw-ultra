# DMA Engine

## Overview

The `DmaEngine` is the central component for submitting and completing DMA transfers. It uses a fixed-size ring buffer of 128 descriptors.

## Singleton

```rust
static DMA_ENGINE: Once<DmaEngine>
pub fn init()
pub fn get() -> Option<&'static DmaEngine>
```

## Ring buffer

```
DmaEngine {
    ring: [Descriptor; 128]
    head: AtomicUsize     — next slot to write (producer)
    tail: AtomicUsize     — next slot to read (consumer)
}
```

The ring uses atomic head/tail indices for lock-free operation:
- **Submit path**: writes descriptor at `ring[head % 128]`, increments `head`
- **Drain path**: reads descriptor at `ring[tail % 128]`, increments `tail`
- **Full**: `(head + 1) % 128 == tail` — submit returns 0
- **Empty**: `head == tail` — drain returns 0

## Submit flow

### Without IOMMU

```
prepare_descriptor(buf, flags)  →  Descriptor { phys: buf.phys_addr(), len, flags }
submit(&[desc])                 →  writes to ring, returns count submitted
```

### With IOMMU

```
submit_buffer(buf, flags, align)  →  alloc IOVA → map through IOMMU → submit
```

Returns `Err` if IOVA allocation or IOMMU mapping fails.

## Completion flow

```
drain(out: &mut [Descriptor]) -> usize
```

Copies completed descriptors into `out`, advances `tail`, returns count.

After draining:
- Call `unmap_descriptor(desc)` or `unmap_iova(iova)` to release IOMMU mappings
- Free the `DmaBuffer` if no longer needed

## Capacity

| Limit | Value |
|-------|-------|
| Ring size | 128 descriptors |
| Max in-flight | 127 (one slot reserved for full detection) |
