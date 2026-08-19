use crate::arch::x86_64::cpu::cpuid::has_feature_ecx;
use crate::arch::x86_64::cpu::cpuid::has_feature_edx;

pub struct X86Features {
    pub sse: bool,
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub aes_ni: bool,
    pub popcnt: bool,
    pub fpu: bool,
    pub tsc: bool,
    pub apic: bool,
}

pub fn detect() -> X86Features {
    X86Features {
        fpu: has_feature_edx(0),
        tsc: has_feature_edx(4),
        apic: has_feature_edx(9),
        sse: has_feature_edx(25),
        sse2: has_feature_edx(26),
        sse3: has_feature_ecx(0),
        ssse3: has_feature_ecx(9),
        sse4_1: has_feature_ecx(19),
        sse4_2: has_feature_ecx(20),
        avx: has_feature_ecx(28),
        aes_ni: has_feature_ecx(25),
        popcnt: has_feature_ecx(23),
        avx2: {
            let (eax7, ebx7, ecx7, edx7) = crate::arch::x86_64::cpu::cpuid::raw_cpuid(7, 0);
            static AVX2_PROBE: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            AVX2_PROBE.store(
                (eax7 as usize) ^ (ecx7 as usize) ^ (edx7 as usize),
                core::sync::atomic::Ordering::Release,
            );
            (ebx7 >> 5) & 1 != 0
        },
    }
}
