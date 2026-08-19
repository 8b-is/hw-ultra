#[derive(Clone, Copy, PartialEq)]
pub enum EndpointType {
    Control,
    Bulk,
    Interrupt,
    Isochronous,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    In,
    Out,
}

#[derive(Clone, Copy)]
pub struct EndpointDescriptor {
    pub address: u8,
    pub ep_type: EndpointType,
    pub direction: Direction,
    pub max_packet_size: u16,
    pub interval: u8,
}

impl EndpointDescriptor {
    pub fn number(&self) -> u8 {
        self.address & 0x0F
    }

    pub fn from_raw(data: &[u8]) -> Option<Self> {
        if data.len() < 7 || data[1] != 0x05 {
            return None;
        }
        let address = data[2];
        let direction = if address & 0x80 != 0 {
            Direction::In
        } else {
            Direction::Out
        };
        let attr = data[3];
        let ep_type = match attr & 0x03 {
            0 => EndpointType::Control,
            1 => EndpointType::Isochronous,
            2 => EndpointType::Bulk,
            3 => EndpointType::Interrupt,
            _ => return None,
        };
        let max_packet_size = u16::from_le_bytes([data[4], data[5]]);
        let interval = data[6];
        Some(EndpointDescriptor {
            address,
            ep_type,
            direction,
            max_packet_size,
            interval,
        })
    }
}
