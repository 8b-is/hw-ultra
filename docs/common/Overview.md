# Common Module

The `common` module provides low-level utilities used throughout the crate. All types are `no_std` compatible and use no heap allocation.

## Submodules

| File | Description |
|------|-------------|
| `atomic.rs` | Wrapper around `AtomicUsize` with simplified API |
| `volatile.rs` | `read_volatile` / `write_volatile` wrappers |
| `bitfield.rs` | Bit extraction and insertion for register fields |
| `registers.rs` | 32-entry `AtomicUsize` register bank |
| `barrier.rs` | Compiler and memory fences |
| `endian.rs` | Byte-order detection and conversion |
| `alignment.rs` | Address alignment utility |
| `once.rs` | `Once<T>` and `OnceCopy<T>` write-once primitives |
| `guard.rs` | Rate-limiting guard with atomic counter |
| `error.rs` | Minimal error enum |

## Atomic

```rust
pub struct Atomic(AtomicUsize);

impl Atomic {
    pub fn new(val: usize) -> Self
    pub fn load(&self) -> usize
    pub fn store(&self, val: usize)
    pub fn swap(&self, val: usize) -> usize
    pub fn compare_exchange(&self, current: usize, new: usize) -> Result<usize, usize>
    pub fn compare_exchange_weak(&self, current: usize, new: usize) -> Result<usize, usize>
}
```

## Volatile

```rust
pub unsafe fn read_volatile<T>(src: *const T) -> T
pub unsafe fn write_volatile<T>(dst: *mut T, val: T)
```

Thin wrappers around `core::ptr::read_volatile`/`write_volatile`.

## BitField

```rust
pub struct BitField { offset: u8, width: u8 }

impl BitField {
    pub fn new(offset: u8, width: u8) -> Self
    pub fn mask_u32(&self) -> u32
    pub fn mask_u64(&self) -> u64
    pub fn extract_u32(&self, value: u32) -> u32
    pub fn extract_u64(&self, value: u64) -> u64
    pub fn insert_u32(&self, original: u32, field_val: u32) -> u32
    pub fn insert_u64(&self, original: u64, field_val: u64) -> u64
}
```

## Registers

```rust
pub fn write_register(index: usize, value: usize) -> bool
pub fn read_register(index: usize) -> Option<usize>
pub fn register_count() -> usize
```

32-entry bank backed by `[AtomicUsize; 32]`.

## Barrier

```rust
pub fn compiler_fence()   // Ordering::SeqCst compiler fence
pub fn memory_fence()     // Ordering::SeqCst memory fence
pub fn load_fence()       // Ordering::Acquire fence
pub fn store_fence()      // Ordering::Release fence
```

## Endian

```rust
pub fn is_little_endian() -> bool
pub fn swap_u16(val: u16) -> u16
pub fn swap_u32(val: u32) -> u32
pub fn swap_u64(val: u64) -> u64
pub fn u16_from_be(val: u16) -> u16
pub fn u32_from_be(val: u32) -> u32
pub fn u16_to_be(val: u16) -> u16
pub fn u32_to_be(val: u32) -> u32
```

## Alignment

```rust
pub const fn align_up(addr: usize, align: usize) -> usize
```

Rounds `addr` up to the next multiple of `align`. `align` must be a power of 2.

## Once / OnceCopy

```rust
pub struct Once<T> { ... }
impl<T> Once<T> {
    pub fn set(&self, val: T) -> bool     // true if first call
    pub fn get(&self) -> Option<&T>
}

pub struct OnceCopy<T: Copy> { ... }
impl<T: Copy> OnceCopy<T> {
    pub fn set(&self, val: T) -> bool
    pub fn get(&self) -> Option<T>
}
```

Thread-safe write-once primitives using 3-state CAS: `EMPTY → WRITING → READY`.

## Guard

```rust
pub struct Guard { max_ops: u32, counter: AtomicU32 }

impl Guard {
    pub fn new(max_ops: u32) -> Self
    pub fn allow(&self) -> bool    // Increments counter, returns false if limit reached
}
```

## Error

```rust
pub enum Error { Unspecified }

pub const ERR_NOT_IMPLEMENTED: i64 = -1;
pub const ERR_NOT_FOUND: i64 = -2;
```

`ERR_NOT_IMPLEMENTED` replaces all former uses of `-38` (Linux `ENOSYS`) and `0` as default syscall numbers. `ERR_NOT_FOUND` is a generic "not found" sentinel. Both are platform-independent.
