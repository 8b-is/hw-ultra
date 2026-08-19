pub const REG_CTRL: u32 = 0x00;
pub const REG_STATUS: u32 = 0x04;
pub const REG_VERSION: u32 = 0x08;
pub const REG_LINK_STATUS: u32 = 0x0C;
pub const REG_SIGNAL_STRENGTH: u32 = 0x10;
pub const REG_NETWORK_MODE: u32 = 0x14;
pub const REG_BAND_SELECT: u32 = 0x18;
pub const REG_TX_CTRL: u32 = 0x20;
pub const REG_TX_DATA: u32 = 0x24;
pub const REG_TX_STATUS: u32 = 0x28;
pub const REG_RX_CTRL: u32 = 0x30;
pub const REG_RX_DATA: u32 = 0x34;
pub const REG_RX_STATUS: u32 = 0x38;
pub const REG_DMA_TX_ADDR_LO: u32 = 0x40;
pub const REG_DMA_TX_ADDR_HI: u32 = 0x44;
pub const REG_DMA_TX_LEN: u32 = 0x48;
pub const REG_DMA_RX_ADDR_LO: u32 = 0x50;
pub const REG_DMA_RX_ADDR_HI: u32 = 0x54;
pub const REG_DMA_RX_LEN: u32 = 0x58;
pub const REG_DMA_CTRL: u32 = 0x5C;
pub const REG_IRQ_STATUS: u32 = 0x60;
pub const REG_IRQ_MASK: u32 = 0x64;

pub const CTRL_ENABLE: u32 = 1 << 0;
pub const CTRL_RESET: u32 = 1 << 1;
pub const CTRL_TX_EN: u32 = 1 << 2;
pub const CTRL_RX_EN: u32 = 1 << 3;
pub const CTRL_DMA_EN: u32 = 1 << 4;

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

pub fn set_network_mode(base: usize, mode: u32) {
    write_reg(base, REG_NETWORK_MODE, mode);
}

pub fn set_band(base: usize, band: u32) {
    write_reg(base, REG_BAND_SELECT, band);
}

pub fn read_signal_strength(base: usize) -> u32 {
    read_reg(base, REG_SIGNAL_STRENGTH)
}

pub fn read_link_status(base: usize) -> u32 {
    read_reg(base, REG_LINK_STATUS)
}

pub fn link_up(base: usize) -> bool {
    read_reg(base, REG_LINK_STATUS) & 0x01 != 0
}

pub fn enable_tx(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_TX_EN);
}

pub fn enable_rx(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_RX_EN);
}

pub fn tx_ready(base: usize) -> bool {
    read_reg(base, REG_TX_STATUS) & 0x01 != 0
}

pub fn rx_available(base: usize) -> bool {
    read_reg(base, REG_RX_STATUS) & 0x01 != 0
}

pub fn setup_tx_dma(base: usize, addr: u64, len: u32) {
    write_reg(base, REG_DMA_TX_ADDR_LO, addr as u32);
    write_reg(base, REG_DMA_TX_ADDR_HI, (addr >> 32) as u32);
    write_reg(base, REG_DMA_TX_LEN, len);
}

pub fn setup_rx_dma(base: usize, addr: u64, len: u32) {
    write_reg(base, REG_DMA_RX_ADDR_LO, addr as u32);
    write_reg(base, REG_DMA_RX_ADDR_HI, (addr >> 32) as u32);
    write_reg(base, REG_DMA_RX_LEN, len);
}

pub fn enable_dma(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_DMA_EN);
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
