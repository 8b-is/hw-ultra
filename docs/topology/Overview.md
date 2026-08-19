# Topology Module

The `topology` module detects and models the system's physical topology: sockets, cores, interconnects, and NUMA nodes.

## Submodules

| File | Description |
|------|-------------|
| `detection.rs` | `Topology` struct and `detect_topology()` |
| `node.rs` | NUMA/topology node representation |
| `interconnect.rs` | Inter-node interconnect modeling |
| `system.rs` | System-wide topology view |

## Topology

```
Topology {
    sockets: u8              — number of physical CPU sockets
    cores_per_socket: u8     — cores per socket
}
```

## Detection

`detect_topology() -> Topology`

On x86_64, uses CPUID:
- Leaf `0x0B` (Extended Topology) for core/thread counts
- Leaf `0x04` (Deterministic Cache) for shared cache levels
- Leaf `0x01` for legacy core count

On AArch64, reads MPIDR_EL1 for affinity levels (cluster/core/thread).

## Re-exports

```rust
pub use detection::{detect_topology, Topology};
```

## Integration

- `cpu::topology` provides lower-level per-CPU topology info
- This module provides the system-wide view
- Used by the NUMA subsystem to map cores to memory nodes
- Used by the scheduler for load balancing decisions
