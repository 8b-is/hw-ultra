# Storage Module

The `storage` module provides hardware abstraction for block storage controllers (AHCI, NVMe, IDE, etc.), command queuing, and partition table parsing.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `StorageController` struct, protocol classification |
| `hw.rs` | MMIO register access, AHCI/NVMe enable/reset |
| `detection.rs` | PCI and device-tree scanning for storage controllers |
| `command.rs` | `Command` struct, opcode builders (read, write, flush, trim) |
| `queue.rs` | `CommandQueue` ring buffer, submit/dequeue |
| `partition.rs` | MBR and GPT parsing, `MbrEntry`, `GptEntry` |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `StorageController`

Identifies one storage controller:
- `vendor_id`, `device_id` — PCI identification
- `protocol` — `StorageProtocol` variant
- `ports` — number of ports (AHCI) or queues
- `bar` — MMIO base address
- PCI bus/device/function or device-tree compatibility

### `StorageProtocol` (enum)

| Variant | Description |
|---------|-------------|
| `Ide` | Legacy IDE/PATA |
| `Ahci` | Serial ATA (AHCI) |
| `Nvme` | NVM Express |
| `Raid` | RAID controller |
| `Scsi` | SCSI controller |
| `Mmc` | eMMC/SD card |
| `Ufs` | Universal Flash Storage |
| `Sdhci` | SD Host Controller |
| `Unknown` | Unrecognized storage |

Protocol is inferred from PCI subclass and programming interface during detection.

### `Command`

A block I/O command:
- `opcode` — `CommandOpcode` (Read, Write, Flush, Trim, Identify)
- `lba` — logical block address
- `sector_count` — number of sectors
- `buffer_address` — DMA buffer physical address
- `flags` — command-specific flags

Builder functions: `read_cmd()`, `write_cmd()`, `flush_cmd()`, `trim_cmd()`, `identify_cmd()`.

### `CommandQueue`

Circular ring buffer for command submission:
- Fixed `MAX_QUEUE_DEPTH` = 32
- `submit()` — enqueue a command
- `dequeue()` — retrieve completed command
- `is_empty()` / `is_full()` — capacity checks
- `depth()` — current queue depth
- `inflight_count()` — commands in flight

### Partition Tables

**`MbrEntry`** — one MBR partition:
- `status` — active flag
- `partition_type` — filesystem type byte
- `start_lba`, `sector_count` — location
- `MBR_SIGNATURE` = 0xAA55

**`GptEntry`** — one GPT partition:
- `type_guid`, `unique_guid` — partition GUIDs
- `start_lba`, `end_lba` — extents
- `attributes` — partition flags
- `GPT_MAX_PARTITIONS` = 128

**`PartitionScheme`**: `Mbr`, `Gpt`, `Unknown`.

## Detection

`detect()` scans PCI for mass storage class devices (class `0x01`) and queries the device tree for storage controller compatibility strings. The protocol is classified from the PCI subclass (0x01 → IDE, 0x06 → AHCI, 0x08 → NVMe, etc.) and BAR layout.

## Hardware Control

All register access via `hw::read_reg()` / `hw::write_reg()`:

### AHCI
- `ahci_enable()` — enable AHCI mode
- `ahci_port_count()` — number of implemented ports
- `ahci_ports_implemented()` — port implementation bitmask
- `reset_controller()` — global HBA reset

### NVMe
- `nvme_enable()` / `nvme_disable()` — controller enable/disable
- `nvme_version()` — read NVMe specification version
- `nvme_ready()` — check controller ready status
- `reset_controller()` — controller reset

Register offsets defined for both AHCI (`AHCI_CAP`, `AHCI_GHC`, `AHCI_PI`, etc.) and NVMe (`NVME_CAP`, `NVME_CC`, `NVME_CSTS`, etc.).

## Partition Parsing

- `detect_scheme()` — determine MBR vs GPT from sector 0
- `parse_mbr_entry()` — extract one MBR partition entry from raw bytes
- `parse_gpt_entry()` — extract one GPT partition entry from raw bytes
- `size_bytes()` — partition size in bytes
- `size_lba()` — partition size in logical blocks

## Lifecycle

`lifecycle::init()` dispatches to architecture-specific initialization with the controller's MMIO base. Signals readiness via atomic XOR.
