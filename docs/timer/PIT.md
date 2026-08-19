# PIT — Programmable Interval Timer

## Overview

The PIT (Intel 8253/8254) is the legacy x86 timer. It generates periodic interrupts at a programmable frequency.

## Constants

| Name | Value | Description |
|------|-------|-------------|
| `PIT_FREQ` | `1_193_182` Hz | Base oscillator frequency |
| `PIT_CMD` | `0x43` | Command register I/O port |
| `PIT_CH0` | `0x40` | Channel 0 data I/O port |

## API

| Method | Description |
|--------|-------------|
| `set_frequency(hz: u32) -> u32` | Programs PIT divisor, returns actual divisor |
| `read_count() -> u16` | Reads current counter value from channel 0 |

## Frequency calculation

The PIT generates interrupts at `PIT_FREQ / divisor` Hz. To get a desired frequency:

```
divisor = PIT_FREQ / desired_hz
actual_hz = PIT_FREQ / divisor
```

For example, 1000 Hz (1ms tick):
- divisor = 1_193_182 / 1000 = 1193
- actual = 1_193_182 / 1193 ≈ 1000.15 Hz

## Typical usage

The PIT is used as a fallback timer when HPET is not available. It provides IRQ 0 (vector `0x20`) for the system timer tick.

## Limitations

- 16-bit divisor limits frequency range to ~18.2 Hz – 1.193 MHz
- Single-channel mode (channel 0 only)
- Lower precision than HPET or TSC
