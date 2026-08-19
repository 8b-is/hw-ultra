pub const NDEF_MAGIC: u8 = 0xE1;
pub const NDEF_TLV_TERMINATOR: u8 = 0xFE;
pub const NDEF_TLV_NDEF_MESSAGE: u8 = 0x03;
pub const NDEF_TLV_NULL: u8 = 0x00;

pub const MAX_UID_LEN: usize = 10;
pub const MAX_NDEF_SIZE: usize = 256;

#[derive(Clone, Copy, PartialEq)]
pub enum NdefRecordType {
    Text,
    Uri,
    SmartPoster,
    MimeMedia,
    AbsoluteUri,
    External,
    Unknown,
}

#[derive(Clone, Copy)]
pub struct TagUid {
    pub bytes: [u8; MAX_UID_LEN],
    pub len: u8,
}

impl TagUid {
    pub const fn empty() -> Self {
        TagUid {
            bytes: [0u8; MAX_UID_LEN],
            len: 0,
        }
    }

    pub fn uid_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }

    pub fn is_valid(&self) -> bool {
        self.len > 0 && self.len as usize <= MAX_UID_LEN
    }
}

pub struct NdefMessage {
    pub data: [u8; MAX_NDEF_SIZE],
    pub len: usize,
}

impl NdefMessage {
    pub const fn empty() -> Self {
        NdefMessage {
            data: [0u8; MAX_NDEF_SIZE],
            len: 0,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

pub fn has_ndef_magic(data: &[u8]) -> bool {
    !data.is_empty() && data[0] == NDEF_MAGIC
}

pub fn find_ndef_tlv(data: &[u8]) -> Option<(usize, usize)> {
    let mut i = 0;
    while i < data.len() {
        let tag = data[i];
        if tag == NDEF_TLV_TERMINATOR {
            return None;
        }
        if tag == NDEF_TLV_NULL {
            i += 1;
            continue;
        }
        if i + 1 >= data.len() {
            return None;
        }
        let length = data[i + 1] as usize;
        if tag == NDEF_TLV_NDEF_MESSAGE {
            let start = i + 2;
            if start + length <= data.len() {
                return Some((start, length));
            }
            return None;
        }
        i += 2 + length;
    }
    None
}

pub fn ndef_record_tnf(header: u8) -> u8 {
    header & 0x07
}

pub fn ndef_record_type_from_tnf(tnf: u8) -> NdefRecordType {
    match tnf {
        0x01 => NdefRecordType::Text,
        0x02 => NdefRecordType::MimeMedia,
        0x03 => NdefRecordType::AbsoluteUri,
        0x04 => NdefRecordType::External,
        _ => NdefRecordType::Unknown,
    }
}

pub fn is_uri_record(type_field: &[u8]) -> bool {
    type_field.len() == 1 && type_field[0] == b'U'
}

pub fn is_text_record(type_field: &[u8]) -> bool {
    type_field.len() == 1 && type_field[0] == b'T'
}
