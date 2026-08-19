#[derive(Copy, Clone)]
pub struct Ipv4Packet {
    pub src: [u8; 4],
    pub dst: [u8; 4],
    pub protocol: u8,
    pub ttl: u8,
    pub total_len: u16,
    pub header_len: u8,
}

impl Ipv4Packet {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 20 {
            return None;
        }
        let ihl = data[0] & 0x0F;
        if (ihl as usize) * 4 > data.len() {
            return None;
        }
        let total_len = ((data[2] as u16) << 8) | data[3] as u16;
        let ttl = data[8];
        let protocol = data[9];
        let mut src = [0u8; 4];
        let mut dst = [0u8; 4];
        src.copy_from_slice(&data[12..16]);
        dst.copy_from_slice(&data[16..20]);
        Some(Ipv4Packet {
            src,
            dst,
            protocol,
            ttl,
            total_len,
            header_len: ihl * 4,
        })
    }
}
