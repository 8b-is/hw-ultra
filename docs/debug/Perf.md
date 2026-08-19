# Debug — Performance Monitoring

## Overview

The `perf` module provides lightweight, low-overhead timestamping for measuring code execution time using hardware counters.

## Perf struct

```
Perf {
    start_ticks: u64    — counter value at measurement start
    end_ticks: u64      — counter value at measurement stop
}
```

## API

| Function | Returns | Description |
|----------|---------|-------------|
| `read_timestamp()` | `u64` | Reads hardware timestamp counter directly |
| `start()` | `Perf` | Captures start timestamp |
| `stop(p: &Perf)` | `Perf` | Captures end timestamp, returns updated Perf |
| `elapsed(p: &Perf)` | `u64` | Computes `end_ticks - start_ticks` |
| `sample_count()` | `usize` | Total measurements taken |
| `last_start()` | `usize` | Most recent start tick |
| `last_end()` | `usize` | Most recent end tick |

## Platform implementation

### x86_64

`read_timestamp()` executes `RDTSC` (Read Time-Stamp Counter), which returns the CPU's 64-bit cycle counter. Resolution depends on CPU frequency (typically ~0.3ns at 3 GHz).

For serialized reads (avoiding out-of-order measurement), `RDTSCP` or `LFENCE; RDTSC` sequences may be needed.

### AArch64

`read_timestamp()` reads `CNTVCT_EL0` (Counter-timer Virtual Count), which runs at the frequency reported by `CNTFRQ_EL0` (typically 24–100 MHz).

## Accuracy notes

- TSC frequency may vary between cores or power states on older CPUs
- Modern CPUs with `constant_tsc` and `nonstop_tsc` features provide invariant TSC
- Minimum measurable duration: ~10–50 cycles (overhead of the measurement itself)
