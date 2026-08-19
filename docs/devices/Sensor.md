# Sensor Module

The `sensor` module provides hardware abstraction for accelerometers, gyroscopes, magnetometers, barometers, and other environmental and motion sensors.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `SensorDevice` struct, sensor kind classification |
| `hw.rs` | MMIO register access, FIFO control, sampling rate |
| `detection.rs` | Device-tree scanning with 40+ part number classifier |
| `data.rs` | `RawSample`, `CalibratedSample`, `ScalarSample`, data processing |
| `calibration.rs` | `CalibrationData`, offset/scale/cross-axis correction |
| `sampling.rs` | `SampleRate` enum, rate conversion, period computation |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `SensorDevice`

Identifies one sensor:
- `kind` — `SensorKind` variant
- `reg_base` — MMIO base address
- `irq` — interrupt line
- `compatibility` — device-tree match string

### `SensorKind` (enum)

| Variant | Description |
|---------|-------------|
| `Accelerometer` | Linear acceleration |
| `Gyroscope` | Angular velocity |
| `Magnetometer` | Magnetic field |
| `Barometer` | Atmospheric pressure |
| `Proximity` | Distance sensing |
| `Light` | Ambient light |
| `Imu` | Inertial measurement unit (combined) |
| `Temperature` | Temperature sensor |
| `Humidity` | Humidity sensor |
| `Gravity` | Gravity vector |
| `Unknown` | Unrecognized sensor |

### Sample Types

**`RawSample`** — raw ADC output: `x`, `y`, `z`, `timestamp`.

**`CalibratedSample`** — corrected values: `x`, `y`, `z`, `timestamp`.

**`ScalarSample`** — single-axis measurement: `value`, `timestamp`.

### `CalibrationData`

Per-sensor calibration parameters:
- `offset_x/y/z` — zero-point offsets
- `scale_x/y/z` — gain corrections
- Cross-axis compensation coefficients for 3-axis sensors

### `SampleRate` (enum)

`Hz1` | `Hz10` | `Hz25` | `Hz50` | `Hz100` | `Hz200` | `Hz400` | `Hz800`

## Detection

`detect()` scans the device tree only (sensors are typically I2C/SPI platform devices). The classifier recognizes 40+ part numbers from major sensor vendors (Bosch, STMicro, InvenSense, AKM, etc.) and assigns the appropriate `SensorKind`.

## Data Processing

- `sign_extend_12()` / `sign_extend_16()` — sign-extend raw ADC values
- `combine_bytes()` — merge high/low register bytes
- `scale_raw()` — apply sensitivity scaling to raw data
- `magnitude_squared()` — compute vector magnitude² (for threshold comparison)
- `axis()` — extract one axis from an `Axis` enum selection

## Calibration

- `apply_offset()` — subtract zero-point offset
- `apply_scale()` — multiply by gain correction
- `calibrate_sample()` — full pipeline: offset → scale → cross-axis correction
- `compute_offset()` — derive offset from a set of reference samples

## Sampling Control

- `rate_to_hz()` — convert `SampleRate` enum to frequency
- `rate_to_register()` — convert to hardware register value
- `set_rate()` / `current_rate_hz()` — manage active sample rate
- `period_us()` — sample period in microseconds
- `samples_for_duration_ms()` — compute sample count for a time window

## Hardware Control

All register access via `hw::read_reg()` / `hw::write_reg()`:
- `enable()` / `disable()` — power the sensor
- `reset()` — hardware reset
- `set_continuous()` — enable continuous measurement mode
- `set_single_shot()` — enable single-shot mode
- `enable_fifo()` — turn on hardware FIFO buffering
- `set_sample_rate()` — program the ODR register
- `set_offsets()` / `set_scale()` — write calibration to hardware
- `fifo_level()` — current FIFO fill count
- `read_data()` / `read_data_high()` — read measurement registers
- `sensor_id()` — read the WHO_AM_I register

## Lifecycle

`lifecycle::init()` dispatches to architecture-specific initialization. Signals readiness via atomic XOR.
