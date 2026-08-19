use crate::usb::hw;
use core::sync::atomic::{AtomicU8, Ordering};

static DRIVER_STATE: AtomicU8 = AtomicU8::new(0);

pub struct XhciDriver {
    pub base: usize,
    pub cap_length: usize,
    pub max_ports: u8,
    pub max_slots: u8,
    pub version: u16,
}

pub fn probe(base: usize) -> XhciDriver {
    let cl = hw::cap_length(base);
    let version = hw::hci_version(base);
    let ports = hw::max_ports(base);
    let slots = hw::max_device_slots(base);
    DRIVER_STATE.store(1, Ordering::Release);
    XhciDriver {
        base,
        cap_length: cl,
        max_ports: ports,
        max_slots: slots,
        version,
    }
}

pub fn init(driver: &XhciDriver) -> bool {
    hw::reset_controller(driver.base);
    hw::start(driver.base);
    DRIVER_STATE.store(2, Ordering::Release);
    true
}

pub fn state() -> u8 {
    DRIVER_STATE.load(Ordering::Acquire)
}
