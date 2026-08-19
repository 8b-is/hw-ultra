# Input Module

The `input` module provides hardware abstraction for keyboards, touchscreens, buttons, and other human-interface devices.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `InputDevice` struct, input kind classification |
| `keyboard.rs` | Scancode translation, modifier tracking |
| `hw.rs` | PS/2 I/O ports, MMIO access, FIFO reading |
| `detection.rs` | PCI, PS/2 probe, and device-tree scanning |
| `event.rs` | `InputEvent` struct, event types |
| `button.rs` | Button state tracking, debouncing |
| `touchscreen.rs` | Multi-touch point management, calibration |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `InputDevice`

Identifies one input controller:
- `kind` — `InputKind` variant
- `interface` — `InputInterface` (Ps2, Usb, I2c, Gpio, Spi, Platform)
- `reg_base` — MMIO base address
- `irq` — interrupt line

### `InputKind` (enum)

| Variant | Description |
|---------|-------------|
| `Keyboard` | Standard keyboard |
| `Mouse` | Pointing device |
| `Touchscreen` | Touch panel |
| `Touchpad` | Laptop trackpad |
| `GameController` | Gamepad / joystick |
| `Keys` | Dedicated key array (e.g. media buttons) |
| `Unknown` | Unrecognized input device |

### `InputEvent`

A single input event:
- `event_type` — `EventType` (KeyPress, KeyRelease, TouchDown, TouchUp, TouchMove, ButtonPress, ButtonRelease, AbsoluteAxis, RelativeAxis)
- `code` — key/button/axis code
- `value` — event-specific value
- `timestamp` — capture timestamp

### `TouchPoint`

Multi-touch tracking:
- `id` — finger/contact identifier
- `x`, `y` — screen coordinates
- `pressure` — contact pressure
- `state` — `TouchState` (Released, Pressed, Moving)

## Detection

`detect()` probes PS/2 ports for keyboard/mouse presence, scans PCI for input class devices (class `0x09`), and queries the device tree for input-related compatibility strings. Returns an `InputDevice` with interface and kind inferred.

## Keyboard

Scancode-to-ASCII translation:
- `scancode_to_ascii()` — maps PS/2 set 1 scancodes to characters
- `update_modifiers()` — tracks Shift, Ctrl, Alt, Caps Lock state
- `current_modifiers()` — returns active modifier bitmask
- `is_break_code()` — detects key release codes (bit 7 set)
- `make_code()` — extracts the make code from a break code

Modifier flags: `MODIFIER_SHIFT`, `MODIFIER_CTRL`, `MODIFIER_ALT`, `MODIFIER_CAPSLOCK`.

Standard scan codes defined as constants: `KEY_ESCAPE` (0x01), `KEY_A`–`KEY_Z`, `KEY_F1`–`KEY_F12`, etc.

## Button Management

- `set_button_state()` / `get_button_state()` — per-button state via `AtomicU16`
- `pressed_mask()` — bitmask of currently pressed buttons
- `should_accept()` — debouncing logic using atomic timestamps
- `MAX_BUTTONS` = 16

## Touchscreen

- `set_screen_size()` — configure the touchscreen coordinate space
- `calibrate_point()` — apply calibration matrix to raw touch coordinates
- `distance_squared()` — compute distance between two points (for gesture detection)
- `set_active_touches()` — update multi-touch state
- `MAX_TOUCH_POINTS` = 10

## Hardware Access

PS/2 port I/O:
- `ps2_read_data()` — read from the PS/2 data port
- `ps2_send_command()` — write to the PS/2 command port
- `ps2_send_data()` — write to the PS/2 data port

MMIO-based input:
- `mmio_enable()` — enable an MMIO input controller
- `mmio_read_scancode()` — read a scancode from MMIO
- `mmio_fifo_level()` — check the input FIFO fill level

## Lifecycle

`lifecycle::init()` dispatches to architecture-specific initialization. Signals readiness via atomic XOR.
