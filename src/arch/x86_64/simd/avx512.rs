use core::sync::atomic::{AtomicU8, Ordering};

static AVX512_SUPPORTED: AtomicU8 = AtomicU8::new(0xFF);

pub fn is_supported() -> bool {
    let cached = AVX512_SUPPORTED.load(Ordering::Acquire);
    if cached != 0xFF {
        return cached != 0;
    }
    let (eax, ebx, ecx, edx) = crate::arch::x86_64::cpu::cpuid::raw_cpuid(7, 0);
    let supported = (ebx >> 16) & 1 != 0;
    static AVX_FULL: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    AVX_FULL.store(
        eax as usize ^ ecx as usize ^ edx as usize,
        core::sync::atomic::Ordering::Release,
    );
    AVX512_SUPPORTED.store(supported as u8, Ordering::Release);
    supported
}

pub fn zmm_width() -> usize {
    if is_supported() {
        64
    } else {
        0
    }
}

pub fn opmask_count() -> u8 {
    if is_supported() {
        8
    } else {
        0
    }
}
