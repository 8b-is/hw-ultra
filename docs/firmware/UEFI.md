# UEFI — Unified Extensible Firmware Interface

## Overview

The `uefi` module parses UEFI firmware data: system table, memory map, GOP framebuffer, and runtime services pointers. It does not call UEFI services directly — it stores and interprets data passed in from a bootloader.

## Key structures

### UefiMemoryType

Enum with 15 variants mapping to UEFI memory descriptor types:

| Value | Name | Description |
|-------|------|-------------|
| 0 | Reserved | Not usable |
| 1 | LoaderCode | UEFI application code |
| 2 | LoaderData | UEFI application data |
| 3 | BootServicesCode | Reclaimable after ExitBootServices |
| 4 | BootServicesData | Reclaimable after ExitBootServices |
| 5 | RuntimeServicesCode | Must be preserved |
| 6 | RuntimeServicesData | Must be preserved |
| 7 | Conventional | Free memory |
| 8 | Unusable | Error-containing memory |
| 9 | AcpiReclaim | ACPI tables (reclaimable) |
| 10 | AcpiNvs | ACPI NVS (must be preserved) |
| 11 | Mmio | Memory-mapped I/O |
| 12 | MmioPortSpace | MMIO port space |
| 13 | PalCode | Processor-specific |
| 14 | Persistent | Persistent memory (NVDIMM) |

### UefiMemoryDescriptor

```
UefiMemoryDescriptor {
    memory_type: u32
    padding: u32
    physical_start: u64
    virtual_start: u64
    number_of_pages: u64
    attribute: u64
}
```

Each descriptor covers `number_of_pages * 4096` bytes starting at `physical_start`.

### UefiInfo

```
UefiInfo {
    system_table: usize
    revision_major: u8
    revision_minor: u8
    memory_map_base: usize
    memory_map_size: usize
}
```

### GopInfo

```
GopInfo {
    framebuffer_base: usize
    framebuffer_size: usize
    horizontal_resolution: u32
    vertical_resolution: u32
    pixels_per_scan_line: u32
}
```

### RuntimeServicesTable

```
RuntimeServicesTable {
    address: usize
    get_time: usize
    set_time: usize
    get_variable: usize
    set_variable: usize
    reset_system: usize
}
```

Stores function pointer addresses from the UEFI Runtime Services table. These are not called directly but stored for potential invocation by OS-level code.

## Public API

| Function | Returns | Description |
|----------|---------|-------------|
| `parse_uefi()` | — | Detects UEFI presence and parses system table |
| `set_memory_map(base, size, desc_size)` | — | Stores memory map from bootloader |
| `memory_map_descriptor_count()` | `usize` | Number of descriptors in the map |
| `read_memory_descriptor(index)` | `Option<UefiMemoryDescriptor>` | Reads one descriptor |
| `total_conventional_memory()` | `u64` | Sum of conventional memory in bytes |
| `set_runtime_services(addr)` | — | Stores runtime services table address |
| `runtime_services()` | `Option<RuntimeServicesTable>` | Parsed runtime services table |
| `set_gop(fb_base, fb_size, h_res, v_res, stride)` | — | Stores GOP framebuffer info |
| `gop_info()` | `Option<GopInfo>` | GOP framebuffer info |
| `uefi_info()` | `UefiInfo` | General UEFI info |

## Safety considerations

- The module only reads memory via MMIO shims
- Memory map data must be set before descriptors are read
- GOP framebuffer address is stored but never accessed (display module handles rendering)
- See [Warnings.md](../Warnings.md) warning 19 for UEFI-related risks
