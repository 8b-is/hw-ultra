# Security Module

The `security` module provides hardware security features: SGX enclaves, isolation domains, and speculation attack mitigations.

## Submodules

| File | Description |
|------|-------------|
| `enclaves.rs` | Intel SGX enclave management |
| `isolation.rs` | Isolation domains with configurable levels |
| `speculation.rs` | CPU speculation vulnerability mitigations |

## SGX Enclaves

```
Enclave { id: u8, base: usize, size: usize }
```

| Function | Description |
|----------|-------------|
| `sgx_supported()` | Checks CPUID leaf 7, EBX bit 2 |
| `create_enclave(base, size)` | Creates an enclave region |
| `enclave_count()` | Number of active enclaves |
| `enclave_info(id)` | Returns enclave by ID |

Maximum 8 enclaves (`MAX_ENCLAVES`).

## Isolation domains

```
IsolationDomain { id: u8, level: IsolationLevel }
```

```rust
pub enum IsolationLevel {
    None,       // No isolation
    Process,    // Process-level (address space)
    Hardware,   // Hardware-enforced (IOMMU, page tables)
    Enclave,    // SGX enclave
}
```

| Function | Description |
|----------|-------------|
| `create_domain(level)` | Creates a domain at the given level |
| `domain_count()` | Number of active domains |
| `domain_level(id)` | Returns isolation level |
| `isolate()` | Applies isolation to the current context |

Maximum 16 domains (`MAX_DOMAINS`).

## Speculation mitigations

```rust
pub enum SpeculationMitigation {
    None,       // No mitigation
    Ibrs,       // Indirect Branch Restricted Speculation
    Ibpb,       // Indirect Branch Prediction Barrier
    Stibp,      // Single Thread Indirect Branch Predictors
    Ssbd,       // Speculative Store Bypass Disable
    Retpoline,  // Return trampoline
}
```

| Function | Description |
|----------|-------------|
| `mitigations_active()` | Whether mitigations are applied |
| `active_mitigation()` | Returns current mitigation type |
| `mitigations()` | Detects and applies appropriate mitigation |

`mitigations()` selects the mitigation based on CPU vendor and available features (CPUID leaf 7).
