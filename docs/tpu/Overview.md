# TPU Module

The `tpu` module provides Tensor Processing Unit support: device management, tensor operations, graph compilation and execution, DMA transfers, and runtime scheduling.

## Submodules

| File | Description |
|------|-------------|
| `device.rs` | `TpuDevice` — device initialization, mode, DMA transfer |
| `tensor.rs` | `Tensor` — multi-dimensional tensor backed by DMA buffer |
| `compiler.rs` | Graph-to-execution-plan compiler |
| `graph.rs` | Computation graph representation |
| `executor.rs` | Graph execution engine |
| `runtime.rs` | TPU runtime lifecycle |
| `memory.rs` | TPU memory management |
| `dma.rs` | TPU-specific DMA operations |
| `scheduler.rs` | Task scheduling for TPU operations |
| `lifecycle.rs` | Device lifecycle (init, reset, shutdown) |
| `drivers/` | Vendor-specific TPU drivers |

## Trait

```rust
pub trait Tpu {
    type Error;
}
```

All TPU implementations must implement this trait.

## TpuDevice

```
TpuDevice {
    base: usize             — MMIO base address
    initialized: AtomicBool — initialization state
    mode: AtomicUsize       — operating mode
}
```

Singleton via `Once<TpuDevice>`.

| Method | Description |
|--------|-------------|
| `init()` | Initializes the TPU hardware |
| `is_initialized()` | Whether init succeeded |
| `base_addr()` | MMIO base address |
| `set_mode(m)` | Sets operating mode |
| `get_mode()` | Current operating mode |
| `transfer(data, flags, align)` | DMA transfer to/from TPU |

| Function | Description |
|----------|-------------|
| `init_with_base(base)` | Initializes singleton with base address |
| `get()` | Returns `Option<&'static TpuDevice>` |
| `tpu_irq_shim()` | IRQ handler stub |
| `tpu_irq_count()` | Number of IRQs received |
| `register_irq_vector(vec)` | Registers IRQ vector |

## Tensor

```
Tensor {
    dims: [usize; 4]         — up to 4D dimensions
    buf: Option<DmaBuffer>    — backing DMA buffer
    len: usize                — total element count
}
```

| Method | Description |
|--------|-------------|
| `new_from_buffer(dims, buf, len)` | Creates tensor from DMA buffer |
| `as_ptr()` | Raw pointer to tensor data |
| `numel()` | Total number of elements |
| `take_buffer()` | Moves DMA buffer out of tensor |

## Graph compilation

### CompiledGraph

```
CompiledGraph {
    sizes: [usize; 16]       — memory size per operation
    exec_order: [u8; 16]     — execution order
    count: usize              — number of operations
    total_memory: usize       — total memory required
}
```

`compile(graph)` / `compile_graph(graph)` convert a `Graph` into an optimized execution plan.

## Safety considerations

- DMA transfers must complete before accessing tensor data
- See [Warnings.md](../Warnings.md) for DMA safety rules (warning 5)
