#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum EtherType {
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Ipv6 = 0x86DD,
    Vlan = 0x8100,
    Unknown = 0xFFFF,
}

impl EtherType {
    pub fn from_u16(v: u16) -> Self {
        match v {
            0x0800 => EtherType::Ipv4,
            0x0806 => EtherType::Arp,
            0x86DD => EtherType::Ipv6,
            0x8100 => EtherType::Vlan,
            _ => EtherType::Unknown,
        }
    }
}

#[derive(Copy, Clone)]
pub struct EthernetFrame {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ether_type: EtherType,
    pub payload_len: usize,
}

impl EthernetFrame {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 14 {
            return None;
        }
        let mut dst = [0u8; 6];
        let mut src = [0u8; 6];
        dst.copy_from_slice(&data[0..6]);
        src.copy_from_slice(&data[6..12]);
        let et = ((data[12] as u16) << 8) | data[13] as u16;
        Some(EthernetFrame {
            dst,
            src,
            ether_type: EtherType::from_u16(et),
            payload_len: data.len() - 14,
        })
    }
}
