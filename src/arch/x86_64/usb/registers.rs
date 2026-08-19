pub const REG_CAPLENGTH: u32 = 0x00;
pub const REG_HCSPARAMS1: u32 = 0x04;
pub const REG_HCSPARAMS2: u32 = 0x08;
pub const REG_HCSPARAMS3: u32 = 0x0C;
pub const REG_HCCPARAMS1: u32 = 0x10;
pub const REG_DBOFF: u32 = 0x14;
pub const REG_RTSOFF: u32 = 0x18;
pub const REG_HCCPARAMS2: u32 = 0x1C;

pub const REG_USBCMD: u32 = 0x00;
pub const REG_USBSTS: u32 = 0x04;
pub const REG_PAGESIZE: u32 = 0x08;
pub const REG_DNCTRL: u32 = 0x14;
pub const REG_CRCR: u32 = 0x18;
pub const REG_DCBAAP: u32 = 0x30;
pub const REG_CONFIG: u32 = 0x38;

pub const USBCMD_RUN: u32 = 1 << 0;
pub const USBCMD_HCRST: u32 = 1 << 1;
pub const USBCMD_INTE: u32 = 1 << 2;
pub const USBCMD_HSEE: u32 = 1 << 3;

pub const USBSTS_HCH: u32 = 1 << 0;
pub const USBSTS_HSE: u32 = 1 << 2;
pub const USBSTS_EINT: u32 = 1 << 3;
pub const USBSTS_PCD: u32 = 1 << 4;
pub const USBSTS_CNR: u32 = 1 << 11;

fn read_reg(base: usize, offset: u32) -> u32 {
    unsafe { super::super::mmio::mmio_read32(base + offset as usize) }
}

fn write_reg(base: usize, offset: u32, val: u32) {
    unsafe { super::super::mmio::mmio_write32(base + offset as usize, val) }
}

pub fn read_cap_length(base: usize) -> u8 {
    (read_reg(base, REG_CAPLENGTH) & 0xFF) as u8
}

pub fn operational_base(base: usize) -> usize {
    base + read_cap_length(base) as usize
}

pub fn read_hcsparams1(base: usize) -> (u8, u16, u8) {
    let val = read_reg(base, REG_HCSPARAMS1);
    let max_slots = (val & 0xFF) as u8;
    let max_intrs = ((val >> 8) & 0x7FF) as u16;
    let max_ports = ((val >> 24) & 0xFF) as u8;
    (max_slots, max_intrs, max_ports)
}

pub fn read_usbsts(base: usize) -> u32 {
    let op = operational_base(base);
    read_reg(op, REG_USBSTS)
}

pub fn controller_ready(base: usize) -> bool {
    read_usbsts(base) & USBSTS_CNR == 0
}

pub fn reset(base: usize) {
    let op = operational_base(base);
    let cmd = read_reg(op, REG_USBCMD);
    write_reg(op, REG_USBCMD, cmd & !USBCMD_RUN);
    for _ in 0..1000 {
        if read_reg(op, REG_USBSTS) & USBSTS_HCH != 0 {
            break;
        }
    }
    write_reg(op, REG_USBCMD, USBCMD_HCRST);
    for _ in 0..1000 {
        if read_reg(op, REG_USBCMD) & USBCMD_HCRST == 0 {
            break;
        }
    }
}

pub fn enable(base: usize) {
    let op = operational_base(base);
    for _ in 0..1000 {
        if read_reg(op, REG_USBSTS) & USBSTS_CNR == 0 {
            break;
        }
    }
    let cmd = read_reg(op, REG_USBCMD);
    write_reg(op, REG_USBCMD, cmd | USBCMD_RUN | USBCMD_INTE);
}

pub fn set_max_slots(base: usize, slots: u8) {
    let op = operational_base(base);
    write_reg(op, REG_CONFIG, slots as u32);
}

pub fn set_dcbaap(base: usize, addr: u64) {
    let op = operational_base(base);
    write_reg(op, REG_DCBAAP, addr as u32);
    write_reg(op, REG_DCBAAP + 4, (addr >> 32) as u32);
}

pub fn set_command_ring(base: usize, addr: u64) {
    let op = operational_base(base);
    write_reg(op, REG_CRCR, (addr as u32) | 0x01);
    write_reg(op, REG_CRCR + 4, (addr >> 32) as u32);
}

pub fn read_doorbell_offset(base: usize) -> u32 {
    read_reg(base, REG_DBOFF) & 0xFFFF_FFFC
}

pub fn ring_doorbell(base: usize, slot: u8, target: u8) {
    let db_off = read_doorbell_offset(base);
    let db_addr = base + db_off as usize + (slot as usize) * 4;
    unsafe { super::super::mmio::mmio_write32(db_addr, target as u32) }
}

pub fn read_version(base: usize) -> u32 {
    let raw = read_reg(base, REG_CAPLENGTH);
    (raw >> 16) & 0xFFFF
}

pub fn read_page_size(base: usize) -> u32 {
    let op = operational_base(base);
    let val = read_reg(op, REG_PAGESIZE) & 0xFFFF;
    val << 12
}
