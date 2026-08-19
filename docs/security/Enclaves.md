# Enclaves — Intel SGX Support

## Overview

The `enclaves` module manages Intel SGX (Software Guard Extensions) enclaves — hardware-isolated memory regions that protect code and data even from privileged software.

## SGX detection

`sgx_supported() -> bool`

Checks CPUID leaf 7, subleaf 0, EBX bit 2. Returns `true` if SGX is supported by the CPU.

## Enclave structure

```
Enclave {
    id: u8        — enclave identifier (0–7)
    base: usize   — base address of the EPC (Enclave Page Cache) region
    size: usize   — size of the enclave in bytes
}
```

## API

| Function | Description |
|----------|-------------|
| `create_enclave(base, size)` | Creates an enclave, returns `Option<Enclave>` |
| `enclave_count()` | Number of active enclaves |
| `enclave_info(id)` | Returns enclave by ID |

## Limits

- Maximum 8 enclaves (`MAX_ENCLAVES`)
- Enclave memory comes from the EPC, which is typically 128 MB or less

## Safety considerations

- SGX requires specific BIOS/firmware support
- Enclave creation may fail if EPC is exhausted
- The crate manages enclave metadata, not the enclave lifecycle (ECREATE/EINIT are CPU instructions handled at a lower level)
