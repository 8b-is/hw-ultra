# TPU Tensor

## Overview

The `Tensor` struct represents a multi-dimensional array backed by a DMA-capable buffer, suitable for TPU computation.

## Structure

```
Tensor {
    dims: [usize; 4]         — dimensions (up to 4D)
    buf: Option<DmaBuffer>    — backing DMA buffer
    len: usize                — total element count
}
```

## API

| Method | Returns | Description |
|--------|---------|-------------|
| `new_from_buffer(dims, buf, len)` | `Self` | Creates tensor from existing DMA buffer |
| `as_ptr()` | `*mut u8` | Raw pointer to tensor data |
| `numel()` | `usize` | Total number of elements (`dims[0] * dims[1] * dims[2] * dims[3]`) |
| `take_buffer()` | `Option<DmaBuffer>` | Moves the DMA buffer out of the tensor |

## Dimensionality

Tensors support up to 4 dimensions. Unused dimensions are set to 1:

| Logical shape | `dims` array |
|---------------|-------------|
| Scalar | `[1, 1, 1, 1]` |
| Vector (N) | `[N, 1, 1, 1]` |
| Matrix (M×N) | `[M, N, 1, 1]` |
| 3D (M×N×K) | `[M, N, K, 1]` |
| 4D (batch) | `[B, C, H, W]` |

## Memory layout

Data is stored contiguously in the DMA buffer in row-major order. The buffer is physically contiguous, allowing direct DMA transfer to/from the TPU without scatter-gather.

## Ownership

`take_buffer()` moves the `DmaBuffer` out of the tensor, leaving `buf` as `None`. This is used when the buffer needs to be reused or freed independently.
