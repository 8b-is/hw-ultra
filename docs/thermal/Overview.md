# Thermal Module

The `thermal` module manages thermal zones for temperature monitoring across the system.

## Submodules

| File | Description |
|------|-------------|
| `api.rs` | Zone registration, probing, and temperature reading |

## API

| Function | Description |
|----------|-------------|
| `probe_zones()` | Returns registered zone count |
| `zone_count()` | Number of registered zones |
| `read_thermal_zone(zone)` | Returns temperature in millidegrees Celsius |
| `register_zone(zone, temp_millideg)` | Manually registers a zone with initial temperature |

## Limits

- Maximum 8 thermal zones (`MAX_ZONES`)

## Zone mapping

| Zone | Typical source |
|------|---------------|
| 0 | CPU package |
| 1 | CPU core 0 |
| 2 | CPU core 1 |
| 3 | GPU |
| 4 | SSD / NVMe |
| 5 | Ambient |
| 6–7 | Platform-specific |

## Temperature format

All temperatures are in **millidegrees Celsius**. For example:
- `72000` = 72.000°C
- `95500` = 95.500°C

## Integration

- `power::thermal::check_temp()` reads the CPU thermal MSR
- `thermal::api` provides the user-facing zone interface
- The runtime `System` uses thermal data for governor decisions and limit enforcement
- See [Warnings.md](../Warnings.md) warning 15 for thermal safety
