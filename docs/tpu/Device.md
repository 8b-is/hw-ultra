# TPU Device

## Overview

The `TpuDevice` manages a single Tensor Processing Unit through MMIO registers, with DMA capabilities for data transfer.

## Singleton

```rust
static TPU_DEVICE: Once<TpuDevice>
pub fn init_with_base(base: usize)
pub fn get() -> Option<&'static TpuDevice>
```

## Structure

```
TpuDevice {
    base: usize             — MMIO register base address
    initialized: AtomicBool — true after successful init
    mode: AtomicUsize       — current operating mode
}
```

## Initialization

`TpuDevice::init()`:
1. Verifies MMIO base is accessible
2. Reads device identification registers
3. Resets the device to known state
4. Sets `initialized` to `true`

## DMA transfer

`transfer(data: &[u8], flags: u32, align: usize) -> Result<usize, &str>`

1. Allocates a `DmaBuffer` with the given alignment
2. Copies data into the buffer
3. Submits to the DMA engine (with IOMMU mapping if available)
4. Returns bytes transferred or error message

## IRQ support

| Function | Description |
|----------|-------------|
| `tpu_irq_shim()` | Interrupt handler — increments IRQ counter |
| `tpu_irq_count()` | Total IRQs received since init |
| `register_irq_vector(vec)` | Registers the TPU interrupt vector |

## Operating modes

The `mode` field controls TPU operation:
- Modes are implementation-specific (inference, training, idle, etc.)
- Changed atomically via `set_mode()` / `get_mode()`
