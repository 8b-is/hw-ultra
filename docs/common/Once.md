# Once and OnceCopy

Thread-safe write-once primitives used for all global singletons and function pointer registration.

## OnceCopy<T: Copy>

For `Copy` types (function pointers, integers, booleans):

```rust
static MY_FN: OnceCopy<fn(u32) -> u32> = OnceCopy::new();

// Registration (first call wins)
assert!(MY_FN.set(implementation_a));     // returns true
assert!(!MY_FN.set(implementation_b));    // returns false, value unchanged

// Usage
if let Some(f) = MY_FN.get() {
    let result = f(42);
}
```

## Once<T>

For non-Copy types (structs with interior mutability):

```rust
static ENGINE: Once<DmaEngine> = Once::new();

// Registration
ENGINE.set(DmaEngine::init());

// Usage
if let Some(engine) = ENGINE.get() {
    engine.submit(&descriptors);
}
```

## Internal state machine

Both use a 3-state atomic CAS:

```
EMPTY (0) ──set()──> WRITING (1) ──store──> READY (2)
     │                                          │
     └── get() returns None          get() returns Some(value)
```

- `EMPTY → WRITING`: CAS succeeds, caller writes the value
- `WRITING → READY`: value is stored, state finalized
- Any call to `set()` when state is not `EMPTY`: returns `false`
- Any call to `get()` when state is not `READY`: returns `None`

## Thread safety

The CAS ensures that:
- Only one thread can transition `EMPTY → WRITING`
- Other threads calling `set()` see `WRITING` or `READY` and return `false`
- `get()` only returns `Some` after the value is fully written

## Usage throughout the crate

| Static | Type | Module |
|--------|------|--------|
| `DMA_ENGINE` | `Once<DmaEngine>` | `dma/engine` |
| `IOMMU_ONCE` | `Once<IommuController>` | `iommu/controller` |
| `TPU_DEVICE` | `Once<TpuDevice>` | `tpu/device` |
| `LPU_DEVICE` | `Once<LpuDevice>` | `lpu/device` |
| `GLOBAL_CONTROLLER` | `Once<Controller>` | `interrupt/controller` |
| `IDT_ONCE` | `Once<Idt>` | `interrupt/idt` |
| All shim function pointers | `OnceCopy<fn(...)>` | `arch/shim` |
