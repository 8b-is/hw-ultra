# GPU Compute

## Overview

The `gpu/compute/` submodule provides GPGPU (General Purpose GPU) compute capabilities: kernel registration, workgroup configuration, and dispatch.

## Submodules

| File | Description |
|------|-------------|
| `kernel.rs` | Kernel struct, registration, dispatch |
| `dispatch.rs` | Kernel dispatch entry point |

## Kernel

```
Kernel {
    id: usize              — unique kernel identifier
    entry_point: usize     — address of the kernel entry
    workgroup_size: usize  — threads per workgroup
}
```

## API

| Function | Description |
|----------|-------------|
| `register_kernel(entry_point, workgroup_size)` | Registers a kernel, returns `Option<Kernel>` |
| `kernel_info(id)` | Returns kernel by ID |
| `kernel_count()` | Number of registered kernels |
| `dispatch(kernel, num_groups)` | Dispatches `num_groups` workgroups |
| `dispatch_kernel()` | Top-level dispatch entry |

## Limits

- Maximum 16 kernels (`MAX_KERNELS`)

## Dispatch model

Kernels are dispatched in a 1D grid of workgroups. Each workgroup contains `workgroup_size` threads executing in SIMT (Single Instruction, Multiple Threads) fashion.
