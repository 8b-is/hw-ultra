# CPU Affinity

Control which CPU cores a thread is allowed to run on.

## Functions

```rust
pub fn set_affinity(core_mask: usize)
pub fn get_affinity() -> usize
pub fn pin_to_core(core_id: usize)
pub fn is_pinned_to(core_id: usize) -> bool
```

## Implementation

Uses syscalls (numbers provided by the consumer via `SyscallNrTable`):
- `sched_setaffinity(pid, sizeof(mask), &mask)` — set the affinity mask
- `sched_getaffinity(pid, sizeof(mask), &mask)` — read the current mask

`pid = 0` targets the calling thread.

`sched_getaffinity` is also used by the CPU detection module (`parse_cpu_online_count()`) to determine the real number of online CPUs: a 128-byte (1024-bit) mask is populated by the kernel, and the popcount gives the number of available logical cores. This value overrides the CPUID count when higher (e.g. multi-socket systems).

## Core mask format

The mask is a bitmask where bit N corresponds to core N:
- `0b0001` = core 0 only
- `0b0011` = cores 0 and 1
- `0b1010` = cores 1 and 3
- `usize::MAX` = all cores (no restriction)

## pin_to_core

`pin_to_core(core_id)` is a convenience wrapper that sets the mask to `1 << core_id`, restricting the thread to exactly one core.

## is_pinned_to

Returns `true` if the current affinity mask has exactly one bit set and it matches `core_id`.
