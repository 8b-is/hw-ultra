# USB Module

The `usb` module provides hardware abstraction for USB host controllers, device enumeration, descriptor parsing, hub management, and data transfers.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `UsbController` struct, host controller classification |
| `hw.rs` | MMIO register access, xHCI reset/start/stop, port control |
| `detection.rs` | PCI and device-tree scanning for USB controllers |
| `descriptor.rs` | USB descriptor parsing (device, config, interface) |
| `endpoint.rs` | `EndpointDescriptor` parsing, endpoint types |
| `hub.rs` | Hub port management, enumeration, port state |
| `transfer.rs` | `Transfer` struct, control/bulk/interrupt transfers |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `UsbController`

Identifies one USB host controller:
- `vendor_id`, `device_id` — PCI identification
- `host_controller` — `HostController` variant
- `speed` — `UsbSpeed` (LowFull, High, Super, SuperPlus)
- `bar` — MMIO base address
- PCI bus/device/function or device-tree compatibility

### `HostController` (enum)

| Variant | Description |
|---------|-------------|
| `Uhci` | USB 1.x (Intel) |
| `Ohci` | USB 1.x (Compaq/Microsoft) |
| `Ehci` | USB 2.0 |
| `Xhci` | USB 3.x |
| `Dwc2` | DesignWare Core 2 (embedded) |
| `Dwc3` | DesignWare Core 3 (embedded) |
| `Musb` | Mentor Graphics USB (embedded) |
| `Unknown` | Unrecognized controller |

### USB Descriptors

**`DeviceDescriptor`**:
- USB version, device class/subclass/protocol
- `vendor_id`, `product_id`
- Number of configurations
- Helpers: `is_hub()`, `is_hid()`, `is_mass_storage()`, `is_vendor_specific()`

**`ConfigDescriptor`**:
- Total length, interface count
- Attributes: `self_powered()`, `remote_wakeup()`
- `max_power_ma()` — maximum power consumption

**`InterfaceDescriptor`**:
- Interface number, alternate setting
- Endpoint count, class/subclass/protocol

**`EndpointDescriptor`**:
- `address` — endpoint number and direction
- `endpoint_type` — `EndpointType` (Control, Bulk, Interrupt, Isochronous)
- `direction` — `Direction` (In, Out)
- `max_packet_size`, `interval`
- `number()` — extract endpoint number from address

All descriptors implement `from_bytes()` / `from_raw()` for parsing raw USB descriptor data.

### `SetupPacket`

USB control transfer setup stage:
- `request_type`, `request`, `value`, `index`, `length`
- Builders: `get_descriptor()`, `set_address()`, `set_configuration()`
- `to_bytes()` — serialize to 8-byte setup packet
- `direction()` — `SetupDirection` (HostToDevice, DeviceToHost)

### `Transfer`

A USB data transfer:
- `transfer_type` — `TransferType` (Control, Bulk, Interrupt, Isochronous)
- `endpoint` — target endpoint
- `buffer_address`, `buffer_length` — data buffer
- `status` — `TransferStatus` (Pending, Active, Completed, Stalled, Error)
- `actual_length` — bytes actually transferred
- Constructors: `control()`, `bulk()`, `interrupt()`
- `is_complete()` — check transfer completion

### Port and Hub

`PortState`: `Disconnected`, `Connected`, `Enabled`, `Suspended`, `Error`.

`HubSpeed`: `Full`, `High`, `Super`.

Hub management:
- `set_port_count()` / `port_count()` — configure downstream ports
- `set_port_state()` / `get_port_state()` — per-port state tracking
- `enumerate_ports()` — scan all ports for connected devices
- `reset_port()` — reset a specific downstream port
- `MAX_PORTS` = 16

## Detection

`detect()` scans PCI for USB controller class devices (class `0x0C`, subclass `0x03`) and queries the device tree for USB controller compatibility strings. The host controller type is inferred from PCI programming interface (0x00 → UHCI, 0x10 → OHCI, 0x20 → EHCI, 0x30 → xHCI) or device-tree match.

## Hardware Control (xHCI)

All register access via `hw::read_reg()` / `hw::write_reg()`:
- `cap_length()` — capability register length
- `hci_version()` — xHCI specification version
- `max_ports()` — maximum root hub ports
- `max_device_slots()` — maximum device slots
- `reset_controller()` — controller reset
- `start()` / `stop()` — run/halt the controller
- `port_connected()` — check port connection status
- `port_speed()` — read port speed
- `port_reset()` — reset a root hub port

Descriptor type constants: `DESC_DEVICE` (0x01), `DESC_CONFIG` (0x02), `DESC_INTERFACE` (0x04), `DESC_ENDPOINT` (0x05), `DESC_HID` (0x21), `DESC_HUB` (0x29), etc.

## Lifecycle

`lifecycle::init()` dispatches to architecture-specific initialization with the controller's MMIO base. Signals readiness via atomic XOR.
