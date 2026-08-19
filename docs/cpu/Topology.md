# CPU Topology

Detects the physical layout of the CPU: sockets, cores, threads, and their relationships.

## Structures

```rust
pub struct Topology {
    pub physical_cores: u8,
    pub logical_cores: u8,
    pub sockets: u8,
    pub threads_per_core: u8,
}
```

## Detection

```rust
pub fn detect() -> Topology
```

On x86_64, uses CPUID leaves:
- Leaf 1: basic feature flags, hyperthreading bit
- Leaf 4: deterministic cache parameters (core count per package)
- Leaf 0x0B: extended topology enumeration (sub-leaf 0 = SMT, sub-leaf 1 = core)
- Leaf 0x1F: v2 topology (if leaf 0x0B unavailable)

On aarch64, topology is typically provided by the DeviceTree or ACPI MADT table.

## Multi-socket detection

The crate detects multi-socket systems by examining:
1. APIC ID ranges (x86_64)
2. ACPI MADT processor entries

## Relationship to cpu::cores

`cpu::topology::detect()` provides the aggregate view (counts), while `cpu::cores::detect_cores()` provides per-core detail (frequency, temperature).
