# Discovery Module

The `discovery` module maintains a central registry of all detected hardware devices.

## Submodules

| File | Description |
|------|-------------|
| `registry.rs` | Device registry with type classification |

## DeviceType

```rust
pub enum DeviceType {
    Unknown,
    Cpu,
    Gpu,
    Tpu,
    Lpu,
    Network,
    Storage,
    Iommu,
}
```

## DiscoveredDevice

```
DiscoveredDevice {
    id: u8              — unique device identifier
    dev_type: DeviceType — device classification
    base_addr: usize    — MMIO base address
}
```

## API

| Function | Description |
|----------|-------------|
| `register_device(dev_type, base_addr)` | Registers a device, returns `Option<DiscoveredDevice>` |
| `device_count()` | Number of registered devices |
| `device_info(id)` | Returns device by ID |
| `discover_all()` | Runs full device discovery (PCI scan + accelerator detection) |

## Limits

- Maximum 32 devices (`MAX_DISCOVERED`)

## Discovery flow

`discover_all()` aggregates devices from:
1. PCI bus scan — classifies by PCI class code
2. GPU detection — registers GPUs found via PCI or callback
3. TPU/LPU detection — registers accelerators found via PCI or DeviceTree
4. IOMMU detection — registers IOMMU from ACPI/DeviceTree

## Integration

The discovery registry is populated during `init_discovery()` (phase 12 of init). Other modules query it to find devices by type or enumerate all hardware.
