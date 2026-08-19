#[derive(Clone, Copy, PartialEq)]
pub enum NfcProtocol {
    NfcA,
    NfcB,
    NfcF,
    NfcV,
    IsoDep,
    NfcDep,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TagType {
    Type1,
    Type2,
    Type3,
    Type4,
    Type5,
}

pub const SENS_REQ: u8 = 0x26;
pub const ALL_REQ: u8 = 0x52;
pub const SDD_REQ: u8 = 0x93;
pub const SEL_REQ: u8 = 0x93;
pub const RATS: u8 = 0xE0;

pub const SENSB_REQ: u8 = 0x05;
pub const ATTRIB: u8 = 0x1D;

pub const SENSF_REQ: u8 = 0x04;

pub fn protocol_bitrate(protocol: NfcProtocol) -> u32 {
    match protocol {
        NfcProtocol::NfcA => 106,
        NfcProtocol::NfcB => 106,
        NfcProtocol::NfcF => 212,
        NfcProtocol::NfcV => 26,
        NfcProtocol::IsoDep => 106,
        NfcProtocol::NfcDep => 106,
    }
}

pub fn modulation_code(protocol: NfcProtocol) -> u32 {
    match protocol {
        NfcProtocol::NfcA => 0x01,
        NfcProtocol::NfcB => 0x02,
        NfcProtocol::NfcF => 0x04,
        NfcProtocol::NfcV => 0x08,
        NfcProtocol::IsoDep => 0x10,
        NfcProtocol::NfcDep => 0x20,
    }
}

pub fn tag_type_for_protocol(protocol: NfcProtocol) -> Option<TagType> {
    match protocol {
        NfcProtocol::NfcA => Some(TagType::Type2),
        NfcProtocol::NfcB => Some(TagType::Type4),
        NfcProtocol::NfcF => Some(TagType::Type3),
        NfcProtocol::NfcV => Some(TagType::Type5),
        _ => None,
    }
}

pub fn build_sens_req(all: bool) -> u8 {
    if all {
        ALL_REQ
    } else {
        SENS_REQ
    }
}
