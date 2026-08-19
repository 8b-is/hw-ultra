use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const MAX_DISCOVERED: usize = 32;

static DEVICE_COUNT: AtomicU8 = AtomicU8::new(0);
static DEV_TYPE: [AtomicU8; MAX_DISCOVERED] = [const { AtomicU8::new(0) }; MAX_DISCOVERED];
static DEV_BASE: [AtomicUsize; MAX_DISCOVERED] = [const { AtomicUsize::new(0) }; MAX_DISCOVERED];

#[derive(Copy, Clone, PartialEq)]
pub enum DeviceType {
    Unknown,
    Cpu,
    Gpu,
    Tpu,
    Lpu,
    Network,
    Storage,
    Iommu,
}

#[derive(Copy, Clone)]
pub struct DiscoveredDevice {
    pub id: u8,
    pub dev_type: DeviceType,
    pub base_addr: usize,
}

pub fn register_device(dev_type: DeviceType, base_addr: usize) -> Option<DiscoveredDevice> {
    let id = DEVICE_COUNT.fetch_add(1, Ordering::AcqRel);
    if id as usize >= MAX_DISCOVERED {
        DEVICE_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    DEV_TYPE[id as usize].store(dev_type as u8, Ordering::Release);
    DEV_BASE[id as usize].store(base_addr, Ordering::Release);
    Some(DiscoveredDevice {
        id,
        dev_type,
        base_addr,
    })
}

pub fn device_count() -> u8 {
    DEVICE_COUNT.load(Ordering::Acquire)
}

fn type_from_u8(v: u8) -> DeviceType {
    match v {
        1 => DeviceType::Cpu,
        2 => DeviceType::Gpu,
        3 => DeviceType::Tpu,
        4 => DeviceType::Lpu,
        5 => DeviceType::Network,
        6 => DeviceType::Storage,
        7 => DeviceType::Iommu,
        _ => DeviceType::Unknown,
    }
}

pub fn device_info(id: u8) -> Option<DiscoveredDevice> {
    if id >= DEVICE_COUNT.load(Ordering::Acquire) {
        return None;
    }
    let dev_type = type_from_u8(DEV_TYPE[id as usize].load(Ordering::Acquire));
    let base_addr = DEV_BASE[id as usize].load(Ordering::Acquire);
    Some(DiscoveredDevice {
        id,
        dev_type,
        base_addr,
    })
}

pub fn discover_all() {
    register_device(DeviceType::Cpu, 0);
    let pci_count = crate::bus::discovery::device_count();
    let mut i: usize = 0;
    while i < pci_count {
        let base = crate::bus::discovery::device_id(i).unwrap_or(0) as usize;
        register_device(DeviceType::Unknown, base);
        i += 1;
    }
}
