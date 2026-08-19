# Bus Module

The `bus` module provides drivers and discovery for hardware bus architectures: PCI, PCIe, AMBA (ARM), and VirtIO.

## Submodules

| Submodule | Description |
|-----------|-------------|
| `discovery` | Device scanning and registration |
| `pci/` | PCI configuration space access, device enumeration, BAR probing, capabilities |
| `pcie/` | PCIe link and topology management |
| `amba` | ARM AMBA peripheral bus (APB/AHB) |
| `virtio` | VirtIO device detection and feature negotiation |

## Discovery

The `discovery` module maintains a fixed-size registry of up to 64 devices:

| Function | Description |
|----------|-------------|
| `scan_devices()` | Scans for PCI devices, returns count |
| `register_device(vendor, device, bus)` | Adds a device to the registry |
| `device_count()` | Number of registered devices |
| `device_vendor(index)` | Vendor ID at index |
| `device_id(index)` | Device ID at index |
| `device_class(index)` | Class code at index |

## PCI

### PciDevice

```
PciDevice {
    bus: u8
    device: u8
    function: u8
    vendor_id: u16
    device_id: u16
    class: u8
    subclass: u8
}
```

### Enumeration

`scan_all(out: &mut [PciDevice]) -> usize` scans all 256 buses × 32 devices × 8 functions, writing discovered devices into the output array.

### Configuration space

| Function | Description |
|----------|-------------|
| `read_config_u32(bus, dev, func, offset)` | Reads 32-bit config register |
| `probe_bar_size(bus, dev, func, bar_offset)` | Determines BAR size |
| `map_mmio_region(pa, size)` | Maps a physical MMIO region |

### Capabilities

| Function | Description |
|----------|-------------|
| `find_capability(bus, dev, func, cap_id)` | Finds a PCI capability by ID |
| `read_cap_u32(bus, dev, func, cap_offset, reg_offset)` | Reads a capability register |

## PCIe

### Link

Represents a PCIe link with speed and width negotiation.

### Topology

```
Topology {
    links: [Option<Link>; 8]
}
```

Manages up to 8 PCIe links with `add_link()` and `get_link()`.

## AMBA

```
Amba {
    base_addr: usize
    periph_id: u32
}
```

Reads peripheral IDs from standard AMBA offsets (`0xFE0`–`0xFEC`). Provides `read_reg()` / `write_reg()` for MMIO access.

## VirtIO

```
Virtio {
    device_id: u32
    features: usize
}
```

- `negotiate_features(host_features)` — AND of guest and host features
- `init()` — device initialization sequence
- `detect_virtio_devices()` — scans for VirtIO devices

## Safety considerations

- PCI config reads use I/O port shims (x86) or MMIO shims
- BAR probing writes to BAR registers — see [Warnings.md](../Warnings.md) warning 16
- AMBA register access goes through the MMIO shim
