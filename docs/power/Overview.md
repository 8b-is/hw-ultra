# Power Module

The `power` module manages system power states: reboot, shutdown, DVFS (Dynamic Voltage and Frequency Scaling), CPU governors, idle states, sleep states, and thermal monitoring.

## Submodules

| File | Description |
|------|-------------|
| `core.rs` | Reboot and shutdown implementation |
| `dvfs.rs` | Frequency scaling |
| `governor.rs` | Power governor policies |
| `idle.rs` | CPU idle and halt |
| `sleep.rs` | ACPI sleep states (S0/S1/S3/S5) |
| `thermal.rs` | CPU temperature monitoring |

## Module-level API

| Function | Description |
|----------|-------------|
| `reboot()` | Reboots the system |
| `shutdown()` | Shuts down the system |

## Reboot / Shutdown

Both are implemented in `core.rs`:

- **Reboot** (x86_64): writes `0xFE` to I/O port `0x64` (keyboard controller reset)
- **Shutdown** (x86_64): writes `0x2000` to I/O port `0x604` (ACPI power off), falls back to keyboard controller reset

See [Warnings.md](../Warnings.md) warning 17 for safety rules.

## DVFS

| Function | Description |
|----------|-------------|
| `set_frequency(freq_hz)` | Sets CPU frequency |
| `current_frequency()` | Returns current frequency in Hz |

Frequency changes may be constrained by the active governor policy.

## Governor

```rust
pub enum GovernorPolicy {
    Performance,   // Maximum frequency
    Powersave,     // Minimum frequency
    OnDemand,      // Scale with load
    Conservative,  // Gradual scaling
    Schedutil,     // Scheduler-driven
    Unknown,       // Default (no detection)
}
```

| Method | Description |
|--------|-------------|
| `set_policy(p)` | Sets the active governor policy |
| `get_policy()` | Returns current policy |
| `apply()` | Applies the policy to frequency scaling |

## Idle

| Function | Description |
|----------|-------------|
| `enter_idle()` | Sleeps for 1ms (low-power idle) |
| `halt()` | Halts the CPU until next interrupt |

## Sleep states

```rust
pub enum SleepState { S0, S1, S3, S5 }
```

| State | Description |
|-------|-------------|
| S0 | Working state |
| S1 | Standby (CPU clocks stopped) |
| S3 | Suspend to RAM |
| S5 | Soft off (shutdown) |

| Function | Description |
|----------|-------------|
| `suspend(state)` | Enters the specified sleep state |
| `current_state()` | Returns current sleep state |

## Thermal

`check_temp()` reads MSR `0x19C` (IA32_THERM_STATUS) on x86_64 to check CPU temperature.
