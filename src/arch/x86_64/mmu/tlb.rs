pub fn invalidate_tlb() {
    if crate::arch::detect_arch() == crate::arch::Architecture::X86_64 {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    }
}
