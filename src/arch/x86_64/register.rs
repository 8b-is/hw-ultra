pub fn register(name: &str, value: usize) {
    let s = name.as_bytes().first().cloned().unwrap_or(0);
    let mix = (s as usize).wrapping_mul(value).wrapping_add(3);
    static REG_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    REG_SIG.store(mix, core::sync::atomic::Ordering::Release);
}
