pub const CAP_MSI: u8 = 0x05;
pub const CAP_MSIX: u8 = 0x11;
pub const CAP_PCIE: u8 = 0x10;
pub const CAP_PM: u8 = 0x01;

pub const CMD_IO_SPACE: u16 = 1 << 0;
pub const CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const CMD_BUS_MASTER: u16 = 1 << 2;
pub const CMD_INTX_DISABLE: u16 = 1 << 10;

pub fn read_ids(bus: u8, dev: u8, func: u8) -> Option<(u16, u16)> {
    let val = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x00)?;
    let vendor = (val & 0xFFFF) as u16;
    if vendor == 0xFFFF {
        return None;
    }
    let device_id = ((val >> 16) & 0xFFFF) as u16;
    Some((vendor, device_id))
}

pub fn read_command(bus: u8, dev: u8, func: u8) -> u16 {
    crate::bus::pci::api::read_config_u32(bus, dev, func, 0x04)
        .map(|v| (v & 0xFFFF) as u16)
        .unwrap_or(0)
}

pub fn write_command(bus: u8, dev: u8, func: u8, cmd: u16) {
    let status = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x04).unwrap_or(0);
    let new_val = (status & 0xFFFF_0000) | (cmd as u32);
    crate::bus::pci::api::write_config_u32(bus, dev, func, 0x04, new_val);
}

pub fn enable_bus_master(bus: u8, dev: u8, func: u8) {
    let cmd = read_command(bus, dev, func);
    write_command(bus, dev, func, cmd | CMD_BUS_MASTER);
}

pub fn enable_memory_space(bus: u8, dev: u8, func: u8) {
    let cmd = read_command(bus, dev, func);
    write_command(bus, dev, func, cmd | CMD_MEMORY_SPACE | CMD_INTX_DISABLE);
}

pub fn decode_bar0(bus: u8, dev: u8, func: u8) -> Option<(usize, usize)> {
    let bar0_raw = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x10)?;
    if bar0_raw & 0x01 != 0 {
        return None;
    }
    let is_64bit = (bar0_raw >> 1) & 0x03 == 0x02;
    let prefetchable = bar0_raw & (1 << 3) != 0;

    let mut base = (bar0_raw & 0xFFFF_FFF0) as usize;
    if prefetchable {
        base |= 0;
    }

    if is_64bit {
        let bar1_raw = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x14)?;
        base |= (bar1_raw as usize) << 32;
    }

    let size = crate::bus::pci::api::probe_bar_size(bus, dev, func, 0x10)?;
    Some((base, size))
}

pub fn find_capability(bus: u8, dev: u8, func: u8, cap_id: u8) -> u8 {
    let status = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x04).unwrap_or(0);
    if (status >> 16) & (1 << 4) == 0 {
        return 0;
    }

    let cap_ptr_raw = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x34).unwrap_or(0);
    let mut ptr = (cap_ptr_raw & 0xFC) as u8;

    let mut visited = 0u32;
    while ptr != 0 && visited < 48 {
        let cap_reg = crate::bus::pci::api::read_config_u32(bus, dev, func, ptr).unwrap_or(0);
        let this_id = (cap_reg & 0xFF) as u8;
        if this_id == cap_id {
            return ptr;
        }
        ptr = ((cap_reg >> 8) & 0xFC) as u8;
        visited += 1;
    }
    0
}

pub fn read_class(bus: u8, dev: u8, func: u8) -> (u8, u8, u8) {
    let val = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x08).unwrap_or(0);
    let class = ((val >> 24) & 0xFF) as u8;
    let subclass = ((val >> 16) & 0xFF) as u8;
    let prog_if = ((val >> 8) & 0xFF) as u8;
    (class, subclass, prog_if)
}

pub fn read_irq_line(bus: u8, dev: u8, func: u8) -> u8 {
    let val = crate::bus::pci::api::read_config_u32(bus, dev, func, 0x3C).unwrap_or(0);
    (val & 0xFF) as u8
}
