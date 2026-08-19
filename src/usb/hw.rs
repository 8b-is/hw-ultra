use core::ptr;

pub const CAPLENGTH: usize = 0x00;
pub const HCIVERSION: usize = 0x02;
pub const HCSPARAMS1: usize = 0x04;
pub const HCSPARAMS2: usize = 0x08;
pub const HCSPARAMS3: usize = 0x0C;
pub const HCCPARAMS1: usize = 0x10;
pub const DBOFF: usize = 0x14;
pub const RTSOFF: usize = 0x18;
pub const HCCPARAMS2: usize = 0x1C;

pub const USBCMD: usize = 0x00;
pub const USBSTS: usize = 0x04;
pub const PAGESIZE: usize = 0x08;
pub const DNCTRL: usize = 0x14;
pub const CRCR: usize = 0x18;
pub const DCBAAP: usize = 0x30;
pub const CONFIG: usize = 0x38;

pub const PORTSC_BASE: usize = 0x400;
pub const PORTSC_STRIDE: usize = 0x10;

pub const USBCMD_RUN: u32 = 1 << 0;
pub const USBCMD_HCRST: u32 = 1 << 1;
pub const USBCMD_INTE: u32 = 1 << 2;

pub const USBSTS_HCH: u32 = 1 << 0;
pub const USBSTS_HSE: u32 = 1 << 2;
pub const USBSTS_EINT: u32 = 1 << 3;
pub const USBSTS_PCD: u32 = 1 << 4;
pub const USBSTS_CNR: u32 = 1 << 11;

pub const PORTSC_CCS: u32 = 1 << 0;
pub const PORTSC_PED: u32 = 1 << 1;
pub const PORTSC_PR: u32 = 1 << 4;
pub const PORTSC_PP: u32 = 1 << 9;
pub const PORTSC_SPEED_MASK: u32 = 0xF << 10;
pub const PORTSC_SPEED_SHIFT: u32 = 10;

pub fn read_cap(base: usize, offset: usize) -> u32 {
    unsafe { ptr::read_volatile((base + offset) as *const u32) }
}

pub fn read_op(base: usize, cap_length: usize, offset: usize) -> u32 {
    unsafe { ptr::read_volatile((base + cap_length + offset) as *const u32) }
}

pub fn write_op(base: usize, cap_length: usize, offset: usize, val: u32) {
    unsafe { ptr::write_volatile((base + cap_length + offset) as *mut u32, val) }
}

pub fn cap_length(base: usize) -> usize {
    (read_cap(base, CAPLENGTH) & 0xFF) as usize
}

pub fn hci_version(base: usize) -> u16 {
    (read_cap(base, HCIVERSION) >> 16) as u16
}

pub fn max_ports(base: usize) -> u8 {
    let params = read_cap(base, HCSPARAMS1);
    ((params >> 24) & 0xFF) as u8
}

pub fn max_device_slots(base: usize) -> u8 {
    let params = read_cap(base, HCSPARAMS1);
    (params & 0xFF) as u8
}

pub fn max_interrupters(base: usize) -> u16 {
    let params = read_cap(base, HCSPARAMS1);
    ((params >> 8) & 0x7FF) as u16
}

pub fn reset_controller(base: usize) {
    let cl = cap_length(base);
    write_op(base, cl, USBCMD, USBCMD_HCRST);
    loop {
        let cmd = read_op(base, cl, USBCMD);
        if cmd & USBCMD_HCRST == 0 {
            break;
        }
    }
    loop {
        let sts = read_op(base, cl, USBSTS);
        if sts & USBSTS_CNR == 0 {
            break;
        }
    }
}

pub fn start(base: usize) {
    let cl = cap_length(base);
    let cmd = read_op(base, cl, USBCMD);
    write_op(base, cl, USBCMD, cmd | USBCMD_RUN | USBCMD_INTE);
}

pub fn stop(base: usize) {
    let cl = cap_length(base);
    let cmd = read_op(base, cl, USBCMD);
    write_op(base, cl, USBCMD, cmd & !USBCMD_RUN);
    loop {
        let sts = read_op(base, cl, USBSTS);
        if sts & USBSTS_HCH != 0 {
            break;
        }
    }
}

pub fn port_status(base: usize, cap_length: usize, port: u8) -> u32 {
    let offset = PORTSC_BASE + (port as usize) * PORTSC_STRIDE;
    read_op(base, cap_length, offset)
}

pub fn port_connected(base: usize, cap_length: usize, port: u8) -> bool {
    port_status(base, cap_length, port) & PORTSC_CCS != 0
}

pub fn port_speed(base: usize, cap_length: usize, port: u8) -> u8 {
    let sc = port_status(base, cap_length, port);
    ((sc & PORTSC_SPEED_MASK) >> PORTSC_SPEED_SHIFT) as u8
}

pub fn port_reset(base: usize, cap_length: usize, port: u8) {
    let offset = PORTSC_BASE + (port as usize) * PORTSC_STRIDE;
    let sc = read_op(base, cap_length, offset);
    write_op(base, cap_length, offset, sc | PORTSC_PR);
}
