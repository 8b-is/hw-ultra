# Speculation Mitigations

## Overview

The `speculation` module detects and applies CPU speculation vulnerability mitigations (Spectre, Meltdown, MDS, etc.).

## SpeculationMitigation

```rust
pub enum SpeculationMitigation {
    None,        // No mitigation applied
    Ibrs,        // Indirect Branch Restricted Speculation
    Ibpb,        // Indirect Branch Prediction Barrier
    Stibp,       // Single Thread Indirect Branch Predictors
    Ssbd,        // Speculative Store Bypass Disable
    Retpoline,   // Return trampoline (software mitigation)
}
```

## API

| Function | Description |
|----------|-------------|
| `mitigations_active()` | Returns `true` if any mitigation is applied |
| `active_mitigation()` | Returns the current `SpeculationMitigation` |
| `mitigations()` | Detects CPU vulnerabilities and applies mitigations |

## Detection and selection

`mitigations()` performs:
1. Reads CPUID leaf 7 for mitigation support bits
2. Selects the strongest available mitigation
3. Writes the appropriate MSR to enable it

| Mitigation | CPUID bit | MSR | Protects against |
|------------|-----------|-----|-----------------|
| IBRS | EDX[26] | `0x48` (SPEC_CTRL) | Spectre v2 |
| IBPB | EDX[26] | `0x49` (PRED_CMD) | Spectre v2 |
| STIBP | EDX[27] | `0x48` (SPEC_CTRL) | Cross-HT Spectre |
| SSBD | EDX[31] | `0x48` (SPEC_CTRL) | Spectre v4 |
| Retpoline | N/A | N/A | Spectre v2 (software) |

## AArch64

On AArch64, speculation mitigations use:
- CSV2 (Cache Speculation Variant 2) — kernel page table isolation
- SSBS (Speculative Store Bypass Safe) — system register
