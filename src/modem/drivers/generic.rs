use crate::modem::hw;
use crate::modem::signal;
use crate::modem::sim;
use core::sync::atomic::{AtomicU8, Ordering};

static DRIVER_STATE: AtomicU8 = AtomicU8::new(0);

pub struct ModemDriver {
    pub base: usize,
    pub modem_id: u32,
}

pub fn probe(base: usize) -> ModemDriver {
    let id = hw::modem_id(base);
    DRIVER_STATE.store(1, Ordering::Release);
    ModemDriver { base, modem_id: id }
}

pub fn init(driver: &ModemDriver) -> bool {
    hw::reset(driver.base);
    hw::enable(driver.base);
    if hw::sim_present(driver.base) {
        sim::set_state(sim::SimState::Ready);
    } else {
        sim::set_state(sim::SimState::Absent);
    }
    let raw_signal = hw::read_signal_strength(driver.base);
    let rssi = signal::rssi_from_raw(raw_signal);
    signal::update_rssi(rssi);
    let cell = hw::read_cell_id(driver.base);
    signal::update_cell_id(cell);
    DRIVER_STATE.store(2, Ordering::Release);
    true
}

pub fn send_byte(driver: &ModemDriver, byte: u8) -> bool {
    if hw::tx_ready(driver.base) {
        hw::write_tx(driver.base, byte);
        true
    } else {
        false
    }
}

pub fn recv_byte(driver: &ModemDriver) -> Option<u8> {
    if hw::rx_available(driver.base) {
        Some(hw::read_rx(driver.base))
    } else {
        None
    }
}

pub fn shutdown(driver: &ModemDriver) {
    hw::disable(driver.base);
    DRIVER_STATE.store(0, Ordering::Release);
}

pub fn state() -> u8 {
    DRIVER_STATE.load(Ordering::Acquire)
}
