# Isolation Domains

## Overview

Isolation domains provide configurable levels of hardware and software isolation for code and data regions.

## IsolationLevel

```rust
pub enum IsolationLevel {
    None,       // No isolation — shared address space
    Process,    // Process-level — separate page tables
    Hardware,   // Hardware-enforced — IOMMU + page tables
    Enclave,    // SGX enclave — hardware memory encryption
}
```

## IsolationDomain

```
IsolationDomain {
    id: u8                   — domain identifier (0–15)
    level: IsolationLevel    — isolation strength
}
```

## API

| Function | Description |
|----------|-------------|
| `create_domain(level)` | Creates a new domain at the specified level |
| `domain_count()` | Number of active domains |
| `domain_level(id)` | Returns the isolation level for a domain |
| `isolate()` | Applies isolation to the current execution context |

## Limits

- Maximum 16 domains (`MAX_DOMAINS`)

## Isolation strength

| Level | Protects against | Mechanism |
|-------|-----------------|-----------|
| None | Nothing | — |
| Process | User-space faults | Page tables, ASLR |
| Hardware | DMA attacks, device compromise | IOMMU, VT-d/SMMU |
| Enclave | Privileged software, physical access | SGX encryption |
