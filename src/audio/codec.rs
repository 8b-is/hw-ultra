#[derive(Copy, Clone, PartialEq)]
pub enum CodecType {
    AudioOutput,
    AudioInput,
    AudioMixer,
    AudioSelector,
    PinComplex,
    PowerWidget,
    VendorDefined,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct CodecNode {
    pub codec_id: u8,
    pub node_id: u16,
    pub node_type: CodecType,
    pub vendor_id: u32,
    pub subsystem_id: u32,
}

pub fn verb_get_parameter(codec: u8, node: u16, param: u8) -> u32 {
    ((codec as u32) << 28) | ((node as u32) << 20) | (0xF00 << 8) | param as u32
}

pub fn verb_set_stream(codec: u8, node: u16, stream: u8, channel: u8) -> u32 {
    ((codec as u32) << 28)
        | ((node as u32) << 20)
        | (0x706 << 8)
        | ((stream as u32) << 4)
        | channel as u32
}

pub fn verb_set_pin_control(codec: u8, node: u16, val: u8) -> u32 {
    ((codec as u32) << 28) | ((node as u32) << 20) | (0x707 << 8) | val as u32
}

pub fn verb_set_power_state(codec: u8, node: u16, state: u8) -> u32 {
    ((codec as u32) << 28) | ((node as u32) << 20) | (0x705 << 8) | state as u32
}

pub fn node_type_from_cap(cap: u32) -> CodecType {
    match (cap >> 20) & 0xF {
        0x0 => CodecType::AudioOutput,
        0x1 => CodecType::AudioInput,
        0x2 => CodecType::AudioMixer,
        0x3 => CodecType::AudioSelector,
        0x4 => CodecType::PinComplex,
        0x5 => CodecType::PowerWidget,
        0xF => CodecType::VendorDefined,
        _ => CodecType::Unknown,
    }
}
