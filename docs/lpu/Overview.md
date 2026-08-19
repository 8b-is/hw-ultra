# LPU Module

The `lpu` module provides Language Processing Unit support: device management, inference pipeline, model quantization, memory management, and task scheduling.

## Submodules

| File | Description |
|------|-------------|
| `device.rs` | `LpuDevice` — device initialization, mode, task submission |
| `inference.rs` | Inference pipeline for language models |
| `pipeline.rs` | Processing pipeline configuration |
| `quantization.rs` | Model quantization (FP16, INT8, INT4) |
| `memory.rs` | LPU memory management |
| `scheduler.rs` | Task scheduling |
| `lifecycle.rs` | Device lifecycle (init, reset, shutdown) |
| `drivers/` | Vendor-specific LPU drivers |

## Trait

```rust
pub trait Lpu {
    type Error;
}
```

## LpuDevice

```
LpuDevice {
    base: usize             — MMIO register base address
    initialized: AtomicBool — initialization state
    mode: AtomicUsize       — operating mode
}
```

Singleton via `Once<LpuDevice>`.

| Method | Description |
|--------|-------------|
| `init()` | Initializes the LPU hardware |
| `is_initialized()` | Whether init succeeded |
| `base_addr()` | MMIO base address |
| `set_mode(m)` | Sets operating mode |
| `get_mode()` | Current operating mode |
| `submit_task(payload, flags, align)` | Submits a task via DMA |

| Function | Description |
|----------|-------------|
| `init_with_base(base)` | Initializes singleton with base address |
| `init()` | Auto-detects and initializes |
| `get()` | Returns `Option<&'static LpuDevice>` |
| `lpu_irq_shim()` | Interrupt handler stub |
| `lpu_irq_count()` | Total IRQs received |
| `register_irq_vector(vec)` | Registers IRQ vector |

## Task submission

`submit_task(payload: &[u8], flags: u32, align: usize) -> Result<usize, &str>`

1. Allocates a `DmaBuffer` with the given alignment
2. Copies payload into the buffer
3. Submits to the DMA engine
4. Returns bytes transferred or error

## Comparison with TPU

| Aspect | TPU | LPU |
|--------|-----|-----|
| Purpose | Tensor/matrix operations | Language model inference |
| Data unit | `Tensor` | Raw byte payload |
| Pipeline | Graph compilation | Inference pipeline |
| Quantization | N/A | FP16, INT8, INT4 |
