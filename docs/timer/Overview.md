# Timer Module

The `timer` module manages hardware timers and clock sources: PIT (x86 legacy), HPET (x86 modern), ARM generic timer, and abstract clock sources.

## Submodules

| File | Description |
|------|-------------|
| `pit.rs` | Programmable Interval Timer (x86 legacy) |
| `hpet.rs` | High Precision Event Timer (x86 modern) |
| `arm_generic.rs` | ARM Generic Timer (AArch64) |
| `clocksource.rs` | Abstract clock source framework |
| `clockevent.rs` | Clock event device management |

## PIT — Programmable Interval Timer

```
Constants:
    PIT_FREQ = 1_193_182 Hz
    PIT_CMD  = 0x43  (command port)
    PIT_CH0  = 0x40  (channel 0 data port)
```

| Method | Description |
|--------|-------------|
| `set_frequency(hz)` | Programs PIT to fire at `hz` rate, returns actual divisor |
| `read_count()` | Reads the current PIT counter value |

## HPET — High Precision Event Timer

```
Hpet { base_addr: usize }
```

| Method | Description |
|--------|-------------|
| `new(base_addr)` | Creates HPET from MMIO base (from ACPI HPET table) |
| `read_counter()` | Reads the 64-bit main counter |
| `read_period_fs()` | Counter period in femtoseconds |
| `enable()` | Enables the HPET main counter |

## ClockSource

```
ClockSource {
    name: &'static str       — human-readable name
    frequency_hz: u64         — tick frequency
    read_fn: fn() -> u64      — function to read current ticks
}
```

| Function | Description |
|----------|-------------|
| `register(cs)` | Registers a clock source |
| `read_ticks()` | Reads from the registered clock source |
| `frequency()` | Returns the clock source frequency |
| `tsc_source()` | Returns a `ClockSource` backed by TSC |

## Timer selection

During `init_timers()`:
1. If HPET is detected (via ACPI), use HPET
2. Otherwise, fall back to PIT (x86) or ARM Generic Timer (AArch64)
3. TSC is always registered as an additional clock source on x86_64
