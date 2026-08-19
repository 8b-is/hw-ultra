# HPET — High Precision Event Timer

## Overview

The HPET is a modern x86 timer providing high-resolution, monotonic counting. Its base address is discovered via the ACPI HPET table.

## Structure

```
Hpet {
    base_addr: usize   — MMIO register base
}
```

## API

| Method | Description |
|--------|-------------|
| `new(base_addr)` | Creates HPET handle from MMIO base |
| `read_counter()` | Reads the 64-bit main counter |
| `read_period_fs()` | Counter period in femtoseconds (10⁻¹⁵ s) |
| `enable()` | Enables the main counter |

## Register layout

All registers are memory-mapped at `base_addr`:

| Offset | Register | Description |
|--------|----------|-------------|
| `0x000` | General Capabilities | Period, revision, vendor |
| `0x010` | General Configuration | Enable bits |
| `0x0F0` | Main Counter | 64-bit counter value |
| `0x100+` | Timer N Config | Per-comparator configuration |

## Period and frequency

`read_period_fs()` returns the counter increment period in femtoseconds. To convert:

```
frequency_hz = 10^15 / period_fs
```

Typical HPET frequency: 14.318 MHz (69.84 ns period = 69_841_279 fs).

## Usage as ClockSource

The HPET is wrapped in a `ClockSource` and registered during `init_timers()`:

```
ClockSource {
    name: "hpet",
    frequency_hz: 10^15 / period_fs,
    read_fn: || hpet.read_counter(),
}
```
