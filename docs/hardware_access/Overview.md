# Hardware Access Module

The `hardware_access` module provides rate-limited, Guard-protected access to privileged hardware operations.

## Submodules

| File | Description |
|------|-------------|
| `api.rs` | Gated CPUID, MSR, MMIO, and MIDR access |

## API

| Function | Returns | Description |
|----------|---------|-------------|
| `read_cpuid(leaf, subleaf)` | `Option<(u32, u32, u32, u32)>` | Reads CPUID (EAX, EBX, ECX, EDX) |
| `read_msr(msr)` | `Option<u64>` | Reads a Model-Specific Register |
| `mmio_read32(addr)` | `Option<u32>` | Reads 32-bit MMIO |
| `mmio_write32(addr, val)` | `bool` | Writes 32-bit MMIO |
| `read_aarch64_midr()` | `Option<u64>` | Reads AArch64 MIDR_EL1 |

## Guard limits

All operations are rate-limited by Guard atomics:

| Guard | Limit | Covers |
|-------|-------|--------|
| `CPU_GUARD` | 1024 operations | CPUID, MSR reads |
| `MMIO_GUARD` | 10000 operations | MMIO reads and writes |

When the guard limit is reached, the function returns `None` / `false`.

## Delegation chain

```
hardware_access::api      (Guard rate-limit check)
  → arch::shim            (OnceCopy function pointer dispatch)
    → arch::x86_64 / aarch64   (actual hardware instruction)
```

## Re-exports

```rust
pub use api::{mmio_read32, mmio_write32, read_cpuid, read_msr};
```

## Safety considerations

- Guard limits prevent runaway hardware access loops
- MSR writes are not exposed (read-only)
- See [Warnings.md](../Warnings.md) warning 7 for MSR risks
