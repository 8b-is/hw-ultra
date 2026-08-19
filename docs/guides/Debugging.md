# Debugging Guide

## Build issues

### "can't find crate for `std`"

The crate is `#![no_std]`. Ensure you're not importing anything from `std`.

### Clippy warnings about `/// # Safety`

Every `unsafe fn` must have a `/// # Safety` doc comment. This is enforced by clippy.

## Runtime issues

### Hardware access returns `None`

1. **Shims not registered** — call `init::init()` or `System::init()` first
2. **Guard limit exceeded** — `CPU_GUARD` (1024) or `MMIO_GUARD` (10000) exhausted
3. **Architecture mismatch** — CPUID on AArch64, MIDR on x86_64

### Init hangs

Check the init phase order. Each phase depends on earlier phases. If a phase hangs:
1. Identify which phase (add `debug::perf::start()` around each)
2. Check if the dependency phase completed
3. Common cause: MMIO read to unmapped address

### DMA transfer fails

1. Check DMA engine is initialized (`dma::get().is_some()`)
2. Check buffer alignment matches device requirements
3. If IOMMU is present, check IOVA allocation succeeded
4. Check DMA ring is not full (127 max in-flight)

### Interrupts not firing

1. Check controller is initialized (`init_interrupts()` ran)
2. Check handler is registered (`interrupt::register()`)
3. Check IRQ is enabled (`Controller::enable_irq()`)
4. Check EOI is being sent (missing EOI blocks further interrupts)

## Performance measurement

```rust
use hardware::sys::debug::perf;

let p = perf::start();
// ... code to measure ...
let p = perf::stop(&p);
let ticks = perf::elapsed(&p);
// Convert: ns = ticks * 1_000_000_000 / clocksource::frequency()
```

## Examining system state

```rust
let system = System::init();
let status = system.status();
// status.arch, gpu_present, tpu_present, lpu_present
// status.dma_allocated, mem_allocated, irq_registered
// status.total_ram, limits_enforced
```

## Test failures

### `stress_sequential` fails

The stress test has 7 phases with thresholds:
1. CPU > 80% — if failing, check CPU detection
2. RAM > 80% — if failing, check memory detection
3. Disk I/O — if failing, check disk access
4. Cache > 70% — if failing, check cache detection
5. Context switch > 80% — if failing, check context save/restore
6. Swap > 50% — if failing, check swap detection
7. GPU DRM — if failing, check DRM access

### `detect_*` tests fail

Each `detect_*` test initializes and queries one subsystem. Check:
- Shims are registered (automatic in test setup)
- The subsystem's init function runs without error
- The detection result matches expected hardware
