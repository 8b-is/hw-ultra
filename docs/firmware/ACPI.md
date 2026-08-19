# ACPI — Advanced Configuration and Power Interface

## Overview

The `acpi` module discovers ACPI tables by scanning physical memory for the RSDP (Root System Description Pointer) signature, then follows the chain to RSDT/XSDT and individual tables.

## Detection flow

1. `find_rsdp()` scans `0xE0000..0x100000` for `"RSD PTR "` signature
2. Validates checksum, reads revision (1 = ACPI 1.0 / RSDT, 2+ = ACPI 2.0 / XSDT)
3. Stores addresses in `RSDP_ADDR`, `RSDT_ADDR`, `XSDT_ADDR` atomics
4. `find_table(signature)` iterates RSDT/XSDT entries to locate a specific table

## State

All state is stored in `AtomicUsize` statics:

| Static | Purpose |
|--------|---------|
| `RSDP_ADDR` | Physical address of RSDP |
| `RSDT_ADDR` | Physical address of RSDT (32-bit pointers) |
| `XSDT_ADDR` | Physical address of XSDT (64-bit pointers) |
| `ACPI_REVISION` | 1 = legacy, 2+ = extended |

## Public API

| Function | Returns | Description |
|----------|---------|-------------|
| `acpi_revision()` | `usize` | ACPI revision number |
| `is_present()` | `bool` | Whether RSDP was found |
| `find_ioapic_base()` | `Option<usize>` | I/O APIC base from MADT |
| `find_vtd_base()` | `Option<usize>` | Intel VT-d base from DMAR table |

## FadtInfo

The FADT (Fixed ACPI Description Table) is parsed into `FadtInfo`:

```
FadtInfo {
    sci_interrupt: u16       — SCI interrupt number
    smi_cmd_port: u32        — SMI command port
    acpi_enable: u8          — value to write to enable ACPI
    acpi_disable: u8         — value to write to disable ACPI
    pm1a_evt_blk: u32        — PM1a event block address
    pm1a_cnt_blk: u32        — PM1a control block address
    pm_timer_blk: u32        — PM timer block address
}
```

## Helper functions

| Function | Description |
|----------|-------------|
| `find_rsdp()` | Scans for RSDP in BIOS memory region |
| `read_bytes(paddr, buf)` | Reads physical memory via MMIO shim |
| `find_table(signature)` | Locates table by 4-byte signature in RSDT/XSDT |

## Safety considerations

- All memory reads go through the MMIO shim — no direct dereference
- RSDP scan is bounded to the standard BIOS region
- ACPI tables are read-only; the module never writes to ACPI memory
- See [Warnings.md](../Warnings.md) warning 14 for ACPI table corruption risks
