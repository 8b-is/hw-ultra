pub fn has_feature(name: &str) -> bool {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => has_feature_x86(name),
        crate::arch::Architecture::AArch64 => has_feature_arm(name),
        _ => false,
    }
}

fn has_feature_x86(name: &str) -> bool {
    let (ecx1, edx1) = match crate::hardware_access::read_cpuid(1, 0) {
        Some((_, _, c, d)) => (c, d),
        None => return false,
    };
    match name {
        "sse" => (edx1 & (1 << 25)) != 0,
        "sse2" => (edx1 & (1 << 26)) != 0,
        "sse3" => (ecx1 & (1 << 0)) != 0,
        "ssse3" => (ecx1 & (1 << 9)) != 0,
        "sse4_1" | "sse4.1" => (ecx1 & (1 << 19)) != 0,
        "sse4_2" | "sse4.2" => (ecx1 & (1 << 20)) != 0,
        "avx" => (ecx1 & (1 << 28)) != 0,
        "aes" | "aes-ni" => (ecx1 & (1 << 25)) != 0,
        "fma" | "fma3" => (ecx1 & (1 << 12)) != 0,
        "pclmulqdq" => (ecx1 & (1 << 1)) != 0,
        "popcnt" => (ecx1 & (1 << 23)) != 0,
        "f16c" => (ecx1 & (1 << 29)) != 0,
        "avx2" => crate::hardware_access::read_cpuid(7, 0)
            .map(|(_, b, _, _)| (b & (1 << 5)) != 0)
            .unwrap_or(false),
        "bmi1" => crate::hardware_access::read_cpuid(7, 0)
            .map(|(_, b, _, _)| (b & (1 << 3)) != 0)
            .unwrap_or(false),
        "bmi2" => crate::hardware_access::read_cpuid(7, 0)
            .map(|(_, b, _, _)| (b & (1 << 8)) != 0)
            .unwrap_or(false),
        "avx512f" => crate::hardware_access::read_cpuid(7, 0)
            .map(|(_, b, _, _)| (b & (1 << 16)) != 0)
            .unwrap_or(false),
        "sha" => crate::hardware_access::read_cpuid(7, 0)
            .map(|(_, b, _, _)| (b & (1 << 29)) != 0)
            .unwrap_or(false),
        _ => false,
    }
}

/// ARM feature detection via MIDR part number table.
/// Features are determined by the ARM architecture version of the core.
fn has_feature_arm(name: &str) -> bool {
    let midr = match crate::arch::shim::read_aarch64_midr() {
        Some(m) => m,
        None => {
            // Without MIDR, only claim mandatory ARMv8 baseline
            return matches!(name, "fp" | "asimd" | "neon");
        }
    };
    let part = ((midr >> 4) & 0xfff) as u16;
    // Determine architecture version from part number
    // v8.0: A53(d03), A35(d04), A57(d07), A72(d08), A73(d09)
    // v8.2+: A55(d05), A65(d06), A75(d0a), A76(d0b), N1(d0c), A77(d0d),
    //        NV1(d40), A78(d41), X1(d44)
    // v9.0+: A510(d46), A710(d47), X2(d48), A715(d4d), X3(d4e),
    //        A520(d80), A720(d81), X4(d82)
    let is_v81_plus = !matches!(part, 0xd03 | 0xd04 | 0xd07 | 0xd08 | 0xd09);
    match name {
        "fp" | "vfp" | "vfpv3" | "vfpv4" => true,
        "asimd" | "neon" => true,
        "crc32" => is_v81_plus,
        "atomics" | "lse" => is_v81_plus,
        _ => false,
    }
}
