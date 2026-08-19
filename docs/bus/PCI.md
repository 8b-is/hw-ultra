# PCI — Peripheral Component Interconnect

## Overview

The PCI subsystem (`bus/pci/`) handles device enumeration, configuration space access, BAR probing, and capability traversal for conventional PCI devices.

## Submodules

| File | Description |
|------|-------------|
| `device.rs` | `PciDevice` struct and `scan_all()` |
| `api.rs` | `read_config_u32()`, `scan_video_devices()` |
| `config.rs` | BAR size probing and MMIO region mapping |
| `capability.rs` | PCI capability linked list traversal |

## Device enumeration

`scan_all(out: &mut [PciDevice]) -> usize`

Iterates all 256 buses × 32 devices × 8 functions (65536 BDFs). For each BDF:
1. Reads vendor ID from config offset 0x00
2. Skips if `0xFFFF` (no device)
3. Reads device ID, class, subclass
4. Writes a `PciDevice` entry to the output array

`from_bdf(bus, device, function) -> Option<PciDevice>` creates a `PciDevice` from a specific BDF if present.

## Configuration space access

All config reads use the `read_config_u32(bus, dev, func, offset)` function which:
- On x86_64: writes address to I/O port `0xCF8`, reads data from `0xCFC` (via I/O port shim)
- Access is privilege-gated by Guardian

## BAR probing

`probe_bar_size(bus, device, function, bar_offset) -> Option<usize>`

1. Saves current BAR value
2. Writes `0xFFFFFFFF` to the BAR
3. Reads back to determine size (inverted mask + 1)
4. Restores original BAR value

This is a destructive-read operation — see [Warnings.md](../Warnings.md) warning 16.

## Capability traversal

`find_capability(bus, dev, func, cap_id) -> Option<u8>`

Follows the PCI capability linked list starting from config offset `0x34`. Returns the offset of the matching capability structure. Standard capability IDs:
- `0x05` — MSI
- `0x10` — PCIe
- `0x11` — MSI-X

## MMIO region mapping

`map_mmio_region(pa: usize, size: usize) -> usize`

Maps a physical address range for MMIO access. Returns the mapped virtual address for device register access.
