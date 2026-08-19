# Arch Module

The `arch` module provides runtime architecture detection and dispatch. It contains the shim layer, architecture-specific backends, and the Guardian resource gating system.

## Submodules

| Submodule | Description |
|-----------|-------------|
| `shim` | Function pointer dispatch layer with `OnceCopy` statics |
| `x86_64` | Intel/AMD x86_64 backend (CPUID, MSR, I/O ports, MMIO) |
| `aarch64` | ARM AArch64 backend (MIDR, system registers, MMIO) |
| `guardian` | Resource gating with dual-layer protection: capacity ceiling + surge rate limiter. Usage reader callbacks, `GuardianSnapshot` for observability |
| `architecture` | `Architecture` enum definition |

## Architecture enum

```rust
pub enum Architecture {
    Unknown,
    X86_64,
    AArch64,
}
```

Detected at runtime by `detect_arch()` which tries CPUID first, then MIDR_EL1. `arch_cached()` reads the cached result without triggering detection (used by `native_cpuid` to avoid recursion).

## Re-exports

The `arch` module re-exports key functions from the shim:
- `arch_cached` — cached architecture (no detection trigger)
- `cpuid_count` — CPUID with leaf/subleaf
- `detect_arch` — runtime architecture detection
- `mmio_read32`, `mmio_write32` — memory-mapped I/O
- `read_aarch64_midr` — ARM Main ID Register
- `read_msr` — Model-Specific Register read

## How it works

1. `init_shims()` is called (exactly once via CAS guard)
2. It detects the architecture
3. It registers the correct backend functions as `OnceCopy` pointers
4. All subsequent calls go through these pointers
5. If detection fails, `Architecture::Unknown` is used and all operations return safe defaults
