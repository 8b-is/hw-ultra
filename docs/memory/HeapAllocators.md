# Heap Allocators

Three heap allocator implementations, all operating on pre-allocated fixed-size backing buffers. None of them call `mmap`, `brk`, or any system allocator.

## Bump Allocator

The simplest allocator. Advances a pointer through a contiguous buffer.

- **Allocation**: O(1) — increments the bump pointer
- **Deallocation**: not supported (entire buffer freed at once)
- **Use case**: temporary allocations where lifetime matches the buffer

## Buddy Allocator

Power-of-two block allocator with free and merge support.

- **Allocation**: O(log n) — finds smallest sufficient power-of-two block
- **Deallocation**: O(log n) — frees block, merges with buddy if both free
- **Use case**: general-purpose allocation with varying sizes
- **Minimum block**: determined by the backing buffer alignment

## Slab Allocator

Fixed-size object pool allocator.

- **Allocation**: O(1) — pops from free list
- **Deallocation**: O(1) — pushes to free list
- **Use case**: many identically-sized objects (descriptors, control blocks)
- **Object size**: fixed at slab creation time

## Creation (via HAL)

The runtime HAL provides factory functions:
```rust
pub fn new_bump_allocator(buf: &mut [u8]) -> BumpAllocator
pub fn new_buddy_allocator(buf: &mut [u8]) -> BuddyAllocator
pub fn new_slab(buf: &mut [u8], obj_size: usize) -> SlabAllocator
```

## No global allocator

The crate never registers a Rust global allocator (`#[global_allocator]`). All heap-like allocation is explicit through these allocators or `sys_mmap_anon()`.
