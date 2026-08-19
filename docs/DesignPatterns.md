# Design Patterns

Recurring patterns and conventions used throughout the `hardware` crate.

## OnceCopy / Once singleton

Global mutable state is initialized exactly once using `OnceCopy<T>` (for `Copy` types) or `Once<T>` (for non-Copy):

```rust
static MY_FN: OnceCopy<fn(u32) -> u32> = OnceCopy::new();

// First call wins, returns true
MY_FN.set(my_implementation);

// Subsequent calls return false, value unchanged
MY_FN.set(other_implementation); // returns false

// Read the value
if let Some(f) = MY_FN.get() {
    let result = f(42);
}
```

Internally uses a 3-state CAS: `EMPTY(0) → WRITING(1) → READY(2)`.

## Guard rate limiting

Sensitive hardware operations are capped with `Guard`:

```rust
static CPU_GUARD: Guard = Guard::new(1024);

pub fn read_cpuid(leaf: u32, subleaf: u32) -> Option<(u32, u32, u32, u32)> {
    if !CPU_GUARD.allow() {
        return None; // rate limit exceeded
    }
    arch::cpuid_count(leaf, subleaf)
}
```

## Guardian resource gating

Resource-intensive operations check the Guardian before proceeding. Each gate performs a dual check — capacity ceiling and surge rate limit:

```rust
if !gate_memory(current, bytes_needed) {
    return None; // would exceed 80% RAM limit or surge budget
}
// proceed with allocation
```

The consumer can inject real usage from the hardware:

```rust
// Inject real RAM usage from page tables
set_memory_reader(|| read_page_table_usage());

// Configure surge: max 64 MB per 100ms window
set_memory_surge(100_000_000, 64 * 1024 * 1024);

// Snapshot for monitoring / graphing
let snap = guardian_snapshot();
```

## Shim delegation

Architecture-specific code follows a consistent pattern:

1. A shim function pointer is declared as `OnceCopy<fn(...)>`
2. A `default_*()` attempts `init_shims()` then retries
3. A `set_*_fn()` registers the real implementation
4. A public function dispatches through the shim

## Atomic state machines

Configuration and status tracking uses atomic integers as state machines:

```rust
static STATE: AtomicUsize = AtomicUsize::new(0); // 0=uninit, 1=initializing, 2=ready

fn init() {
    if STATE.compare_exchange(0, 1, AcqRel, Acquire).is_ok() {
        // do init...
        STATE.store(2, Release);
    }
}
```

## Fixed-size arrays instead of Vec

Since there is no allocator, all collections are fixed-size arrays:
- Register bank: `[AtomicUsize; 32]`
- Interrupt handlers: `[Option<fn()>; 256]`
- DMA ring: `[Descriptor; 128]`
- PCI devices: `[PciDevice; 64]`
- Thermal zones: `[AtomicU32; 8]`
- IOMMU mappings: `[(usize, usize); 64]`

## Option-based error handling

No `Result<T, E>` with complex error types. Functions return:
- `Option<T>` when the operation may not be available
- `bool` for success/failure
- Safe default values (0, empty arrays) when appropriate
- `Result<usize, &'static str>` only for DMA submission where the error message matters
