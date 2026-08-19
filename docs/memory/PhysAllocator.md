# Physical Memory Allocator

Manages physical page frames for DMA buffers, page tables, and device memory mapping.

## Frame

```rust
pub struct Frame(usize);
```

Represents a single 4 KiB physical page, identified by its page frame number (PFN).

## Allocator

The `PhysAllocator` manages a free list of 8192 frame slots:

```rust
pub fn region() -> Option<(usize, usize)>
pub fn init() -> bool
pub fn alloc_frame() -> Option<Frame>
pub fn free_frame(f: Frame)
```

- `region()` returns the managed physical address range `(base, size)`
- `init()` initializes the free list, returns `true` on success
- `alloc_frame()` returns the next free frame, or `None` if exhausted
- `free_frame()` returns a frame to the free list

## Design

The allocator uses a simple free-list backed by a fixed-size array `[AtomicUsize; 8192]`. Each entry stores either a PFN or 0 (free). Allocation scans for the first non-zero entry and removes it via CAS. Freeing scans for the first zero entry and stores the PFN.

This is not a high-performance allocator — it is designed for correctness and simplicity in a `no_std` environment.

## Integration with DMA

`DmaBuffer::new()` calls `alloc_frame()` to back DMA buffers with physical memory. The physical address from `Frame::as_usize()` is used for IOMMU mapping.
