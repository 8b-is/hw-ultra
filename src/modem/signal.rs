use core::sync::atomic::{AtomicI16, AtomicU32, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum SignalQuality {
    NoSignal,
    Poor,
    Fair,
    Good,
    Excellent,
}

static RSSI_DBM: AtomicI16 = AtomicI16::new(-120);
static CELL_ID: AtomicU32 = AtomicU32::new(0);

pub fn rssi_to_quality(rssi_dbm: i16) -> SignalQuality {
    if rssi_dbm <= -110 {
        SignalQuality::NoSignal
    } else if rssi_dbm <= -100 {
        SignalQuality::Poor
    } else if rssi_dbm <= -85 {
        SignalQuality::Fair
    } else if rssi_dbm <= -70 {
        SignalQuality::Good
    } else {
        SignalQuality::Excellent
    }
}

pub fn rssi_from_raw(raw: u32) -> i16 {
    if raw == 99 || raw == 0 {
        return -120;
    }
    -113 + (raw as i16 * 2)
}

pub fn bars(rssi_dbm: i16) -> u8 {
    match rssi_to_quality(rssi_dbm) {
        SignalQuality::NoSignal => 0,
        SignalQuality::Poor => 1,
        SignalQuality::Fair => 2,
        SignalQuality::Good => 3,
        SignalQuality::Excellent => 4,
    }
}

pub fn update_rssi(dbm: i16) {
    RSSI_DBM.store(dbm, Ordering::Release);
}

pub fn current_rssi() -> i16 {
    RSSI_DBM.load(Ordering::Acquire)
}

pub fn update_cell_id(id: u32) {
    CELL_ID.store(id, Ordering::Release);
}

pub fn current_cell_id() -> u32 {
    CELL_ID.load(Ordering::Acquire)
}

pub fn quality() -> SignalQuality {
    rssi_to_quality(current_rssi())
}
