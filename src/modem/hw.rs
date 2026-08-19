use core::ptr;

pub const MODEM_ID: usize = 0x00;
pub const MODEM_STATUS: usize = 0x04;
pub const MODEM_CTRL: usize = 0x08;
pub const MODEM_INT_CTRL: usize = 0x0C;
pub const MODEM_INT_STATUS: usize = 0x10;
pub const MODEM_TX_DATA: usize = 0x14;
pub const MODEM_RX_DATA: usize = 0x18;
pub const MODEM_TX_STATUS: usize = 0x1C;
pub const MODEM_RX_STATUS: usize = 0x20;
pub const MODEM_BAUD_RATE: usize = 0x24;
pub const MODEM_SIGNAL: usize = 0x28;
pub const MODEM_CELL_ID: usize = 0x2C;
pub const MODEM_NETWORK_STATUS: usize = 0x30;

pub const CTRL_ENABLE: u32 = 1 << 0;
pub const CTRL_RESET: u32 = 1 << 1;
pub const CTRL_RADIO_ON: u32 = 1 << 2;
pub const CTRL_AIRPLANE: u32 = 1 << 3;
pub const CTRL_TX_EN: u32 = 1 << 4;
pub const CTRL_RX_EN: u32 = 1 << 5;

pub const STATUS_POWERED: u32 = 1 << 0;
pub const STATUS_REGISTERED: u32 = 1 << 1;
pub const STATUS_DATA_READY: u32 = 1 << 2;
pub const STATUS_SIM_PRESENT: u32 = 1 << 3;
pub const STATUS_ROAMING: u32 = 1 << 4;

pub const TX_READY: u32 = 1 << 0;
pub const RX_AVAILABLE: u32 = 1 << 0;

pub fn read_reg(base: usize, offset: usize) -> u32 {
    unsafe { ptr::read_volatile((base + offset) as *const u32) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    unsafe { ptr::write_volatile((base + offset) as *mut u32, val) }
}

pub fn modem_id(base: usize) -> u32 {
    read_reg(base, MODEM_ID)
}

pub fn enable(base: usize) {
    let ctrl = read_reg(base, MODEM_CTRL);
    write_reg(base, MODEM_CTRL, ctrl | CTRL_ENABLE | CTRL_RADIO_ON);
}

pub fn disable(base: usize) {
    let ctrl = read_reg(base, MODEM_CTRL);
    write_reg(base, MODEM_CTRL, ctrl & !(CTRL_ENABLE | CTRL_RADIO_ON));
}

pub fn reset(base: usize) {
    write_reg(base, MODEM_CTRL, CTRL_RESET);
    loop {
        let ctrl = read_reg(base, MODEM_CTRL);
        if ctrl & CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn set_airplane_mode(base: usize, on: bool) {
    let ctrl = read_reg(base, MODEM_CTRL);
    if on {
        write_reg(base, MODEM_CTRL, (ctrl | CTRL_AIRPLANE) & !CTRL_RADIO_ON);
    } else {
        write_reg(base, MODEM_CTRL, (ctrl & !CTRL_AIRPLANE) | CTRL_RADIO_ON);
    }
}

pub fn is_registered(base: usize) -> bool {
    read_reg(base, MODEM_STATUS) & STATUS_REGISTERED != 0
}

pub fn sim_present(base: usize) -> bool {
    read_reg(base, MODEM_STATUS) & STATUS_SIM_PRESENT != 0
}

pub fn is_roaming(base: usize) -> bool {
    read_reg(base, MODEM_STATUS) & STATUS_ROAMING != 0
}

pub fn tx_ready(base: usize) -> bool {
    read_reg(base, MODEM_TX_STATUS) & TX_READY != 0
}

pub fn rx_available(base: usize) -> bool {
    read_reg(base, MODEM_RX_STATUS) & RX_AVAILABLE != 0
}

pub fn write_tx(base: usize, byte: u8) {
    write_reg(base, MODEM_TX_DATA, byte as u32);
}

pub fn read_rx(base: usize) -> u8 {
    read_reg(base, MODEM_RX_DATA) as u8
}

pub fn read_signal_strength(base: usize) -> u32 {
    read_reg(base, MODEM_SIGNAL)
}

pub fn read_cell_id(base: usize) -> u32 {
    read_reg(base, MODEM_CELL_ID)
}
