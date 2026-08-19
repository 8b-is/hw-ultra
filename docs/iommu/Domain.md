# IOMMU Domains

## Overview

An IOMMU domain represents an isolated address space for a group of devices. Each domain has its own set of IOVA-to-physical mappings, preventing devices in one domain from accessing memory belonging to another.

## Structure

```
Domain {
    id: u8        — domain identifier (0–15)
    base: usize   — start of the domain's physical region
    size: usize   — size of the physical region
    flags: u8     — domain attributes
}
```

## API

| Function | Description |
|----------|-------------|
| `create_domain(base, size, flags)` | Creates a new domain, returns `Option<Domain>` |
| `domain_info(id)` | Returns domain by ID |
| `domain_count()` | Number of active domains |

## Limits

- Maximum 16 domains (`MAX_DOMAINS`)
- Domain IDs are assigned sequentially starting from 0

## Use cases

- **Device isolation**: Assign untrusted devices (e.g., NIC, GPU) to separate domains
- **DMA protection**: Prevent a compromised device from reading arbitrary physical memory
- **Virtualization**: Each VM's devices can be assigned to a separate domain
