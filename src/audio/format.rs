#[derive(Copy, Clone, PartialEq)]
pub enum SampleRate {
    Hz8000,
    Hz11025,
    Hz16000,
    Hz22050,
    Hz32000,
    Hz44100,
    Hz48000,
    Hz88200,
    Hz96000,
    Hz176400,
    Hz192000,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BitDepth {
    Bits8,
    Bits16,
    Bits20,
    Bits24,
    Bits32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Channels {
    Mono,
    Stereo,
    Surround4,
    Surround51,
    Surround71,
}

pub fn sample_rate_hz(rate: SampleRate) -> u32 {
    match rate {
        SampleRate::Hz8000 => 8000,
        SampleRate::Hz11025 => 11025,
        SampleRate::Hz16000 => 16000,
        SampleRate::Hz22050 => 22050,
        SampleRate::Hz32000 => 32000,
        SampleRate::Hz44100 => 44100,
        SampleRate::Hz48000 => 48000,
        SampleRate::Hz88200 => 88200,
        SampleRate::Hz96000 => 96000,
        SampleRate::Hz176400 => 176400,
        SampleRate::Hz192000 => 192000,
    }
}

pub fn bits_per_sample(depth: BitDepth) -> u8 {
    match depth {
        BitDepth::Bits8 => 8,
        BitDepth::Bits16 => 16,
        BitDepth::Bits20 => 20,
        BitDepth::Bits24 => 24,
        BitDepth::Bits32 => 32,
    }
}

pub fn channel_count(ch: Channels) -> u8 {
    match ch {
        Channels::Mono => 1,
        Channels::Stereo => 2,
        Channels::Surround4 => 4,
        Channels::Surround51 => 6,
        Channels::Surround71 => 8,
    }
}

pub fn frame_size(depth: BitDepth, ch: Channels) -> u16 {
    (bits_per_sample(depth) as u16 / 8) * channel_count(ch) as u16
}

pub fn encode_hda_format(rate: SampleRate, depth: BitDepth, ch: Channels) -> u32 {
    let base = match rate {
        SampleRate::Hz44100 | SampleRate::Hz88200 | SampleRate::Hz176400 => 1u32 << 14,
        _ => 0u32,
    };
    let mult = match rate {
        SampleRate::Hz88200 | SampleRate::Hz96000 => 1u32 << 11,
        SampleRate::Hz176400 | SampleRate::Hz192000 => 3u32 << 11,
        _ => 0u32,
    };
    let div = match rate {
        SampleRate::Hz8000 => 5u32 << 8,
        SampleRate::Hz11025 => 3u32 << 8,
        SampleRate::Hz16000 => 2u32 << 8,
        SampleRate::Hz22050 => 1u32 << 8,
        SampleRate::Hz32000 => 2u32 << 8,
        _ => 0u32,
    };
    let bits = match depth {
        BitDepth::Bits8 => 0u32,
        BitDepth::Bits16 => 1u32 << 4,
        BitDepth::Bits20 => 2u32 << 4,
        BitDepth::Bits24 => 3u32 << 4,
        BitDepth::Bits32 => 4u32 << 4,
    };
    let chan = (channel_count(ch) as u32).saturating_sub(1);
    base | mult | div | bits | chan
}
