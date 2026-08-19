# Sleep States

## Overview

The `sleep` module manages ACPI sleep states (S-states) for system-level power management.

## SleepState

```rust
pub enum SleepState {
    S0,  // Working state — fully operational
    S1,  // Standby — CPU clocks stopped, context preserved
    S3,  // Suspend to RAM — only RAM powered
    S5,  // Soft off — requires full boot to resume
}
```

## API

| Function | Description |
|----------|-------------|
| `suspend(state)` | Enters the specified sleep state |
| `current_state()` | Returns the current sleep state |

## State transitions

```
S0 (Working)
 ├─► S1 (Standby)   → fast resume, CPU idle
 ├─► S3 (Suspend)   → slow resume, minimal power
 └─► S5 (Soft off)  → requires reboot
```

## Implementation

Sleep state entry on x86_64 involves:
1. Saving CPU context (all registers)
2. Writing the appropriate value to `PM1a_CNT_BLK` (from FADT)
3. On resume: restoring CPU context

S5 is implemented as `shutdown()` — see `power/core.rs`.

## Safety considerations

- S3 requires all DMA transfers to be complete or fenced
- S3 requires all interrupt handlers to be quiesced
- See [Warnings.md](../Warnings.md) warning 17 for reboot/shutdown safety
