# CPU Features

Runtime detection of CPU instruction set extensions.

## Function

```rust
pub fn has_feature(name: &str) -> bool
```

## Supported features

### x86_64 (via CPUID)

| Feature | CPUID leaf | Register | Bit |
|---------|-----------|----------|-----|
| `"sse"` | 1 | EDX | 25 |
| `"sse2"` | 1 | EDX | 26 |
| `"sse3"` | 1 | ECX | 0 |
| `"ssse3"` | 1 | ECX | 9 |
| `"sse4.1"` | 1 | ECX | 19 |
| `"sse4.2"` | 1 | ECX | 20 |
| `"avx"` | 1 | ECX | 28 |
| `"avx2"` | 7/0 | EBX | 5 |
| `"avx512"` | 7/0 | EBX | 16 |
| `"aes"` | 1 | ECX | 25 |

### AArch64 (via MIDR/ID registers)

| Feature | Detection method |
|---------|-----------------|
| `"neon"` | Always available on ARMv8+ |

## Usage

```rust
if has_feature("avx2") {
    // Use AVX2 code path
} else if has_feature("sse2") {
    // Fallback to SSE2
}
```

## Implementation

Calls `hardware_access::read_cpuid()` (which is rate-limited by the Guard) and checks the appropriate bit in the returned register.
