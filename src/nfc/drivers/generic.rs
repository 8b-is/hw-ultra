use crate::nfc::hw;
use crate::nfc::protocol;
use core::sync::atomic::{AtomicU8, Ordering};

static DRIVER_STATE: AtomicU8 = AtomicU8::new(0);

pub struct NfcDriver {
    pub base: usize,
    pub chip_id: u32,
    pub protocol: protocol::NfcProtocol,
}

pub fn probe(base: usize) -> NfcDriver {
    let id = hw::nfc_id(base);
    DRIVER_STATE.store(1, Ordering::Release);
    NfcDriver {
        base,
        chip_id: id,
        protocol: protocol::NfcProtocol::NfcA,
    }
}

pub fn init(driver: &NfcDriver) -> bool {
    hw::reset(driver.base);
    hw::enable(driver.base);
    let bitrate = protocol::protocol_bitrate(driver.protocol);
    hw::set_bitrate(driver.base, bitrate);
    let modulation = protocol::modulation_code(driver.protocol);
    hw::set_modulation(driver.base, modulation);
    DRIVER_STATE.store(2, Ordering::Release);
    true
}

pub fn send_bytes(driver: &NfcDriver, data: &[u8]) -> usize {
    let mut sent = 0;
    let mut i = 0;
    while i < data.len() {
        if hw::tx_busy(driver.base) {
            break;
        }
        hw::write_tx(driver.base, data[i]);
        sent += 1;
        i += 1;
    }
    sent
}

pub fn recv_bytes(driver: &NfcDriver, buf: &mut [u8]) -> usize {
    let mut received = 0;
    while received < buf.len() {
        if !hw::rx_ready(driver.base) {
            break;
        }
        buf[received] = hw::read_rx(driver.base);
        received += 1;
    }
    received
}

pub fn shutdown(driver: &NfcDriver) {
    hw::rf_field_off(driver.base);
    hw::disable(driver.base);
    DRIVER_STATE.store(0, Ordering::Release);
}

pub fn state() -> u8 {
    DRIVER_STATE.load(Ordering::Acquire)
}
