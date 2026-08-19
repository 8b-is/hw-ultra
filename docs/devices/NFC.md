# NFC Module

The `nfc` module provides hardware abstraction for NFC controllers, RF polling, protocol handling, and NDEF tag parsing.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `NfcController` struct, chip vendor classification |
| `hw.rs` | MMIO register access, RF field control, TX/RX |
| `detection.rs` | Device-tree scanning for NFC hardware |
| `polling.rs` | Poll cycle management, target discovery |
| `protocol.rs` | NFC-A/B/F/V protocol parameters, command building |
| `tag.rs` | `TagUid`, NDEF parsing, tag type classification |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `NfcController`

Identifies one NFC controller:
- `chip` — `NfcChip` (Nxp, St, Broadcom, Ti, Unknown)
- `reg_base` — MMIO base address
- `irq` — interrupt line
- `compatibility` — device-tree match string

### `NfcChip` (enum)

`Nxp` | `St` | `Broadcom` | `Ti` | `Unknown`

Chip vendor identified from device-tree compatibility strings during detection.

### `PollState` (enum)

RF polling state machine:

`Idle` → `Polling` → `TargetFound` → `Activated` → `DataExchange`

### NFC Protocols

`NfcProtocol`: `NfcA`, `NfcB`, `NfcF`, `NfcV`, `IsoDep`, `NfcDep`.

- `protocol_bitrate()` — default bitrate for each protocol
- `modulation_code()` — modulation type code
- `tag_type_for_protocol()` — maps protocol to expected tag type

### Tag Types

`TagType`: `Type1`, `Type2`, `Type3`, `Type4`, `Type5`.

### `TagUid`

Tag unique identifier:
- `bytes` — UID byte array
- `length` — actual UID length (4, 7, or 10 bytes)
- `MAX_UID_LEN` = 10

### NDEF

`NdefMessage` — raw NDEF data container with `data` array and `length` field (`MAX_NDEF_SIZE` = 256).

`NdefRecordType`: `Text`, `Uri`, `SmartPoster`, `MimeMedia`, `AbsoluteUri`, `External`, `Unknown`.

## Detection

`detect()` scans the device tree only (no PCI — NFC controllers are typically platform devices). Vendor chip type is inferred from compatibility strings (e.g. `nxp,pn5xx`, `st,st21nfc`).

## Hardware Control

All register access via `hw::read_reg()` / `hw::write_reg()`:
- `enable()` / `disable()` — power the NFC controller
- `reset()` — hardware reset
- `rf_field_on()` / `rf_field_off()` — control the RF antenna field
- `start_poll()` / `stop_poll()` — begin/end target polling
- `target_present()` — check if a tag is in range
- `rf_active()` — query RF field state
- `tx_busy()` / `rx_ready()` — TX/RX status
- `write_tx()` / `read_rx()` — data transfer
- `set_bitrate()` / `set_modulation()` — configure RF parameters
- `field_detected()` — external field detection
- `read_target_id()` — read discovered tag UID

## Polling

- `poll_cycle()` — execute one poll iteration across configured protocols
- `set_interval_ms()` — configure polling interval
- `start()` / `stop()` — begin/end continuous polling

## Protocol Commands

- `build_sens_req()` — NFC-A SENS_REQ command
- Command constants: `SENS_REQ`, `RATS`, `SENSB_REQ`, etc.

## NDEF Parsing

- `has_ndef_magic()` — check for NDEF magic byte (`0xE1`)
- `find_ndef_tlv()` — locate the NDEF TLV block in tag data
- `ndef_record_tnf()` — extract TNF (Type Name Format) from record header
- `is_uri_record()` / `is_text_record()` — record type classification

## Lifecycle

`lifecycle::init()` dispatches to architecture-specific initialization. Signals readiness via atomic XOR.
