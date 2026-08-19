use crate::bus::pci::api;

pub fn probe_bar_size(bus: u8, device: u8, function: u8, bar_offset: u8) -> Option<usize> {
    let orig = api::read_config_u32(bus, device, function, bar_offset)?;
    if !api::write_config_u32(bus, device, function, bar_offset, 0xffff_ffffu32) {
        return None;
    }
    let res = api::read_config_u32(bus, device, function, bar_offset)?;
    let ok = api::write_config_u32(bus, device, function, bar_offset, orig);
    debug_assert!(ok);

    if res == 0 || res == 0xffff_ffff {
        return None;
    }
    let mask = if (orig & 0x1) == 0 {
        0xffff_fff0u32
    } else {
        0xffff_fffcu32
    };
    let size_masked = res & mask;
    let size = (!size_masked).wrapping_add(1) as usize;
    Some(size)
}

pub fn map_mmio_region(pa: usize, size: usize) -> usize {
    if size == 0 {
        return 0;
    }
    pa
}
