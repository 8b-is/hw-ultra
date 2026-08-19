use core::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum SampleRate {
    Hz1,
    Hz10,
    Hz25,
    Hz50,
    Hz100,
    Hz200,
    Hz400,
    Hz800,
}

static CURRENT_RATE: AtomicU32 = AtomicU32::new(100);

pub fn rate_to_hz(rate: SampleRate) -> u32 {
    match rate {
        SampleRate::Hz1 => 1,
        SampleRate::Hz10 => 10,
        SampleRate::Hz25 => 25,
        SampleRate::Hz50 => 50,
        SampleRate::Hz100 => 100,
        SampleRate::Hz200 => 200,
        SampleRate::Hz400 => 400,
        SampleRate::Hz800 => 800,
    }
}

pub fn rate_to_register(rate: SampleRate) -> u32 {
    match rate {
        SampleRate::Hz1 => 0x01,
        SampleRate::Hz10 => 0x02,
        SampleRate::Hz25 => 0x03,
        SampleRate::Hz50 => 0x04,
        SampleRate::Hz100 => 0x05,
        SampleRate::Hz200 => 0x06,
        SampleRate::Hz400 => 0x07,
        SampleRate::Hz800 => 0x08,
    }
}

pub fn set_rate(rate: SampleRate) {
    CURRENT_RATE.store(rate_to_hz(rate), Ordering::Release);
}

pub fn current_rate_hz() -> u32 {
    CURRENT_RATE.load(Ordering::Acquire)
}

pub fn period_us(rate: SampleRate) -> u32 {
    let hz = rate_to_hz(rate);
    if hz == 0 {
        return 0;
    }
    1_000_000 / hz
}

pub fn samples_for_duration_ms(rate: SampleRate, ms: u32) -> u32 {
    let hz = rate_to_hz(rate);
    (hz * ms) / 1000
}
