# Debug Module

The `debug` module provides performance monitoring, hardware counters, and execution tracing.

## Submodules

| File | Description |
|------|-------------|
| `perf.rs` | Timestamp-based performance measurement |
| `counters.rs` | Hardware performance counters |
| `trace.rs` | Execution tracing |

## Performance measurement

### Perf

```
Perf {
    start_ticks: u64    — timestamp at start
    end_ticks: u64      — timestamp at stop
}
```

### API

| Function | Description |
|----------|-------------|
| `read_timestamp()` | Reads current timestamp counter |
| `start()` | Creates a `Perf` with current timestamp |
| `stop(p)` | Sets `end_ticks` on a `Perf` |
| `elapsed(p)` | Returns `end_ticks - start_ticks` |
| `sample_count()` | Number of samples taken |
| `last_start()` | Last recorded start timestamp |
| `last_end()` | Last recorded end timestamp |

### Timestamp source

| Architecture | Source | Register |
|-------------|--------|----------|
| x86_64 | TSC | `RDTSC` instruction |
| AArch64 | Virtual counter | `CNTVCT_EL0` system register |

### Usage

```
let perf = debug::perf::start();
// ... code to measure ...
let perf = debug::perf::stop(&perf);
let ticks = debug::perf::elapsed(&perf);
```

To convert ticks to time, divide by the clock source frequency.
