# GPU Device

## Overview

The `Device` struct wraps a detected `GpuDevice` with MMIO access and monitoring capabilities.

## Structure

```
Device {
    info: GpuDevice           — from detection (vendor ID, device ID, BARs)
    mmio_base: Option<usize>  — mapped MMIO base address
}
```

## Construction

`Device::new(info: GpuDevice) -> Self`

Creates a device handle. The MMIO base is resolved from the PCI BAR0 of the detected GPU.

## Register access

| Method | Description |
|--------|-------------|
| `read_mmio32(offset)` | Reads 32-bit register at `mmio_base + offset` |
| `write_mmio32(offset, val)` | Writes 32-bit register at `mmio_base + offset` |

Both methods return `None` / `false` if `mmio_base` is not set.

## Monitoring

| Method | Returns | Description |
|--------|---------|-------------|
| `bar0_base()` | `Option<usize>` | PCI BAR0 base address |
| `gpu_clock_mhz()` | `u64` | Current GPU clock frequency in MHz |
| `gpu_temp_millideg()` | `u32` | GPU temperature in millidegrees Celsius |
| `power_state()` | `u64` | Current power state value |

These monitoring functions read vendor-specific registers. The exact register offsets depend on the vendor driver in use.
