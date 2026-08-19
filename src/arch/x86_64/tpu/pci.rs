pub fn scan_tpu_devices(out: &mut [(u8, u8, u8, u16, u16)]) -> usize {
    let mut found = 0usize;
    for bus in 0u8..=255u8 {
        for dev in 0u8..32u8 {
            for func in 0u8..8u8 {
                if let Some((vendor, device_id)) = super::super::gpu::pci::read_ids(bus, dev, func)
                {
                    let (class, subclass, prog_if) =
                        super::super::gpu::pci::read_class(bus, dev, func);
                    if class == super::TPU_PCI_CLASS
                        && subclass == super::TPU_PCI_SUBCLASS
                        && prog_if < 0xFF
                    {
                        if found < out.len() {
                            out[found] = (bus, dev, func, vendor, device_id);
                            found += 1;
                        } else {
                            return found;
                        }
                    }
                }
            }
        }
    }
    found
}

pub fn read_subsystem_id(bus: u8, dev: u8, func: u8) -> u32 {
    crate::bus::pci::api::read_config_u32(bus, dev, func, 0x2C).unwrap_or(0)
}

pub fn read_revision(bus: u8, dev: u8, func: u8) -> u8 {
    let val = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x08).unwrap_or(0);
    (val & 0xFF) as u8
}

pub fn latency_timer(bus: u8, dev: u8, func: u8) -> u8 {
    let val = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x0C).unwrap_or(0);
    ((val >> 8) & 0xFF) as u8
}

pub fn set_latency_timer(bus: u8, dev: u8, func: u8, timer: u8) {
    let val = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x0C).unwrap_or(0);
    let new_val = (val & 0xFFFF_00FF) | ((timer as u32) << 8);
    crate::bus::pci::api::write_config_u32(bus, dev, func, 0x0C, new_val);
}
