# SMBIOS — System Management BIOS

## Overview

The `smbios` module parses SMBIOS tables to extract BIOS, CPU, and memory module information. SMBIOS data is typically placed by the firmware at a well-known address and provides detailed hardware inventory.

## Structures

### SmbiosHeader

```
SmbiosHeader {
    entry_type: u8    — table type (0=BIOS, 4=Processor, 17=Memory)
    length: u8        — formatted area length
    handle: u16       — unique handle for cross-references
}
```

### SmbiosInfo

```
SmbiosInfo {
    base: usize           — base address of SMBIOS table area
    version_major: u8     — SMBIOS major version
    version_minor: u8     — SMBIOS minor version
    table_count: usize    — number of tables found
}
```

### BiosInfo (Type 0)

```
BiosInfo {
    vendor_str_idx: u8
    version_str_idx: u8
    release_date_str_idx: u8
    rom_size_64k: u8         — ROM size in 64 KB units
    major_release: u8
    minor_release: u8
}
```

### CpuInfo (Type 4)

```
CpuInfo {
    socket_str_idx: u8
    processor_type: u8
    processor_family: u8
    processor_id: u64
    max_speed_mhz: u16
    current_speed_mhz: u16
    core_count: u8
    thread_count: u8
    voltage: u8
}
```

### MemModuleInfo (Type 17)

```
MemModuleInfo {
    locator_str_idx: u8
    size_mb: u16
    speed_mhz: u16
    memory_type: u8
    form_factor: u8
    data_width: u16
    rank: u8
}
```

## Public API

| Function | Returns | Description |
|----------|---------|-------------|
| `parse_smbios()` | — | Scans for SMBIOS entry point and parses tables |
| `smbios_info()` | `SmbiosInfo` | General SMBIOS metadata |
| `read_cpu_info(index)` | `Option<CpuInfo>` | CPU info for socket at `index` |
| `read_mem_module(index)` | `Option<MemModuleInfo>` | Memory module at `index` |

## Table types parsed

| Type | Name | Struct |
|------|------|--------|
| 0 | BIOS Information | `BiosInfo` |
| 4 | Processor Information | `CpuInfo` |
| 17 | Memory Device | `MemModuleInfo` |

## Safety considerations

- All reads go through MMIO shim — no raw pointer dereference
- String indices reference the unformatted string area following each table
- Bounds checked against table length before field access
