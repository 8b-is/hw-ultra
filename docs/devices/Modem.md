# Modem Module

The `modem` module provides hardware abstraction for cellular modems, AT command protocol handling, SIM management, and network registration.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `ModemDevice` struct, generation classification |
| `hw.rs` | MMIO register access, modem enable/reset, TX/RX |
| `detection.rs` | PCI and device-tree scanning for modem hardware |
| `protocol.rs` | AT command encoding/decoding, `AtCommand`/`AtResponse` |
| `network.rs` | Network type management, registration state |
| `signal.rs` | RSSI measurement, signal quality mapping |
| `sim.rs` | SIM card state machine |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `ModemDevice`

Identifies one modem controller:
- `generation` — `ModemGeneration` (Gen2G, Gen3G, Lte, Gen5G, Unknown)
- `interface` — `ModemInterface` (Pci, Usb, Platform)
- `vendor_id`, `device_id` — PCI identification
- `reg_base` — MMIO base address
- `compatibility` — device-tree match string

### `ModemGeneration` (enum)

`Gen2G` | `Gen3G` | `Lte` | `Gen5G` | `Unknown`

Generation is inferred from vendor/device IDs and device-tree compatibility during detection.

### Network Types

`NetworkType` tracks the active radio access technology:

| Variant | Description |
|---------|-------------|
| `None` | No network |
| `Gsm` | 2G GSM |
| `Gprs` / `Edge` | 2G data |
| `Umts` | 3G |
| `Hspa` / `HspaPlus` | 3.5G / 3.75G |
| `Lte` / `LteAdvanced` | 4G / 4G+ |
| `Nr5g` | 5G NR |

### Registration and SIM

`RegistrationState`: `NotRegistered`, `RegisteredHome`, `Searching`, `Denied`, `Unknown`, `RegisteredRoaming`.

`SimState`: `Absent`, `Ready`, `PinRequired`, `PukRequired`, `Locked`, `Error`.

### Signal Quality

`SignalQuality`: `NoSignal`, `Poor`, `Fair`, `Good`, `Excellent`.

- `rssi_to_quality()` — maps raw RSSI to quality level
- `rssi_from_raw()` — converts register value to dBm
- `bars()` — returns signal bar count (0–5)
- `update_rssi()` / `current_rssi()` — atomic RSSI storage

## Detection

`detect()` scans PCI for modem-class devices (classes `0x07`, `0x02`, `0x0D`) and queries the device tree for modem compatibility strings. Vendor/device pairs are used to infer the modem generation.

## AT Command Protocol

### `AtCommand`

Encodes an AT command with a fixed-size byte buffer:
- Builder functions: `cmd_signal_quality()`, `cmd_operator()`, `cmd_sim_status()`, `cmd_imei()`
- `MAX_CMD_LEN` = 128

### `AtResponse`

Decodes modem responses:
- `from_bytes()` — parse raw bytes into a structured response
- `AtResult`: `Ok`, `Error`, `Timeout`, `Incomplete`
- `MAX_RESPONSE_LEN` = 256

## Hardware Control

All register access via `hw::read_reg()` / `hw::write_reg()`:
- `enable()` / `disable()` — power the modem
- `reset()` — hardware reset
- `set_airplane_mode()` — toggle RF
- `tx_ready()` — check transmit buffer availability
- `rx_available()` — check receive data presence
- `write_tx()` — send a byte
- `read_rx()` — receive a byte

Register offsets: `MODEM_CTRL`, `MODEM_STATUS`, `MODEM_TX_DATA`, `MODEM_RX_DATA`, and related flags.

## Network Management

- `set_network_type()` — configure preferred network technology
- `set_registration()` — update registration state
- `is_connected()` — check registration status
- `is_data_capable()` — query whether current network supports data
- `is_roaming()` — check roaming status
- `parse_creg_stat()` — parse AT+CREG response status byte

## SIM Management

- `sim_present()` — check SIM card presence
- State machine tracks SIM readiness through `SimState` variants

## Lifecycle

`lifecycle::init()` dispatches to architecture-specific initialization with the controller's MMIO base. Signals readiness via atomic XOR.
