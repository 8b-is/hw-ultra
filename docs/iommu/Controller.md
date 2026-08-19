# IOMMU Controller

## Overview

The `IommuController` is the central IOMMU management object. It handles hardware probing, IOVA space management, and DMA buffer mapping through the IOMMU translation tables.

## Probing

`IommuController::probe()` attempts to detect IOMMU hardware:

1. **Intel VT-d** (x86_64): Reads the DMAR (DMA Remapping) ACPI table via `acpi::find_vtd_base()`. If found, the base address points to the VT-d MMIO registers.

2. **ARM SMMU** (AArch64): Reads the DeviceTree via `devicetree::find_smmu_base()`. If found, the base address points to the SMMU global registers.

Returns `None` on platforms without IOMMU support.

## IOVA allocation

IOVA (I/O Virtual Address) space is managed with a simple bump allocator:

```
IOVA_BASE = 0x1_0000_0000  (4 GB)
IOVA_LIMIT = 0x2_0000_0000  (8 GB)
```

`alloc_iova(size, align)`:
1. Aligns `iova_next` up to `align`
2. Checks that `iova_next + size <= IOVA_LIMIT`
3. Atomically advances `iova_next`
4. Returns the allocated IOVA

This allocator does not support freeing — IOVAs are allocated monotonically. This is acceptable for the crate's use case of long-lived DMA regions.

## DMA buffer mapping

`map_dma_buffer(buf: &DmaBuffer, align) -> Option<usize>`:

1. Calls `alloc_iova(buf.len(), align)` to get an IOVA
2. Programs the IOMMU to translate that IOVA to `buf.phys_addr()`
3. Stores the mapping in the `mappings` array
4. Returns the IOVA

Devices use the returned IOVA instead of the physical address for DMA transfers.

## Translation

`translate_iova(iova) -> Option<usize>` searches the mappings array for a matching IOVA and returns the corresponding physical address.

## Limits

| Limit | Value |
|-------|-------|
| Max mappings | 64 |
| IOVA range | 4 GB (4 GB – 8 GB) |
| Max domains | 16 |
