# CPU Module

The `cpu` module handles CPU detection, feature enumeration, topology analysis, frequency estimation, affinity control, context management, and thermal monitoring.

## Submodules

| File | Description |
|------|-------------|
| `api.rs` | Public API: `CpuInfo`, `ComponentStatus`, `detect_cpu_info()` |
| `info.rs` | `CpuInfo` struct definition, `model_name_str()` |
| `detect.rs` | CPU detection via CPUID/MIDR parsing |
| `core.rs` | `Core` struct, core registration (max 64) |
| `cores.rs` | `CoreInfo`, per-core detection |
| `features.rs` | `has_feature()` — SSE, AVX, NEON, etc. |
| `frequency.rs` | Brand string parsing, frequency estimation |
| `context.rs` | `Context` — 16-register save/restore |
| `affinity.rs` | CPU affinity via `sched_setaffinity`/`sched_getaffinity` |
| `topology.rs` | `Topology` — physical/logical cores, sockets, threads |
| `thermal.rs` | Per-core temperature via MSR 0x19C |
| `scheduler.rs` | Simple round-robin task scheduler |
| `vector.rs` | SIMD width detection |
| `speculation.rs` | Speculation mitigation (IBRS via MSR 0x48) |
| `interrupt.rs` | `raise_interrupt()` |
| `ram.rs` | `RamInfo` detection from SMBIOS |
| `arch_x86_64.rs` | x86_64-specific CPU utilities |
| `arch_aarch64.rs` | aarch64-specific CPU utilities |
| `power/` | CPU power states (Active, Idle, DeepIdle, Offline) |

## CpuInfo

```rust
pub struct CpuInfo {
    pub arch: Architecture,
    pub id: u64,
    pub vendor: &'static str,
    pub frequency_hz: u64,
    pub cores: u8,
    pub physical_cores: u8,
    pub logical_cores: u8,
    pub threads_per_core: u8,
    pub model_name: [u8; 48],
    pub model_name_len: u8,
    pub l1_cache_kb: u16,
    pub l2_cache_kb: u16,
    pub l3_cache_kb: u16,
    pub has_ht: bool,
}
```

## Detection flow

### x86_64
1. CPUID leaf 0 → vendor string (GenuineIntel, AuthenticAMD)
2. CPUID leaf 1 → family, model, stepping, features
3. CPUID leaf 4 → cache topology
4. CPUID leaf 0x0B → extended topology (cores, threads)
5. CPUID leaf 0x16 → processor frequency info
6. CPUID leaf 0x80000002-4 → brand string (model name)

### AArch64

For both architectures, the CPUID/MIDR core count is then overridden by the OS affinity count via `sched_getaffinity` (128-byte mask, popcount) when higher, to handle multi-socket systems.
1. MIDR_EL1 → implementer, variant, part number, revision
2. Implementer codes: 0x41=ARM, 0x51=Qualcomm, 0x43=Cavium, etc.

## Feature detection

```rust
pub fn has_feature(name: &str) -> bool
```

Checks CPUID for: `"sse"`, `"sse2"`, `"sse3"`, `"ssse3"`, `"sse4.1"`, `"sse4.2"`, `"avx"`, `"avx2"`, `"avx512"`, `"aes"`, `"neon"` (aarch64).

## Frequency estimation

Four methods, tried in order:
1. **CPUID leaf 0x16** (Intel 6th gen+): direct frequency in MHz
2. **CPUID leaf 0x15** (TSC/crystal ratio): if ECX reports a crystal frequency, use it directly. If ECX is 0, `infer_crystal_hz()` derives the crystal clock from the CPU family/model (Intel SDM Table 18-85): 19.2 MHz for Atom Goldmont/Denverton, 24 MHz for Skylake through Raptor Lake. Returns 0 for unknown models, skipping this path.
3. **Brand string parsing**: extracts "X.XX GHz" or "XXXX MHz" from CPUID 0x80000002–4
4. **TSC calibration** via PIT-based measurement

## Thermal monitoring

```rust
pub fn read_core_temperatures(out: &mut [Option<i32>]) -> usize
```

Reads MSR 0x19C (IA32_THERM_STATUS) and MSR 0x1A2 (MSR_TEMPERATURE_TARGET) on Intel CPUs. Returns temperature in Celsius: `TJ_MAX - digital_readout`. Returns 0 on AMD or unsupported CPUs.
