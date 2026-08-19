# Power Governor

## Overview

The governor module implements CPU frequency scaling policies that automatically adjust CPU frequency based on system load.

## GovernorPolicy

```rust
pub enum GovernorPolicy {
    Performance  = 0,   // Always maximum frequency
    Powersave    = 1,   // Always minimum frequency
    OnDemand     = 2,   // Scale up quickly, scale down slowly
    Conservative = 3,   // Scale up and down gradually
    Schedutil    = 4,   // Scheduler-integrated scaling
    Unknown      = 0xFF // Default — no policy detected
}
```

`get_policy()` returns `Unknown` by default. The consumer sets the active policy via `set_policy()`.

## API

| Method | Description |
|--------|-------------|
| `set_policy(p)` | Changes the active governor policy |
| `get_policy()` | Returns the current policy |
| `apply()` | Executes the policy — adjusts frequency via DVFS |

## Policy behavior

| Policy | Scale up | Scale down | Use case |
|--------|----------|------------|----------|
| Performance | Always max | Never | Benchmarks, real-time |
| Powersave | Never | Always min | Battery, thermal emergency |
| OnDemand | Immediate at load threshold | Gradual | General purpose |
| Conservative | Gradual step-up | Gradual step-down | Laptop, quiet operation |
| Schedutil | Based on scheduler utilization | Based on scheduler utilization | Modern Linux-style |
