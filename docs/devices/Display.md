# Display Module

The `display` module provides hardware abstraction for display controllers, framebuffers, timing generators, and backlight control.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `DisplayController` struct, display type classification |
| `framebuffer.rs` | Framebuffer setup, pixel writing, clearing |
| `hw.rs` | MMIO register access, enable/disable control |
| `detection.rs` | PCI and device-tree scanning for display hardware |
| `timing.rs` | `TimingParams`, standard modes, pixel clock computation |
| `backlight.rs` | Brightness control, enable/disable |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `DisplayController`

Represents one display output:
- `vendor_id`, `device_id` — PCI identification
- `display_type` — `DisplayType` variant
- `fb_bar` — framebuffer MMIO base address
- PCI bus/device/function or device-tree compatibility string

### `DisplayType` (enum)

| Variant | Description |
|---------|-------------|
| `Vga` | Legacy VGA |
| `Xga` | Extended Graphics Array |
| `ThreeD` | 3D controller (co-processor) |
| `Dsi` | MIPI DSI panel |
| `Hdmi` | HDMI output |
| `Dp` | DisplayPort |
| `Lvds` | Low-Voltage Differential Signaling |
| `Edp` | Embedded DisplayPort |
| `Lcdc` | LCD controller (embedded) |
| `Unknown` | Unrecognized display class |

### `TimingParams`

Complete mode timing:
- `pixel_clock_khz` — pixel clock frequency
- `h_active`, `h_front_porch`, `h_sync`, `h_back_porch` — horizontal timing
- `v_active`, `v_front_porch`, `v_sync`, `v_back_porch` — vertical timing
- `refresh_rate_hz` — target refresh rate

### `PixelFormat` (enum)

Supported framebuffer formats: `Rgb565`, `Rgb888`, `Rgba8888`, `Bgra8888`, `Xrgb8888`, `Yuv420`, `Yuv422`.

## Detection

`detect()` scans PCI for display class devices (class `0x03`) and queries the device tree for display/framebuffer/panel compatibility strings. Returns a `DisplayController` with the display type inferred from PCI subclass and vendor data.

## Framebuffer

- `set_framebuffer()` — configure the active framebuffer address and dimensions
- `framebuffer_base()` / `framebuffer_size()` — query current framebuffer region
- `stride()` — bytes per scanline
- `write_pixel()` — write a single pixel at (x, y)
- `clear()` — fill the entire framebuffer with a solid color

## Timing

Predefined standard modes:
- `TIMING_640X480_60` — VGA 640×480 @ 60 Hz
- `TIMING_1280X720_60` — HD 1280×720 @ 60 Hz
- `TIMING_1920X1080_60` — Full HD 1920×1080 @ 60 Hz

Helper functions:
- `apply_timing()` — program timing registers
- `h_total()` / `v_total()` — total pixel counts including blanking
- `compute_pixel_clock()` — derive pixel clock from resolution and refresh rate

## Backlight

- `set_brightness()` — set backlight intensity (0–100 or 0–255 depending on hardware)
- `get_brightness()` — read current brightness
- `enable()` / `disable()` — power the backlight on or off

## Hardware Control

All register access via `hw::read_reg()` / `hw::write_reg()` on the controller's MMIO region.
- `enable_display()` / `disable_display()` — power the display pipeline
- `is_enabled()` — query display power state

## Lifecycle

`lifecycle::init()` dispatches to `x86_64::display::init_display()` or `aarch64::display::init_display()` with the controller's MMIO base. State transitions signaled via atomic XOR.
