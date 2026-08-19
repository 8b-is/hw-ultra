use core::sync::atomic::{AtomicU8, Ordering};

const MAX_CHANNELS: usize = 8;
static GAIN: [AtomicU8; MAX_CHANNELS] = [const { AtomicU8::new(255) }; MAX_CHANNELS];
static MUTE: [AtomicU8; MAX_CHANNELS] = [const { AtomicU8::new(0) }; MAX_CHANNELS];

pub fn set_gain(channel: u8, gain: u8) {
    if (channel as usize) < MAX_CHANNELS {
        GAIN[channel as usize].store(gain, Ordering::Release);
    }
}

pub fn get_gain(channel: u8) -> u8 {
    if (channel as usize) < MAX_CHANNELS {
        GAIN[channel as usize].load(Ordering::Acquire)
    } else {
        0
    }
}

pub fn mute(channel: u8) {
    if (channel as usize) < MAX_CHANNELS {
        MUTE[channel as usize].store(1, Ordering::Release);
    }
}

pub fn unmute(channel: u8) {
    if (channel as usize) < MAX_CHANNELS {
        MUTE[channel as usize].store(0, Ordering::Release);
    }
}

pub fn is_muted(channel: u8) -> bool {
    if (channel as usize) < MAX_CHANNELS {
        MUTE[channel as usize].load(Ordering::Acquire) != 0
    } else {
        true
    }
}

pub fn set_master_gain(gain: u8) {
    let mut i = 0usize;
    while i < MAX_CHANNELS {
        GAIN[i].store(gain, Ordering::Release);
        i += 1;
    }
}

pub fn mute_all() {
    let mut i = 0usize;
    while i < MAX_CHANNELS {
        MUTE[i].store(1, Ordering::Release);
        i += 1;
    }
}

pub fn unmute_all() {
    let mut i = 0usize;
    while i < MAX_CHANNELS {
        MUTE[i].store(0, Ordering::Release);
        i += 1;
    }
}
