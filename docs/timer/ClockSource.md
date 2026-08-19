# ClockSource

## Overview

The `ClockSource` framework provides an abstract, platform-independent time reference. Any timer hardware can be wrapped as a `ClockSource` and registered as the system's primary time base.

## Structure

```
ClockSource {
    name: &'static str       — human-readable name (e.g., "tsc", "hpet")
    frequency_hz: u64         — ticks per second
    read_fn: fn() -> u64      — function pointer to read the counter
}
```

## API

| Function | Description |
|----------|-------------|
| `register(cs)` | Registers a clock source (replaces previous) |
| `read_ticks()` | Reads from the registered clock source |
| `frequency()` | Returns the clock source frequency |
| `tsc_source()` | Returns a `ClockSource` backed by TSC (x86_64) |

## Available clock sources

| Source | Platform | Frequency | Notes |
|--------|----------|-----------|-------|
| TSC | x86_64 | CPU frequency | Highest resolution, may be unstable |
| HPET | x86_64 | ~14.3 MHz | Stable, lower resolution than TSC |
| PIT | x86_64 | ~1.19 MHz | Legacy, low resolution |
| ARM Generic Timer | AArch64 | CNTFRQ_EL0 | Standard ARM counter |

## Time conversion

To convert ticks to nanoseconds:

```
ns = ticks * 1_000_000_000 / frequency_hz
```

To convert ticks to milliseconds:

```
ms = ticks * 1_000 / frequency_hz
```
