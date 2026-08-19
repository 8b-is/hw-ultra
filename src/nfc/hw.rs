use core::ptr;

pub const NFC_ID: usize = 0x00;
pub const NFC_STATUS: usize = 0x04;
pub const NFC_CTRL: usize = 0x08;
pub const NFC_INT_CTRL: usize = 0x0C;
pub const NFC_INT_STATUS: usize = 0x10;
pub const NFC_TX_DATA: usize = 0x14;
pub const NFC_RX_DATA: usize = 0x18;
pub const NFC_TX_STATUS: usize = 0x1C;
pub const NFC_RX_STATUS: usize = 0x20;
pub const NFC_RF_CTRL: usize = 0x24;
pub const NFC_BITRATE: usize = 0x28;
pub const NFC_MODULATION: usize = 0x2C;
pub const NFC_FIELD_DETECT: usize = 0x30;
pub const NFC_TARGET_ID: usize = 0x34;

pub const CTRL_ENABLE: u32 = 1 << 0;
pub const CTRL_RESET: u32 = 1 << 1;
pub const CTRL_RF_ON: u32 = 1 << 2;
pub const CTRL_POLL: u32 = 1 << 3;
pub const CTRL_LISTEN: u32 = 1 << 4;
pub const CTRL_P2P: u32 = 1 << 5;

pub const STATUS_POWERED: u32 = 1 << 0;
pub const STATUS_RF_ACTIVE: u32 = 1 << 1;
pub const STATUS_TARGET_PRESENT: u32 = 1 << 2;
pub const STATUS_TX_BUSY: u32 = 1 << 3;
pub const STATUS_RX_READY: u32 = 1 << 4;

pub fn read_reg(base: usize, offset: usize) -> u32 {
    unsafe { ptr::read_volatile((base + offset) as *const u32) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    unsafe { ptr::write_volatile((base + offset) as *mut u32, val) }
}

pub fn nfc_id(base: usize) -> u32 {
    read_reg(base, NFC_ID)
}

pub fn enable(base: usize) {
    let ctrl = read_reg(base, NFC_CTRL);
    write_reg(base, NFC_CTRL, ctrl | CTRL_ENABLE);
}

pub fn disable(base: usize) {
    let ctrl = read_reg(base, NFC_CTRL);
    write_reg(base, NFC_CTRL, ctrl & !CTRL_ENABLE);
}

pub fn reset(base: usize) {
    write_reg(base, NFC_CTRL, CTRL_RESET);
    loop {
        let ctrl = read_reg(base, NFC_CTRL);
        if ctrl & CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn rf_field_on(base: usize) {
    let ctrl = read_reg(base, NFC_CTRL);
    write_reg(base, NFC_CTRL, ctrl | CTRL_RF_ON);
}

pub fn rf_field_off(base: usize) {
    let ctrl = read_reg(base, NFC_CTRL);
    write_reg(base, NFC_CTRL, ctrl & !CTRL_RF_ON);
}

pub fn start_poll(base: usize) {
    let ctrl = read_reg(base, NFC_CTRL);
    write_reg(base, NFC_CTRL, ctrl | CTRL_POLL);
}

pub fn stop_poll(base: usize) {
    let ctrl = read_reg(base, NFC_CTRL);
    write_reg(base, NFC_CTRL, ctrl & !CTRL_POLL);
}

pub fn target_present(base: usize) -> bool {
    read_reg(base, NFC_STATUS) & STATUS_TARGET_PRESENT != 0
}

pub fn rf_active(base: usize) -> bool {
    read_reg(base, NFC_STATUS) & STATUS_RF_ACTIVE != 0
}

pub fn tx_busy(base: usize) -> bool {
    read_reg(base, NFC_STATUS) & STATUS_TX_BUSY != 0
}

pub fn rx_ready(base: usize) -> bool {
    read_reg(base, NFC_STATUS) & STATUS_RX_READY != 0
}

pub fn write_tx(base: usize, byte: u8) {
    write_reg(base, NFC_TX_DATA, byte as u32);
}

pub fn read_rx(base: usize) -> u8 {
    read_reg(base, NFC_RX_DATA) as u8
}

pub fn set_bitrate(base: usize, rate: u32) {
    write_reg(base, NFC_BITRATE, rate);
}

pub fn set_modulation(base: usize, mod_type: u32) {
    write_reg(base, NFC_MODULATION, mod_type);
}

pub fn field_detected(base: usize) -> bool {
    read_reg(base, NFC_FIELD_DETECT) & 0x01 != 0
}

pub fn read_target_id(base: usize) -> u32 {
    read_reg(base, NFC_TARGET_ID)
}
