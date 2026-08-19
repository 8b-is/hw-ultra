# DMA Buffer

## Overview

`DmaBuffer` represents a contiguous, physically-addressable memory region suitable for DMA transfers. Buffers are allocated through the physical frame allocator to guarantee contiguity.

## Structure

```
DmaBuffer {
    ptr: *mut u8     — virtual address of the buffer
    len: usize       — size in bytes
}
```

## Allocation

`DmaBuffer::new(size: usize, align: usize) -> Option<Self>`

1. Rounds `size` up to frame alignment
2. Allocates contiguous physical frames
3. Maps frames into virtual address space
4. Returns `None` if allocation fails

The alignment parameter ensures the buffer starts at an address aligned to `align` bytes. Common alignments:
- 4096 (page-aligned) — standard DMA
- 64 — cache-line aligned for performance
- 16 — minimum for most DMA controllers

## Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `as_ptr()` | `*mut u8` | Raw pointer to the buffer |
| `len()` | `usize` | Buffer size in bytes |
| `is_empty()` | `bool` | Whether length is zero |
| `phys_addr()` | `usize` | Physical address |
| `frame()` | `Frame` | The underlying physical frame |

## Usage with DmaEngine

```
let buf = DmaBuffer::new(4096, 4096)?;
let desc = DmaEngine::prepare_descriptor(&buf, flags);
engine.submit(&[desc]);
```

Or in one step with IOMMU mapping:

```
engine.submit_buffer(&buf, flags, 4096)?;
```

## Safety rules

- Never free a `DmaBuffer` while a DMA transfer is in progress
- Never access buffer contents while a device is writing to it (cache coherency)
- Always unmap the descriptor/IOVA before freeing the buffer
