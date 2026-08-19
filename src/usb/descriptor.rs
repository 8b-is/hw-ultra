pub const DESC_DEVICE: u8 = 0x01;
pub const DESC_CONFIGURATION: u8 = 0x02;
pub const DESC_STRING: u8 = 0x03;
pub const DESC_INTERFACE: u8 = 0x04;
pub const DESC_ENDPOINT: u8 = 0x05;
pub const DESC_HID: u8 = 0x21;
pub const DESC_HUB: u8 = 0x29;
pub const DESC_SS_HUB: u8 = 0x2A;
pub const DESC_BOS: u8 = 0x0F;

#[derive(Clone, Copy)]
pub struct DeviceDescriptor {
    pub usb_version: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub max_packet_size0: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_version: u16,
    pub num_configurations: u8,
}

#[derive(Clone, Copy)]
pub struct ConfigDescriptor {
    pub total_length: u16,
    pub num_interfaces: u8,
    pub config_value: u8,
    pub attributes: u8,
    pub max_power: u8,
}

#[derive(Clone, Copy)]
pub struct InterfaceDescriptor {
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub num_endpoints: u8,
    pub interface_class: u8,
    pub interface_subclass: u8,
    pub interface_protocol: u8,
}

impl DeviceDescriptor {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 18 || data[1] != DESC_DEVICE {
            return None;
        }
        Some(DeviceDescriptor {
            usb_version: u16::from_le_bytes([data[2], data[3]]),
            device_class: data[4],
            device_subclass: data[5],
            device_protocol: data[6],
            max_packet_size0: data[7],
            vendor_id: u16::from_le_bytes([data[8], data[9]]),
            product_id: u16::from_le_bytes([data[10], data[11]]),
            device_version: u16::from_le_bytes([data[12], data[13]]),
            num_configurations: data[17],
        })
    }

    pub fn is_hub(&self) -> bool {
        self.device_class == 0x09
    }

    pub fn is_hid(&self) -> bool {
        self.device_class == 0x03
    }

    pub fn is_mass_storage(&self) -> bool {
        self.device_class == 0x08
    }

    pub fn is_vendor_specific(&self) -> bool {
        self.device_class == 0xFF
    }
}

impl ConfigDescriptor {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 9 || data[1] != DESC_CONFIGURATION {
            return None;
        }
        Some(ConfigDescriptor {
            total_length: u16::from_le_bytes([data[2], data[3]]),
            num_interfaces: data[4],
            config_value: data[5],
            attributes: data[7],
            max_power: data[8],
        })
    }

    pub fn self_powered(&self) -> bool {
        self.attributes & (1 << 6) != 0
    }

    pub fn remote_wakeup(&self) -> bool {
        self.attributes & (1 << 5) != 0
    }

    pub fn max_power_ma(&self) -> u16 {
        self.max_power as u16 * 2
    }
}

impl InterfaceDescriptor {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 9 || data[1] != DESC_INTERFACE {
            return None;
        }
        Some(InterfaceDescriptor {
            interface_number: data[2],
            alternate_setting: data[3],
            num_endpoints: data[4],
            interface_class: data[5],
            interface_subclass: data[6],
            interface_protocol: data[7],
        })
    }
}
