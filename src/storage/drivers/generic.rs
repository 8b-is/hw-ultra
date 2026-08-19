use crate::storage::detection::StorageProtocol;
use crate::storage::hw;
use core::sync::atomic::{AtomicU8, Ordering};

static DRIVER_STATE: AtomicU8 = AtomicU8::new(0);

const STATE_PROBED: u8 = 1;
const STATE_ACTIVE: u8 = 2;

pub struct AhciDriver {
    pub base: usize,
    pub port_count: u8,
    pub ports_implemented: u32,
}

pub struct NvmeDriver {
    pub base: usize,
    pub version_major: u16,
    pub version_minor: u16,
}

pub enum StorageDriver {
    Ahci(AhciDriver),
    Nvme(NvmeDriver),
}

pub fn probe(base: usize, protocol: StorageProtocol) -> Option<StorageDriver> {
    match protocol {
        StorageProtocol::Ahci => {
            let ports = hw::ahci_port_count(base);
            let implemented = hw::ahci_ports_implemented(base);
            DRIVER_STATE.store(STATE_PROBED, Ordering::Release);
            Some(StorageDriver::Ahci(AhciDriver {
                base,
                port_count: ports,
                ports_implemented: implemented,
            }))
        }
        StorageProtocol::Nvme => {
            let (major, minor, _) = hw::nvme_version(base);
            DRIVER_STATE.store(STATE_PROBED, Ordering::Release);
            Some(StorageDriver::Nvme(NvmeDriver {
                base,
                version_major: major,
                version_minor: minor as u16,
            }))
        }
        _ => None,
    }
}

pub fn init(driver: &StorageDriver) -> bool {
    match driver {
        StorageDriver::Ahci(ahci) => {
            hw::ahci_enable(ahci.base);
            DRIVER_STATE.store(STATE_ACTIVE, Ordering::Release);
            true
        }
        StorageDriver::Nvme(nvme) => {
            hw::nvme_enable(nvme.base);
            let ready = hw::nvme_ready(nvme.base);
            if ready {
                DRIVER_STATE.store(STATE_ACTIVE, Ordering::Release);
            }
            ready
        }
    }
}

pub fn state() -> u8 {
    DRIVER_STATE.load(Ordering::Acquire)
}
