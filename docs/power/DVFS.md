# DVFS — Dynamic Voltage and Frequency Scaling

## Overview

DVFS allows runtime adjustment of CPU clock frequency (and associated voltage) to balance performance and power consumption.

## API

| Function | Description |
|----------|-------------|
| `set_frequency(freq_hz: u64)` | Sets CPU frequency in Hz |
| `current_frequency() -> u64` | Returns current frequency in Hz |

## Implementation

On x86_64, frequency scaling uses MSR writes:
- `IA32_PERF_CTL` (MSR `0x199`) — sets target P-state
- `IA32_PERF_STATUS` (MSR `0x198`) — reads current P-state

On AArch64, frequency scaling uses system registers or platform-specific MMIO (SCMI, DVFS mailbox).

## Integration with governor

The DVFS module provides the low-level frequency change mechanism. The governor module decides *when* and *what* frequency to set based on its policy.

Typical flow:
1. Governor observes CPU load
2. Governor calls `dvfs::set_frequency(target_hz)`
3. DVFS writes the appropriate MSR/register
4. `current_frequency()` reflects the change
