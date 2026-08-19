# GPU Shader

## Overview

The `Shader` struct is a lightweight container for GPU shader bytecode.

## Structure

```
Shader {
    bytecode_ptr: *const u8    — pointer to shader bytecode
    bytecode_len: usize        — bytecode length in bytes
}
```

## API

| Method | Description |
|--------|-------------|
| `from_bytes(bytes)` | Creates a shader from a byte slice |
| `len()` | Returns bytecode length |
| `is_empty()` | Returns true if bytecode is empty |

## Design notes

- `Shader` does not own the bytecode — it borrows from the caller
- The bytecode format depends on the vendor driver (AMD ISA, NVIDIA PTX, SPIR-V, etc.)
- Shader compilation from source (GLSL, HLSL, etc.) is out of scope for this crate
