# Memory Module

The `memory` module manages physical memory allocation, virtual address mapping, cache coherence, heap allocators, and NUMA topology.

## Submodules

| Submodule | Description |
|-----------|-------------|
| `info` | `MemoryInfo` struct, detection via callback |
| `phys/` | Physical frame allocator (8192 frames) |
| `virt/` | Virtual address mapping and paging |
| `cache/` | Cache hierarchy and coherence |
| `heap/` | Heap allocators: bump, buddy, slab |
| `numa/` | NUMA node detection |

## MemoryInfo

```rust
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub available_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_free_bytes: u64,
}
```

Detected via a registered callback function:

```rust
pub fn detect_memory_info() -> Option<MemoryInfo>
pub fn set_detect_memory_fn(f: impl Fn() -> Option<MemoryInfo>)
```

The callback is registered by the consumer via `set_detect_memory_fn()`. How memory is detected depends on the consumer (e.g. firmware tables, device tree, or OS-specific mechanisms in test code). The library itself contains zero OS-specific paths.

## Physical memory

### Frame

```rust
pub struct Frame(usize);
impl Frame {
    pub fn new(pfn: usize) -> Self
    pub fn as_usize(self) -> usize
}
```

A frame represents a single physical page (4 KiB).

### PhysAllocator

```rust
pub struct PhysAllocator;
impl PhysAllocator {
    pub fn region() -> Option<(usize, usize)>   // (base, size)
    pub fn init() -> bool
    pub fn alloc_frame() -> Option<Frame>
    pub fn free_frame(f: Frame)
}
```

Manages up to 8192 frames using a free list.

## Virtual memory

| File | Description |
|------|-------------|
| `address.rs` | Virtual address representation |
| `mapping.rs` | Virtual-to-physical page mapping |
| `paging.rs` | Page table management |

## Cache

| File | Description |
|------|-------------|
| `hierarchy.rs` | L1/L2/L3 cache detection |
| `coherence.rs` | Cache coherence operations |

## Heap allocators

Three allocators, all operating on pre-allocated fixed-size backing buffers (no `mmap` or `brk`):

| Allocator | Description |
|-----------|-------------|
| `bump` | Fast linear allocator, no free support |
| `buddy` | Power-of-two block allocator with free/merge |
| `slab` | Fixed-size object pool allocator |

## NUMA

| File | Description |
|------|-------------|
| `node.rs` | NUMA node representation and detection |
