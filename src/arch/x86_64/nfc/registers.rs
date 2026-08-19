pub const REG_CTRL: u32 = 0x00;
pub const REG_STATUS: u32 = 0x04;
pub const REG_VERSION: u32 = 0x08;
pub const REG_RF_CTRL: u32 = 0x10;
pub const REG_RF_STATUS: u32 = 0x14;
pub const REG_RF_FIELD: u32 = 0x18;
pub const REG_TAG_TYPE: u32 = 0x20;
pub const REG_TAG_DATA: u32 = 0x24;
pub const REG_TAG_LENGTH: u32 = 0x28;
pub const REG_TAG_STATUS: u32 = 0x2C;
pub const REG_TX_DATA: u32 = 0x30;
pub const REG_TX_LENGTH: u32 = 0x34;
pub const REG_TX_CTRL: u32 = 0x38;
pub const REG_RX_DATA: u32 = 0x40;
pub const REG_RX_LENGTH: u32 = 0x44;
pub const REG_RX_STATUS: u32 = 0x48;
pub const REG_IRQ_STATUS: u32 = 0x60;
pub const REG_IRQ_MASK: u32 = 0x64;

pub const CTRL_ENABLE: u32 = 1 << 0;
pub const CTRL_RESET: u32 = 1 << 1;
pub const CTRL_RF_EN: u32 = 1 << 2;
pub const CTRL_POLL_EN: u32 = 1 << 3;

fn read_reg(base: usize, offset: u32) -> u32 {
    unsafe { super::super::mmio::mmio_read32(base + offset as usize) }
}

fn write_reg(base: usize, offset: u32, val: u32) {
    unsafe { super::super::mmio::mmio_write32(base + offset as usize, val) }
}

pub fn reset(base: usize) {
    write_reg(base, REG_CTRL, CTRL_RESET);
    for _ in 0..1000 {
        if read_reg(base, REG_CTRL) & CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn enable(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_ENABLE);
}

pub fn enable_rf_field(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_RF_EN);
}

pub fn disable_rf_field(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val & !CTRL_RF_EN);
}

pub fn rf_field_detected(base: usize) -> bool {
    read_reg(base, REG_RF_FIELD) & 0x01 != 0
}

pub fn enable_polling(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_POLL_EN);
}

pub fn tag_present(base: usize) -> bool {
    read_reg(base, REG_TAG_STATUS) & 0x01 != 0
}

pub fn read_tag_type(base: usize) -> u32 {
    read_reg(base, REG_TAG_TYPE)
}

pub fn read_tag_data(base: usize) -> u32 {
    read_reg(base, REG_TAG_DATA)
}

pub fn read_tag_length(base: usize) -> u32 {
    read_reg(base, REG_TAG_LENGTH)
}

pub fn transmit(base: usize, data: u32, length: u32) {
    write_reg(base, REG_TX_DATA, data);
    write_reg(base, REG_TX_LENGTH, length);
    write_reg(base, REG_TX_CTRL, 0x01);
}

pub fn rx_available(base: usize) -> bool {
    read_reg(base, REG_RX_STATUS) & 0x01 != 0
}

pub fn read_rx_data(base: usize) -> u32 {
    read_reg(base, REG_RX_DATA)
}

pub fn read_rx_length(base: usize) -> u32 {
    read_reg(base, REG_RX_LENGTH)
}

pub fn read_irq_status(base: usize) -> u32 {
    read_reg(base, REG_IRQ_STATUS)
}

pub fn clear_irq(base: usize, bits: u32) {
    write_reg(base, REG_IRQ_STATUS, bits);
}

pub fn read_version(base: usize) -> u32 {
    read_reg(base, REG_VERSION)
}
