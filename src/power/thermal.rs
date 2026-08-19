pub fn check_temp() {
    if let Some(v) = crate::hardware_access::read_msr(0x19C) {
        let digital = ((v >> 16) & 0x7f) as u8;
        static TEMP_SIG: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);
        TEMP_SIG.store(digital, core::sync::atomic::Ordering::Release);
    }
}
