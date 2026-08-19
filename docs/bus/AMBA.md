# AMBA — ARM Memory-Mapped Bus

## Overview

The `amba` module provides access to ARM AMBA (Advanced Microcontroller Bus Architecture) peripherals. AMBA peripherals are identified by reading their Peripheral ID registers at standard offsets.

## Struct

```
Amba {
    base_addr: usize    — base address of the peripheral
    periph_id: u32      — combined peripheral ID
}
```

## API

| Method | Description |
|--------|-------------|
| `new(base_addr)` | Creates an AMBA handle, reads peripheral ID |
| `read_periph_id(base)` | Reads 4 PID registers at `0xFE0`–`0xFEC` |
| `read_reg(&self, offset)` | Reads a 32-bit register via MMIO shim |
| `write_reg(&self, offset, val)` | Writes a 32-bit register via MMIO shim |

## Peripheral ID layout

The peripheral ID is composed from 4 registers:

| Offset | Register | Contains |
|--------|----------|----------|
| `0xFE0` | PID0 | Part number [7:0] |
| `0xFE4` | PID1 | Designer [3:0], Part number [11:8] |
| `0xFE8` | PID2 | Revision, Designer [6:4] |
| `0xFEC` | PID3 | Customer modified, ECO revision |

## Usage

AMBA is the primary bus on ARM SoCs. Common AMBA peripherals include:
- PL011 UART
- GIC (Generic Interrupt Controller)
- SMMU (System MMU)
- Timer and watchdog units
