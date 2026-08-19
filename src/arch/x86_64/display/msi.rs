const MSI_ADDR_BASE: u32 = 0xFEE0_0000;
pub const MSI_CTRL_OFFSET: u8 = 0x02;
const MSI_ADDR_LOW_OFFSET: u8 = 0x04;
const MSI_ADDR_HIGH_OFFSET: u8 = 0x08;
const MSI_DATA_32_OFFSET: u8 = 0x08;
const MSI_DATA_64_OFFSET: u8 = 0x0C;

const MSI_CTRL_ENABLE: u16 = 1 << 0;
const MSI_CTRL_64BIT: u16 = 1 << 7;

pub fn read_msi_control(bus: u8, dev: u8, func: u8, cap_offset: u8) -> u16 {
    let val = crate::bus::pci::api::read_config_u32(bus, dev, func, cap_offset).unwrap_or(0);
    ((val >> 16) & 0xFFFF) as u16
}

pub fn is_64bit(bus: u8, dev: u8, func: u8, cap_offset: u8) -> bool {
    read_msi_control(bus, dev, func, cap_offset) & MSI_CTRL_64BIT != 0
}

pub fn program_msi(bus: u8, dev: u8, func: u8, cap_offset: u8, vector: u8, dest_apic: u8) {
    let addr_low = MSI_ADDR_BASE | ((dest_apic as u32) << 12);

    crate::bus::pci::api::write_config_u32(
        bus,
        dev,
        func,
        cap_offset + MSI_ADDR_LOW_OFFSET,
        addr_low,
    );

    let data = vector as u32;

    if is_64bit(bus, dev, func, cap_offset) {
        crate::bus::pci::api::write_config_u32(
            bus,
            dev,
            func,
            cap_offset + MSI_ADDR_HIGH_OFFSET,
            0,
        );
        crate::bus::pci::api::write_config_u32(
            bus,
            dev,
            func,
            cap_offset + MSI_DATA_64_OFFSET,
            data,
        );
    } else {
        crate::bus::pci::api::write_config_u32(
            bus,
            dev,
            func,
            cap_offset + MSI_DATA_32_OFFSET,
            data,
        );
    }
}

pub fn enable_msi(bus: u8, dev: u8, func: u8, cap_offset: u8) {
    let cap_dword = crate::bus::pci::api::read_config_u32(bus, dev, func, cap_offset).unwrap_or(0);
    let ctrl = ((cap_dword >> 16) & 0xFFFF) as u16;
    let new_ctrl = ctrl | MSI_CTRL_ENABLE;
    let new_dword = (cap_dword & 0x0000_FFFF) | ((new_ctrl as u32) << 16);
    crate::bus::pci::api::write_config_u32(bus, dev, func, cap_offset, new_dword);
}

pub fn disable_msi(bus: u8, dev: u8, func: u8, cap_offset: u8) {
    let cap_dword = crate::bus::pci::api::read_config_u32(bus, dev, func, cap_offset).unwrap_or(0);
    let ctrl = ((cap_dword >> 16) & 0xFFFF) as u16;
    let new_ctrl = ctrl & !MSI_CTRL_ENABLE;
    let new_dword = (cap_dword & 0x0000_FFFF) | ((new_ctrl as u32) << 16);
    crate::bus::pci::api::write_config_u32(bus, dev, func, cap_offset, new_dword);
}

pub fn allocated_vectors(bus: u8, dev: u8, func: u8, cap_offset: u8) -> usize {
    let ctrl = read_msi_control(bus, dev, func, cap_offset);
    let mme = (ctrl >> 4) & 0x07;
    1usize << mme
}
