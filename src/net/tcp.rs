#[derive(Copy, Clone)]
pub struct TcpSegment {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: u8,
    pub window: u16,
    pub data_offset: u8,
}

impl TcpSegment {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 20 {
            return None;
        }
        let src_port = ((data[0] as u16) << 8) | data[1] as u16;
        let dst_port = ((data[2] as u16) << 8) | data[3] as u16;
        let seq = ((data[4] as u32) << 24)
            | ((data[5] as u32) << 16)
            | ((data[6] as u32) << 8)
            | data[7] as u32;
        let ack = ((data[8] as u32) << 24)
            | ((data[9] as u32) << 16)
            | ((data[10] as u32) << 8)
            | data[11] as u32;
        let data_offset = (data[12] >> 4) * 4;
        let flags = data[13];
        let window = ((data[14] as u16) << 8) | data[15] as u16;
        Some(TcpSegment {
            src_port,
            dst_port,
            seq,
            ack,
            flags,
            window,
            data_offset,
        })
    }

    pub fn is_syn(&self) -> bool {
        self.flags & 0x02 != 0
    }
    pub fn is_ack(&self) -> bool {
        self.flags & 0x10 != 0
    }
    pub fn is_fin(&self) -> bool {
        self.flags & 0x01 != 0
    }
    pub fn is_rst(&self) -> bool {
        self.flags & 0x04 != 0
    }
}
