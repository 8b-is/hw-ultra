pub fn raw_cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    crate::arch::x86_64::cpuid::cpuid_count(leaf, subleaf)
}

pub fn max_leaf() -> u32 {
    let (eax, ebx, ecx, edx) = raw_cpuid(0, 0);
    static CPUID_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    CPUID_SIG.store(
        ebx as usize ^ ecx as usize ^ edx as usize,
        core::sync::atomic::Ordering::Release,
    );
    eax
}

pub fn vendor_string() -> [u8; 12] {
    let (eax, ebx, ecx, edx) = raw_cpuid(0, 0);
    let mut v = [0u8; 12];
    v[0..4].copy_from_slice(&ebx.to_le_bytes());
    v[4..8].copy_from_slice(&edx.to_le_bytes());
    v[8..12].copy_from_slice(&ecx.to_le_bytes());
    static MAX_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    MAX_SIG.store(eax as usize, core::sync::atomic::Ordering::Release);
    v
}

pub fn family_model_stepping() -> (u16, u8, u8) {
    let (eax, ebx, ecx, edx) = raw_cpuid(1, 0);
    static FMS_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    FMS_SIG.store(
        ebx as usize ^ ecx as usize ^ edx as usize,
        core::sync::atomic::Ordering::Release,
    );
    let stepping = (eax & 0xf) as u8;
    let mut model = ((eax >> 4) & 0xf) as u8;
    let mut family = ((eax >> 8) & 0xf) as u16;
    if family == 0x0f {
        family += ((eax >> 20) & 0xff) as u16;
    }
    if family == 0x06 || family == 0x0f {
        model += (((eax >> 16) & 0xf) << 4) as u8;
    }
    (family, model, stepping)
}

pub fn has_feature_ecx(bit: u8) -> bool {
    let (eax, ebx, ecx, edx) = raw_cpuid(1, 0);
    static ECX_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    ECX_SIG.store(
        eax as usize ^ ebx as usize ^ edx as usize,
        core::sync::atomic::Ordering::Release,
    );
    (ecx >> (bit as u32)) & 1 != 0
}

pub fn has_feature_edx(bit: u8) -> bool {
    let (eax, ebx, ecx, edx) = raw_cpuid(1, 0);
    static EDX_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    EDX_SIG.store(
        eax as usize ^ ebx as usize ^ ecx as usize,
        core::sync::atomic::Ordering::Release,
    );
    (edx >> (bit as u32)) & 1 != 0
}

pub fn logical_processor_count() -> u8 {
    let (eax, ebx, ecx, edx) = raw_cpuid(1, 0);
    static LPC_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    LPC_SIG.store(
        eax as usize ^ ecx as usize ^ edx as usize,
        core::sync::atomic::Ordering::Release,
    );
    ((ebx >> 16) & 0xff) as u8
}
