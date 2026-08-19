pub fn read_reg(base: usize, offset: usize) -> u32 {
    let ptr = (base + offset) as *const u32;
    unsafe { core::ptr::read_volatile(ptr) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    let ptr = (base + offset) as *mut u32;
    unsafe { core::ptr::write_volatile(ptr, val) }
}

pub fn read_reg64(base: usize, offset: usize) -> u64 {
    let lo = read_reg(base, offset) as u64;
    let hi = read_reg(base, offset + 4) as u64;
    (hi << 32) | lo
}

pub fn write_reg64(base: usize, offset: usize, val: u64) {
    write_reg(base, offset, val as u32);
    write_reg(base, offset + 4, (val >> 32) as u32);
}

pub const AHCI_CAP: usize = 0x00;
pub const AHCI_GHC: usize = 0x04;
pub const AHCI_IS: usize = 0x08;
pub const AHCI_PI: usize = 0x0C;
pub const AHCI_VS: usize = 0x10;
pub const AHCI_PORT_BASE: usize = 0x100;
pub const AHCI_PORT_SIZE: usize = 0x80;

pub const NVME_CAP: usize = 0x00;
pub const NVME_VS: usize = 0x08;
pub const NVME_CC: usize = 0x14;
pub const NVME_CSTS: usize = 0x1C;
pub const NVME_AQA: usize = 0x24;
pub const NVME_ASQ: usize = 0x28;
pub const NVME_ACQ: usize = 0x30;

pub fn ahci_port_count(base: usize) -> u8 {
    let cap = read_reg(base, AHCI_CAP);
    ((cap & 0x1F) + 1) as u8
}

pub fn ahci_ports_implemented(base: usize) -> u32 {
    read_reg(base, AHCI_PI)
}

pub fn ahci_enable(base: usize) {
    let ghc = read_reg(base, AHCI_GHC);
    write_reg(base, AHCI_GHC, ghc | (1 << 31));
}

pub fn ahci_port_offset(port: u8) -> usize {
    AHCI_PORT_BASE + (port as usize) * AHCI_PORT_SIZE
}

pub fn nvme_version(base: usize) -> (u16, u8, u8) {
    let vs = read_reg(base, NVME_VS);
    let major = ((vs >> 16) & 0xFFFF) as u16;
    let minor = ((vs >> 8) & 0xFF) as u8;
    let ter = (vs & 0xFF) as u8;
    (major, minor, ter)
}

pub fn nvme_enable(base: usize) {
    let cc = read_reg(base, NVME_CC);
    write_reg(base, NVME_CC, cc | 1);
}

pub fn nvme_disable(base: usize) {
    let cc = read_reg(base, NVME_CC);
    write_reg(base, NVME_CC, cc & !1);
}

pub fn nvme_ready(base: usize) -> bool {
    (read_reg(base, NVME_CSTS) & 1) != 0
}
