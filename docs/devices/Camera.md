# Camera Module

The `camera` module provides hardware abstraction for image sensors, CSI interfaces, and ISP (Image Signal Processing) pipelines.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `CameraDevice` struct, interface classification |
| `sensor.rs` | `SensorInfo`, Bayer patterns, resolution management |
| `hw.rs` | MMIO register access, CSI-2 PHY and ISP control |
| `detection.rs` | PCI and device-tree scanning for imaging hardware |
| `frame.rs` | `FrameConfig`, pixel formats, stride/buffer computation |
| `pipeline.rs` | ISP processing stages, stage sequencing |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `CameraDevice`

Identifies one camera controller:
- `interface` — `CameraInterface` (Csi, Mipi, Usb, Platform)
- `reg_base`, `reg_size` — MMIO region
- `irq` — interrupt line
- `compatibility` — device-tree match string

### `FrameConfig`

Describes one capture frame:
- `width`, `height` — resolution in pixels
- `format` — `PixelFormat` variant
- `stride` — bytes per row
- `buffer_address`, `buffer_size` — DMA target

### `SensorInfo`

Physical sensor properties:
- `sensor_type` — `Cmos` or `Ccd`
- `bayer_pattern` — `Rggb`, `Bggr`, `Grbg`, or `Gbrg`
- `max_width`, `max_height` — sensor native resolution
- `pixel_depth` — bits per pixel
- `sensor_id` — unique identifier

### `PixelFormat` (enum)

| Variant | Description |
|---------|-------------|
| `Raw8`–`Raw14` | Raw Bayer at various bit depths |
| `Yuv422` | YUV 4:2:2 packed |
| `Rgb888` | 24-bit RGB |
| `Nv12` / `Nv21` | Semi-planar YUV |
| `Jpeg` | Compressed JPEG output |

### ISP Pipeline Stages

The `PipelineStage` enum sequences ISP processing:

`Idle` → `BlackLevel` → `Demosaic` → `WhiteBalance` → `ColorCorrection` → `GammaCorrection` → `NoiseReduction` → `Sharpening` → `Output`

`advance_stage()` moves to the next stage; `reset_pipeline()` returns to `Idle`.

## Detection

`detect()` scans PCI for imaging-class devices and queries the device tree for camera/CSI compatibility strings. Returns a `CameraDevice` with the interface type inferred from bus topology.

## Hardware Control

Register access via `hw::read_reg()` / `hw::write_reg()`. Key subsystems:

### CSI-2 PHY
- `csi2_set_lanes()` — configure data lane count
- `csi2_phy_enable()` / `csi2_phy_disable()` — power the physical layer

### ISP Core
- `isp_enable()` — power on the image signal processor
- `isp_start_stream()` / `isp_stop_stream()` — begin/end capture
- `isp_set_white_balance()` — adjust white balance gains
- `isp_set_exposure()` — set exposure time
- `isp_set_gamma()` — configure gamma correction curve

Key register offsets: `CSI2_VERSION`, `ISP_CTRL`, `ISP_GAIN_R/G/B`, `ISP_EXPOSURE`, `ISP_GAMMA`.

## Frame Management

- `compute_stride()` — calculates row byte count from width and pixel format
- `compute_buffer_size()` — total frame memory requirement
- `frame_offset()` — byte offset for a given (x, y) position
- `bytes_per_pixel()` — size per pixel for each format

## Sensor Management

- `set_active_resolution()` — configure capture resolution (clamped to sensor maximum)
- `register_sensor()` — register sensor info for a sensor slot
- `megapixels()` — compute megapixel count
- `raw_line_bytes()` — bytes per raw scan line

## Lifecycle

`lifecycle::init()` dispatches to architecture-specific initialization with the controller's base address. Signals readiness via atomic XOR.
