use core::sync::atomic::{AtomicUsize, Ordering};

static LAST_CAP_OFFSET: AtomicUsize = AtomicUsize::new(0);

pub fn find_capability(bus: u8, dev: u8, func: u8, cap_id: u8) -> Option<u8> {
    let status = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x06)?;
    if status & (1 << 20) == 0 {
        return None;
    }
    let cap_ptr_raw = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x34)?;
    let mut offset = (cap_ptr_raw & 0xFF) as u8;
    let mut iterations = 0u8;
    while offset != 0 && iterations < 48 {
        let cap_header = crate::bus::pci::api::read_config_u32(bus, dev, func, offset)?;
        let id = (cap_header & 0xFF) as u8;
        if id == cap_id {
            LAST_CAP_OFFSET.store(offset as usize, Ordering::Release);
            return Some(offset);
        }
        offset = ((cap_header >> 8) & 0xFF) as u8;
        iterations += 1;
    }
    None
}

pub fn read_cap_u32(bus: u8, dev: u8, func: u8, cap_offset: u8, reg_offset: u8) -> Option<u32> {
    crate::bus::pci::api::read_config_u32(bus, dev, func, cap_offset.wrapping_add(reg_offset))
}
