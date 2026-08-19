# Config Module

The `config` module manages feature flags, platform capabilities, and target platform configuration.

## Submodules

| File | Description |
|------|-------------|
| `feature.rs` | Feature flag bitmask |
| `capability.rs` | Capability registry |
| `target.rs` | Target platform selection |

## Feature flags

A 32-bit bitmask tracks detected hardware features:

| Bit | Constant | Feature |
|-----|----------|---------|
| 0 | `FEATURE_SSE` | SSE instructions |
| 1 | `FEATURE_AVX` | AVX instructions |
| 2 | `FEATURE_NEON` | ARM NEON |
| 3 | `FEATURE_IOMMU` | IOMMU present |
| 4 | `FEATURE_DMA` | DMA engine |
| 5 | `FEATURE_GPU` | GPU detected |
| 6 | `FEATURE_TPU` | TPU detected |
| 7 | `FEATURE_LPU` | LPU detected |
| 8 | `FEATURE_ACPI` | ACPI available |
| 9 | `FEATURE_UEFI` | UEFI firmware |
| 10 | `FEATURE_SMBIOS` | SMBIOS tables |
| 11 | `FEATURE_DEVICETREE` | DeviceTree present |
| 12 | `FEATURE_HPET` | HPET timer |
| 13 | `FEATURE_PCIE_ECAM` | PCIe ECAM |
| 14 | `FEATURE_VTD` | Intel VT-d |
| 15 | `FEATURE_GOP` | UEFI GOP framebuffer |

### API

| Function | Description |
|----------|-------------|
| `enable(feature)` | Sets a feature bit |
| `disable(feature)` | Clears a feature bit |
| `is_enabled(feature)` | Checks a feature bit |
| `all_enabled()` | Returns the full bitmask |
| `set_all(mask)` | Sets the entire bitmask |

## Capabilities

A key-value store for runtime capabilities (up to 16 entries):

| Function | Description |
|----------|-------------|
| `register_capability(id, value)` | Registers a capability |
| `read_capability(id)` | Reads a capability value |
| `capability_count()` | Number of registered capabilities |

## Target platform

```rust
pub enum TargetPlatform {
    Generic  = 0,
    Server   = 1,
    Embedded = 2,
    Desktop  = 3,
}
```

| Function | Description |
|----------|-------------|
| `set_platform(p)` | Sets the target platform |
| `get_platform()` | Returns current target platform |

The target platform influences default governor policy, timer selection, and power management behavior.
