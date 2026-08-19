#[derive(Copy, Clone, PartialEq)]
pub enum StorageProtocol {
    Ide,
    Ahci,
    Nvme,
    Raid,
    Scsi,
    Mmc,
    Ufs,
    Sdhci,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct StorageController {
    pub vendor_id: u16,
    pub device_id: u16,
    pub protocol: StorageProtocol,
    pub ports: u8,
    pub bar: u32,
    pub bus: u8,
    pub dev: u8,
    pub func: u8,
    pub pci: bool,
    pub reg_base: u64,
    pub reg_size: u64,
}

pub fn detect(out: &mut [StorageController]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut found = 0usize;
    found += scan_pci(&mut out[found..]);
    found += scan_dt(&mut out[found..]);
    found
}

fn scan_pci(out: &mut [StorageController]) -> usize {
    let mut devs = [crate::bus::pci::device::PciDevice {
        bus: 0,
        device: 0,
        function: 0,
        vendor_id: 0,
        device_id: 0,
        class: 0,
        subclass: 0,
    }; 64];
    let total = crate::bus::pci::device::scan_all(&mut devs);
    let mut found = 0usize;
    let mut i = 0usize;
    while i < total && found < out.len() {
        if devs[i].class == 0x01 {
            let bar = bar_for_protocol(devs[i].subclass, &devs[i]);
            let (proto, ports) = classify_storage_pci(devs[i].subclass, bar, &devs[i]);
            out[found] = StorageController {
                vendor_id: devs[i].vendor_id,
                device_id: devs[i].device_id,
                protocol: proto,
                ports,
                bar,
                bus: devs[i].bus,
                dev: devs[i].device,
                func: devs[i].function,
                pci: true,
                reg_base: 0,
                reg_size: 0,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn bar_for_protocol(subclass: u8, dev: &crate::bus::pci::device::PciDevice) -> u32 {
    let offset = match subclass {
        0x06 => 0x24,
        0x08 => 0x10,
        _ => 0x10,
    };
    crate::bus::pci::read_config_u32(dev.bus, dev.device, dev.function, offset).unwrap_or(0)
}

fn classify_storage_pci(
    subclass: u8,
    bar: u32,
    dev: &crate::bus::pci::device::PciDevice,
) -> (StorageProtocol, u8) {
    match subclass {
        0x01 => (StorageProtocol::Ide, 2),
        0x04 => (StorageProtocol::Raid, 0),
        0x05 | 0x06 => {
            let pi_reg = crate::bus::pci::read_config_u32(dev.bus, dev.device, dev.function, 0x24)
                .unwrap_or(0);
            let port_count = count_bits(pi_reg);
            (StorageProtocol::Ahci, port_count)
        }
        0x07 => (StorageProtocol::Scsi, 0),
        0x08 => {
            let _ = bar;
            (StorageProtocol::Nvme, 1)
        }
        _ => (StorageProtocol::Unknown, 0),
    }
}

fn count_bits(val: u32) -> u8 {
    let mut n = val;
    let mut c = 0u8;
    while n != 0 {
        c += (n & 1) as u8;
        n >>= 1;
    }
    c
}

fn scan_dt(out: &mut [StorageController]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut blob = [0u8; 4096];
    let blen = crate::firmware::devicetree::load_fdt_blob(&mut blob);
    if blen < 40 {
        return 0;
    }
    let mut entries = [crate::firmware::devicetree::DtDeviceEntry {
        name: [0u8; 64],
        name_len: 0,
        reg_base: 0,
        reg_size: 0,
        irq: 0,
        compatible: [0u8; 128],
        compatible_len: 0,
    }; 64];
    let count = crate::firmware::devicetree::enumerate_devices(&blob[..blen], &mut entries);
    let mut found = 0usize;
    let mut i = 0usize;
    while i < count && found < out.len() {
        let compat = &entries[i].compatible[..entries[i].compatible_len];
        let proto = classify_storage_compat(compat);
        if !matches!(proto, StorageProtocol::Unknown) {
            out[found] = StorageController {
                vendor_id: 0,
                device_id: 0,
                protocol: proto,
                ports: 1,
                bar: 0,
                bus: 0,
                dev: 0,
                func: 0,
                pci: false,
                reg_base: entries[i].reg_base,
                reg_size: entries[i].reg_size,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn classify_storage_compat(compat: &[u8]) -> StorageProtocol {
    if contains(compat, b"nvme") {
        return StorageProtocol::Nvme;
    }
    if contains(compat, b"ahci") || contains(compat, b"sata") {
        return StorageProtocol::Ahci;
    }
    if contains(compat, b"ufs") || contains(compat, b"ufshc") {
        return StorageProtocol::Ufs;
    }
    if contains(compat, b"sdhci") {
        return StorageProtocol::Sdhci;
    }
    if contains(compat, b"mmc") || contains(compat, b"emmc") {
        return StorageProtocol::Mmc;
    }
    StorageProtocol::Unknown
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }
    let mut i = 0usize;
    while i + needle.len() <= haystack.len() {
        let mut ok = true;
        let mut j = 0usize;
        while j < needle.len() {
            if haystack[i + j] != needle[j] {
                ok = false;
                break;
            }
            j += 1;
        }
        if ok {
            return true;
        }
        i += 1;
    }
    false
}
