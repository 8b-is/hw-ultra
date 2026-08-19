# IOMMU Module

The `iommu` module provides I/O memory management for DMA isolation and address translation. It supports Intel VT-d (x86_64) and ARM SMMU (AArch64).

## Submodules

| File | Description |
|------|-------------|
| `controller.rs` | `IommuController` — probing, IOVA allocation, mapping |
| `domain.rs` | IOMMU domains for device isolation |
| `mapping.rs` | IOVA-to-physical address mappings |
| `intel_vtd.rs` | Intel VT-d specific logic |
| `arm_smmu.rs` | ARM SMMU specific logic |

## IommuController

```
IommuController {
    base: usize                   — MMIO base address
    enabled: AtomicUsize          — 0 = disabled, 1 = enabled
    mappings: [(usize, usize); 64]  — (IOVA, physical) pairs
    map_count: AtomicUsize
    iova_next: AtomicUsize        — next IOVA to allocate
}
```

Singleton via `Once<IommuController>`.

### IOVA range

| Constant | Value | Description |
|----------|-------|-------------|
| `IOVA_BASE` | `0x1_0000_0000` | Start of IOVA space (4 GB) |
| `IOVA_LIMIT` | `0x2_0000_0000` | End of IOVA space (8 GB) |

IOVA allocation is a bump allocator within this range.

### Probing

`IommuController::probe() -> Option<Self>`

1. Checks ACPI DMAR table for VT-d base address (`find_vtd_base()`)
2. If not found, checks DeviceTree for SMMU base (`find_smmu_base()`)
3. Returns `None` if no IOMMU hardware is detected

### API

| Method | Description |
|--------|-------------|
| `init()` | Enables the IOMMU hardware |
| `is_enabled()` | Whether the IOMMU is active |
| `alloc_iova(size, align)` | Allocates an IOVA region |
| `map_dma_buffer(buf, align)` | Maps a `DmaBuffer` and returns its IOVA |
| `unmap_iova(iova)` | Removes an IOVA mapping |
| `translate_iova(iova)` | Returns physical address for an IOVA |

## Domains

```
Domain {
    id: u8
    base: usize
    size: usize
    flags: u8
}
```

Up to 16 domains (`MAX_DOMAINS`). Each domain isolates a set of devices with their own IOVA-to-physical mappings.

| Function | Description |
|----------|-------------|
| `create_domain(base, size, flags)` | Creates a new domain |
| `domain_info(id)` | Returns domain by ID |
| `domain_count()` | Number of active domains |

## Mapping table

Up to 64 IOVA-to-physical mappings (`MAX_IOMMU_MAPPINGS`).

| Function | Description |
|----------|-------------|
| `add_mapping(iova, phys, size)` | Adds a mapping |
| `translate(iova)` | Translates IOVA to physical |
| `mapping_count()` | Number of active mappings |

## Safety considerations

- Without IOMMU, DMA devices can access all physical memory — see [Warnings.md](../Warnings.md) warning 5
- IOMMU must be initialized before any DMA transfers
- Unmapping an IOVA while a transfer is in flight causes data loss or corruption
