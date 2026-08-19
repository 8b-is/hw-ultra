use crate::camera::hw;
use core::sync::atomic::{AtomicU8, Ordering};

static DRIVER_STATE: AtomicU8 = AtomicU8::new(0);

pub struct CsiDriver {
    pub base: usize,
    pub lanes: u32,
    pub version: u32,
}

pub fn probe(base: usize, lanes: u32) -> CsiDriver {
    let version = hw::csi2_version(base);
    DRIVER_STATE.store(1, Ordering::Release);
    CsiDriver {
        base,
        lanes,
        version,
    }
}

pub fn init(driver: &CsiDriver) -> bool {
    hw::csi2_set_lanes(driver.base, driver.lanes);
    hw::csi2_phy_enable(driver.base);
    let state = hw::csi2_phy_state(driver.base);
    let ready = state & 0x01 != 0;
    if ready {
        DRIVER_STATE.store(2, Ordering::Release);
    }
    ready
}

pub fn shutdown(driver: &CsiDriver) {
    hw::csi2_phy_disable(driver.base);
    DRIVER_STATE.store(0, Ordering::Release);
}

pub fn state() -> u8 {
    DRIVER_STATE.load(Ordering::Acquire)
}
