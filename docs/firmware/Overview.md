# Firmware Module

The `firmware` module probes and parses platform firmware interfaces to discover hardware configuration, memory maps, and device information.

## Submodules

| File | Description |
|------|-------------|
| `acpi.rs` | ACPI table discovery and parsing (RSDP, RSDT, XSDT, FADT, HPET, MADT) |
| `uefi.rs` | UEFI firmware detection, memory map, GOP, runtime services |
| `devicetree.rs` | Flattened DeviceTree (FDT) parser |
| `smbios.rs` | SMBIOS table parser (BIOS info, CPU info, memory modules) |

## Detection order

During `init_firmware()`:
1. **ACPI** — scans physical memory 0xE0000–0x100000 for RSDP signature
2. **UEFI** — checks magic at 0x80000000
3. **DeviceTree** — parses FDT blob if provided (typically on ARM)
4. **SMBIOS** — scans for SMBIOS entry point

Not all firmware types are mutually exclusive. A system may have ACPI + UEFI + SMBIOS (typical x86), or DeviceTree + SMBIOS (typical ARM server).

## Key data provided by each firmware

| Firmware | Provides |
|----------|----------|
| ACPI | I/O APIC base, VT-d base, HPET base, PM timer, SCI interrupt, power management |
| UEFI | Memory map, framebuffer (GOP), runtime services table, system table address |
| DeviceTree | Device nodes with reg/IRQ/compatible, SMMU base address |
| SMBIOS | BIOS vendor/version, CPU socket/speed/cores, memory module size/speed/type |
