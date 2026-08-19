# Boot Module

The `boot` module provides early boot information, primarily the system memory map.

## Submodules

| File | Description |
|------|-------------|
| `memmap.rs` | Total usable RAM detection |

## API

| Function | Returns | Description |
|----------|---------|-------------|
| `total_usable_ram()` | `usize` | Total usable RAM in bytes |

## Implementation

`total_usable_ram()` delegates to `memory::info::detect_memory_info()` which reads the physical memory map from:
- UEFI memory map (if available)
- ACPI memory map
- Consumer-provided detection callback (`set_detect_memory_fn`)

## Re-exports

```rust
pub use memmap::total_usable_ram;
```

## Usage

Called during `init_memory()` to determine the total physical memory available for the frame allocator. The value excludes reserved regions (firmware, MMIO, ACPI NVS).
