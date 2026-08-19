#[derive(Clone, Copy, PartialEq)]
pub enum TransferType {
    Control,
    Bulk,
    Interrupt,
    Isochronous,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TransferStatus {
    Pending,
    Active,
    Completed,
    Stalled,
    Error,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SetupDirection {
    HostToDevice,
    DeviceToHost,
}

#[derive(Clone, Copy)]
pub struct SetupPacket {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

impl SetupPacket {
    pub const fn new(request_type: u8, request: u8, value: u16, index: u16, length: u16) -> Self {
        SetupPacket {
            request_type,
            request,
            value,
            index,
            length,
        }
    }

    pub fn direction(&self) -> SetupDirection {
        if self.request_type & 0x80 != 0 {
            SetupDirection::DeviceToHost
        } else {
            SetupDirection::HostToDevice
        }
    }

    pub fn get_descriptor(desc_type: u8, index: u8, length: u16) -> Self {
        SetupPacket::new(
            0x80,
            0x06,
            ((desc_type as u16) << 8) | index as u16,
            0,
            length,
        )
    }

    pub fn set_address(addr: u8) -> Self {
        SetupPacket::new(0x00, 0x05, addr as u16, 0, 0)
    }

    pub fn set_configuration(config: u8) -> Self {
        SetupPacket::new(0x00, 0x09, config as u16, 0, 0)
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let v = self.value.to_le_bytes();
        let i = self.index.to_le_bytes();
        let l = self.length.to_le_bytes();
        [
            self.request_type,
            self.request,
            v[0],
            v[1],
            i[0],
            i[1],
            l[0],
            l[1],
        ]
    }
}

pub struct Transfer {
    pub transfer_type: TransferType,
    pub endpoint: u8,
    pub buffer_addr: usize,
    pub buffer_len: usize,
    pub status: TransferStatus,
    pub actual_len: usize,
}

impl Transfer {
    pub fn control(endpoint: u8, buffer_addr: usize, buffer_len: usize) -> Self {
        Transfer {
            transfer_type: TransferType::Control,
            endpoint,
            buffer_addr,
            buffer_len,
            status: TransferStatus::Pending,
            actual_len: 0,
        }
    }

    pub fn bulk(endpoint: u8, buffer_addr: usize, buffer_len: usize) -> Self {
        Transfer {
            transfer_type: TransferType::Bulk,
            endpoint,
            buffer_addr,
            buffer_len,
            status: TransferStatus::Pending,
            actual_len: 0,
        }
    }

    pub fn interrupt(endpoint: u8, buffer_addr: usize, buffer_len: usize) -> Self {
        Transfer {
            transfer_type: TransferType::Interrupt,
            endpoint,
            buffer_addr,
            buffer_len,
            status: TransferStatus::Pending,
            actual_len: 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(
            self.status,
            TransferStatus::Completed | TransferStatus::Stalled | TransferStatus::Error
        )
    }
}
