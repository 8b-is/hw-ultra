# DeviceTree — Flattened Device Tree Parser

## Overview

The `devicetree` module parses a Flattened Device Tree (FDT) blob to discover devices on platforms that use DeviceTree (primarily ARM/AArch64). It extracts device nodes, register ranges, IRQs, and compatible strings.

## FDT format constants

| Constant | Value | Meaning |
|----------|-------|---------|
| `FDT_MAGIC` | `0xD00DFEED` | FDT blob magic number |
| `FDT_BEGIN_NODE` | 1 | Start of a device node |
| `FDT_END_NODE` | 2 | End of a device node |
| `FDT_PROP` | 3 | Property within a node |
| `FDT_NOP` | 4 | No operation (padding) |
| `FDT_END` | 9 | End of structure block |

## Structures

### FdtHeader

```
FdtHeader {
    magic: u32               — must be 0xD00DFEED
    totalsize: u32           — total blob size in bytes
    off_dt_struct: u32       — offset to structure block
    off_dt_strings: u32      — offset to strings block
    off_mem_rsvmap: u32      — offset to memory reservation map
    version: u32             — FDT format version
    last_comp_version: u32   — lowest compatible version
    boot_cpuid_phys: u32     — boot CPU physical ID
    size_dt_strings: u32     — strings block size
    size_dt_struct: u32      — structure block size
}
```

### FdtNode

```
FdtNode {
    name: [u8; 64]     — node name (null-terminated)
    name_len: usize     — actual name length
    depth: usize        — nesting depth in tree
    offset: usize       — byte offset within structure block
}
```

### DtDeviceEntry

```
DtDeviceEntry {
    name: [u8; 64]          — device name
    name_len: usize
    reg_base: u64           — register base address
    reg_size: u64           — register region size
    irq: u32                — interrupt number
    compatible: [u8; 128]   — compatible string (e.g., "arm,smmu-v3")
    compatible_len: usize
}
```

## Public API

| Function | Returns | Description |
|----------|---------|-------------|
| `parse_fdt_header(blob)` | `Option<FdtHeader>` | Validates magic and parses header |
| `enumerate_nodes(blob, out)` | `usize` | Fills array of `FdtNode`, returns count |
| `enumerate_devices(blob, out)` | `usize` | Fills array of `DtDeviceEntry`, returns count |
| `find_smmu_base()` | `Option<usize>` | Locates ARM SMMU base address |
| `find_device_by_compatible(needle)` | `Option<(usize, usize, u32)>` | Searches FDT for device by compatible string, returns `(reg_base, reg_size, irq)` |

### `find_device_by_compatible`

Searches the FDT blob for a device whose `compatible` property contains the given byte-slice needle (e.g., `b"arm,gpu"`, `b"arm,lpu"`). Uses `contains_bytes()` for substring matching.

Returns `None` if:
- No FDT is present
- No device matches the needle
- The matching device has `reg_base == 0`

Used by all AArch64 lifecycle modules to discover MMIO base addresses, region sizes, and interrupt numbers at runtime instead of hardcoding them.

## Fixed-size limits

- `MAX_FDT_NODES` — maximum nodes enumerated in one pass
- `MAX_FDT_DEVICES` — maximum device entries extracted

All output arrays are caller-provided and stack-allocated (no heap).

## Safety considerations

- Blob is accessed as a byte slice — no pointer dereference
- Magic validation prevents parsing garbage data
- All string copies are bounded by fixed-size arrays (64 / 128 bytes)
- See [Warnings.md](../Warnings.md) warning 18 for DeviceTree corruption risks
