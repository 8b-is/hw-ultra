# GPU Module

The `gpu` module provides GPU device detection, command submission, shader management, compute dispatch, memory allocation, and vendor drivers.

## Submodules

| Submodule | Description |
|-----------|-------------|
| `detection` | GPU device discovery via PCI scan, VGA probe, or callback |
| `device` | `Device` struct — MMIO access, clock, temperature, power state |
| `command` | `Command` struct — GPU command packets |
| `queue` | `Queue` — 64-entry lock-free command ring |
| `shader` | `Shader` — bytecode container |
| `pipeline` | Render/compute pipeline configuration |
| `scheduler` | GPU command scheduler |
| `compute/` | GPGPU compute dispatch and kernel management |
| `memory/` | GPU memory allocation (allocator, buffer, texture) |
| `drivers/` | Vendor drivers (AMD, NVIDIA, generic, VirtIO GPU) |
| `drm` | DRM (Direct Rendering Manager) interface |
| `hw` | Low-level hardware register access |
| `lifecycle` | Device lifecycle (init, reset, shutdown) |

## Device

```
Device {
    info: GpuDevice       — detection info (vendor, device ID, BARs)
    mmio_base: Option<usize>
}
```

| Method | Description |
|--------|-------------|
| `read_mmio32(offset)` | Reads a 32-bit GPU register |
| `write_mmio32(offset, val)` | Writes a 32-bit GPU register |
| `bar0_base()` | PCI BAR0 base address |
| `gpu_clock_mhz()` | Current GPU clock in MHz |
| `gpu_temp_millideg()` | GPU temperature in millidegrees |
| `power_state()` | Current power state |

## Command and Queue

```
Command { opcode: u32, data_ptr: *const u8, data_len: usize }
Queue { ring: [Option<Command>; 64], head: AtomicUsize, tail: AtomicUsize }
```

Queue is a lock-free ring buffer. `enqueue()` / `dequeue()` use atomic indices.

## Shader

```
Shader { bytecode_ptr: *const u8, bytecode_len: usize }
```

Created from a byte slice via `Shader::from_bytes()`. Does not own the memory.

## Compute

### Kernel

```
Kernel { id: usize, entry_point: usize, workgroup_size: usize }
```

Up to 16 kernels (`MAX_KERNELS`). Registered with `register_kernel()`, dispatched with `dispatch()`.

## Vendor drivers

| Driver | File | Description |
|--------|------|-------------|
| AMD | `drivers/amd.rs` | AMD/ATI GPU support |
| NVIDIA | `drivers/nvidia.rs` | NVIDIA GPU support |
| Generic | `drivers/generic.rs` | Fallback for unknown GPUs |
| VirtIO GPU | `drivers/virtio_gpu.rs` | VirtIO GPU (QEMU/KVM) |

## Detection callback

For platforms without PCI (ARM SoCs, embedded), detection uses a consumer-provided callback:

```rust
pub fn set_detect_gpu_fn(f: fn() -> Option<RawGpuId>)
```

The callback returns raw GPU identification data (`RawGpuId { vendor, raw_id }`). The library parses the register:
- Mali: `parse_mali_gpu_id(raw_id)` extracts product from GPU_ID register bits [31:16]
- Adreno: `parse_adreno_chip_id(raw_id)` extracts chip from RBBM_CHIP_ID bits [31:16]

28 Mali products and 14 Adreno products are identified by `mali_product_name()` / `adreno_product_name()`.

## Safety considerations

- All register access goes through MMIO shim
- GPU freeze risk if incorrect commands are submitted — see [Warnings.md](../Warnings.md) warning 6
- Command queue prevents overflow via ring buffer bounds checking
