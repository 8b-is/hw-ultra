use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_ZONES: usize = 8;

static ZONE_TEMPS: [AtomicUsize; MAX_ZONES] = [const { AtomicUsize::new(0) }; MAX_ZONES];

static ZONE_COUNT: AtomicUsize = AtomicUsize::new(0);

fn read_sysfs_temp(zone: usize) -> u32 {
    let raw = ZONE_TEMPS[zone].load(Ordering::Acquire);
    raw as u32
}

pub fn probe_zones() -> usize {
    ZONE_COUNT.load(Ordering::Acquire)
}

pub fn zone_count() -> usize {
    let c = ZONE_COUNT.load(Ordering::Acquire);
    if c == 0 {
        return probe_zones();
    }
    c
}

pub fn read_thermal_zone(zone: usize) -> Option<u32> {
    if zone >= zone_count() {
        return None;
    }
    let live = read_sysfs_temp(zone);
    if live > 0 {
        ZONE_TEMPS[zone].store(live as usize, Ordering::Release);
        return Some(live);
    }
    let raw = ZONE_TEMPS[zone].load(Ordering::Acquire);
    if raw == 0 {
        None
    } else {
        Some(raw as u32)
    }
}

pub fn register_zone(zone: usize, temp_millideg: u32) -> bool {
    if zone >= MAX_ZONES {
        return false;
    }
    ZONE_TEMPS[zone].store(temp_millideg as usize, Ordering::Release);
    let count = ZONE_COUNT.load(Ordering::Acquire);
    if zone >= count {
        ZONE_COUNT.store(zone + 1, Ordering::Release);
    }
    true
}
