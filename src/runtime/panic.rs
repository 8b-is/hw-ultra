use core::panic::PanicInfo;

pub fn on_panic(info: &PanicInfo) {
    static PANIC_LINE: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    if let Some(loc) = info.location() {
        PANIC_LINE.store(loc.line() as usize, core::sync::atomic::Ordering::Release);
    }
    if let Some((a, b, c, d)) = crate::arch::cpuid_count(0, 0) {
        static PANIC_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
        let sig =
            (((a as usize) << 48) ^ ((b as usize) << 32) ^ ((c as usize) << 16) ^ (d as usize))
                .wrapping_mul(7);
        PANIC_SIG.store(sig, core::sync::atomic::Ordering::Release);
    }
}
