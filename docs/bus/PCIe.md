# PCIe — PCI Express

## Overview

The PCIe subsystem (`bus/pcie/`) manages PCIe link state, speed negotiation, and topology of PCIe devices.

## Submodules

| File | Description |
|------|-------------|
| `link.rs` | `Link` struct — PCIe link speed, width, state |
| `topology.rs` | `Topology` struct — collection of up to 8 links |

## Link

The `Link` struct represents one PCIe link with its negotiated parameters (speed, lane width, link state).

Re-exported as `pcie::Link`.

## Topology

```
Topology {
    links: [Option<Link>; 8]
}
```

| Method | Description |
|--------|-------------|
| `new()` | Creates empty topology |
| `add_link(idx, link)` | Adds a link at the given index (returns false if full) |
| `get_link(idx)` | Returns reference to link at index |

## Relation to PCI

PCIe extends PCI with:
- Enhanced Configuration Space (4 KB per function vs 256 bytes)
- Point-to-point serial links instead of shared bus
- Express capability structure (cap ID `0x10`)

The PCI `find_capability()` function can locate the PCIe capability to determine if a device is PCIe.
