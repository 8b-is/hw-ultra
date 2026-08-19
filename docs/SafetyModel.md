# Safety Model

## Principles

The `hardware` crate manipulates hardware directly: registers, I/O ports, DMA buffers, MMIO regions, and raw syscalls. Safety is enforced through multiple layers.

## Layer 1 — Shim isolation

All architecture-specific operations go through `OnceCopy<fn(...)>` function pointers. This means:
- No raw hardware instruction is ever called unconditionally
- If the shim is not initialized, a safe default is returned
- The same code compiles for both x86_64 and aarch64 without `#[cfg]`

## Layer 2 — Guardian resource limits

The Guardian (`arch/guardian/`) enforces hard caps on resource usage via two checks:
- **Capacity ceiling** (`bounded()`) — RAM: 80%, Swap: 50%, CPU: 80%, IRQ/DMA/Heap: configurable
- **Surge rate limiter** (`check_surge()`) — sliding-window budget per resource, prevents consumption spikes. Uses CAS loop for lock-free atomic correctness.

Every allocation attempt goes through `gate_*()` functions. Both checks must pass — if either limit is exceeded, the allocation is denied. Usage readers (`set_memory_reader()` etc.) can inject real hardware usage instead of relying on internal counters. `guardian_snapshot()` provides auditable state.

## Layer 3 — Guard rate limiting

Sensitive operations use `Guard` structs with atomic counters:
- CPUID reads: max 1024 operations
- MSR reads: max 1024 operations
- MMIO reads: max 10,000 operations

This prevents infinite loops from misconfigured callbacks.

## Layer 4 — Privilege gating

I/O port access requires `HW_PRIVILEGE` (`AtomicBool`) to be set via `request_hw_privilege()`. Without it:
- Port reads return `0xFF` / `0xFFFFFFFF`
- Port writes are silently ignored
- No SIGSEGV can occur

## Layer 5 — OnceCopy write-once semantics

All global function pointers and singletons use `OnceCopy<T>` with a 3-state CAS:
- `EMPTY(0)` → `WRITING(1)` → `READY(2)`
- `set()` returns `false` if already initialized
- No value can ever be overwritten after first initialization

## Layer 6 — Atomic-only mutable state

The crate contains zero instances of `static mut`. All mutable global state uses:
- `AtomicUsize`, `AtomicBool`, `AtomicI64`, `AtomicU32`
- `OnceCopy<T>` for write-once values
- `Once<T>` for complex singleton initialization

## Layer 7 — Clippy-enforced documentation

Every `unsafe` function has a `/// # Safety` documentation comment. Clippy's `undocumented_unsafe_blocks` lint ensures this. Removing these comments will cause CI failure.

## What happens on failure

| Failure | Behavior |
|---------|----------|
| Uninitialized shim called | Returns safe default (0, None, false) |
| Resource limit exceeded | Returns false / None |
| Guard counter exhausted | Returns None |
| No I/O privilege | Returns 0xFF, writes ignored |
| Invalid MMIO address | Returns None |
| Unknown architecture | `Architecture::Unknown`, safe defaults everywhere |
