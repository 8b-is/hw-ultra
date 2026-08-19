# LPU Device

## Overview

The `LpuDevice` manages a Language Processing Unit through MMIO registers with DMA-based task submission. Its API mirrors `TpuDevice` but is specialized for inference workloads.

## Singleton

```rust
static LPU_DEVICE: Once<LpuDevice>
pub fn init_with_base(base: usize)
pub fn init()
pub fn get() -> Option<&'static LpuDevice>
```

## Structure

```
LpuDevice {
    base: usize             — MMIO register base address
    initialized: AtomicBool — true after successful init
    mode: AtomicUsize       — current operating mode
}
```

## Initialization

Two init paths:
- `init_with_base(base)` — explicit base address (from DeviceTree or PCI BAR)
- `init()` — auto-detection via PCI scan or device discovery

## Task submission

`submit_task(payload: &[u8], flags: u32, align: usize) -> Result<usize, &str>`

Submits an inference task to the LPU:
1. Allocates aligned DMA buffer
2. Copies payload data
3. Submits via DMA engine (with IOMMU if available)
4. Returns bytes transferred

## IRQ support

| Function | Description |
|----------|-------------|
| `lpu_irq_shim()` | Interrupt handler — increments counter |
| `lpu_irq_count()` | Total IRQs since init |
| `register_irq_vector(vec)` | Assigns interrupt vector |

## Operating modes

Modes control the LPU's behavior:
- Mode switching is atomic
- Mode values are implementation-defined (idle, inference, batch, etc.)
