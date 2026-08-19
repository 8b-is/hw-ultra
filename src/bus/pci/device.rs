use crate::bus::pci;

#[derive(Copy, Clone, Debug)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
}

impl PciDevice {
    pub fn from_bdf(bus: u8, device: u8, function: u8) -> Option<Self> {
        if let Some(d0) = pci::read_config_u32(bus, device, function, 0) {
            let vendor = (d0 & 0xffff) as u16;
            if vendor == 0xffff {
                return None;
            }
            let did = ((d0 >> 16) & 0xffff) as u16;
            let cls = pci::read_config_u32(bus, device, function, 0x08).unwrap_or(0);
            let class = ((cls >> 24) & 0xff) as u8;
            let subclass = ((cls >> 16) & 0xff) as u8;
            Some(PciDevice {
                bus,
                device,
                function,
                vendor_id: vendor,
                device_id: did,
                class,
                subclass,
            })
        } else {
            None
        }
    }
}

pub fn scan_all(out: &mut [PciDevice]) -> usize {
    let mut found = 0usize;
    for b in 0u8..=255u8 {
        for d in 0u8..32u8 {
            for f in 0u8..8u8 {
                if let Some(dev) = PciDevice::from_bdf(b, d, f) {
                    if found < out.len() {
                        out[found] = dev;
                        found += 1;
                    } else {
                        return found;
                    }
                }
            }
        }
    }
    found
}
