use core::sync::atomic::{AtomicUsize, Ordering};

static FREQ_HZ: AtomicUsize = AtomicUsize::new(0);

pub fn set_frequency(freq_hz: u64) {
    FREQ_HZ.store(freq_hz as usize, Ordering::Release);
}

pub fn current_frequency() -> u64 {
    let cached = FREQ_HZ.load(Ordering::Acquire) as u64;
    if cached > 0 {
        return cached;
    }
    let sysfs = crate::cpu::frequency::read_cpu_freq_sysfs(0);
    if sysfs > 0 {
        FREQ_HZ.store(sysfs as usize, Ordering::Release);
        return sysfs;
    }
    0
}
