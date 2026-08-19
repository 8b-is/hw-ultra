use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_DEVICES: usize = 64;

struct BusDevice {
    vendor_id: AtomicUsize,
    device_id: AtomicUsize,
    bus_type: AtomicUsize,
    class: AtomicUsize,
}

struct DeviceRegistry {
    devices: [BusDevice; MAX_DEVICES],
    count: AtomicUsize,
}

unsafe impl Sync for DeviceRegistry {}

static REGISTRY: DeviceRegistry = DeviceRegistry {
    devices: [const {
        BusDevice {
            vendor_id: AtomicUsize::new(0),
            device_id: AtomicUsize::new(0),
            bus_type: AtomicUsize::new(0),
            class: AtomicUsize::new(0),
        }
    }; MAX_DEVICES],
    count: AtomicUsize::new(0),
};

pub fn scan_sysfs_devices() -> usize {
    REGISTRY.count.load(Ordering::Acquire)
}

pub fn register_device(vendor: u16, device: u16, bus: u8) -> bool {
    let idx = REGISTRY.count.load(Ordering::Acquire);
    if idx >= MAX_DEVICES {
        return false;
    }
    REGISTRY.devices[idx]
        .vendor_id
        .store(vendor as usize, Ordering::Release);
    REGISTRY.devices[idx]
        .device_id
        .store(device as usize, Ordering::Release);
    REGISTRY.devices[idx]
        .bus_type
        .store(bus as usize, Ordering::Release);
    REGISTRY.count.store(idx + 1, Ordering::Release);
    true
}

pub fn device_count() -> usize {
    let c = REGISTRY.count.load(Ordering::Acquire);
    if c == 0 {
        return scan_sysfs_devices();
    }
    c
}

pub fn device_vendor(index: usize) -> Option<u16> {
    if index >= device_count() {
        return None;
    }
    Some(REGISTRY.devices[index].vendor_id.load(Ordering::Acquire) as u16)
}

pub fn device_id(index: usize) -> Option<u16> {
    if index >= device_count() {
        return None;
    }
    Some(REGISTRY.devices[index].device_id.load(Ordering::Acquire) as u16)
}

pub fn device_class(index: usize) -> Option<u16> {
    if index >= device_count() {
        return None;
    }
    Some(REGISTRY.devices[index].class.load(Ordering::Acquire) as u16)
}
