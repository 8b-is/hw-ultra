#[derive(Copy, Clone, PartialEq)]
pub enum ModemGeneration {
    Gen2G,
    Gen3G,
    Lte,
    Gen5G,
    Unknown,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ModemInterface {
    Pci,
    Usb,
    Platform,
}

#[derive(Copy, Clone)]
pub struct ModemDevice {
    pub generation: ModemGeneration,
    pub interface: ModemInterface,
    pub vendor_id: u16,
    pub device_id: u16,
    pub reg_base: u64,
    pub compat: [u8; 64],
    pub compat_len: usize,
}

pub fn detect(out: &mut [ModemDevice]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut found = 0usize;
    found += scan_pci(&mut out[found..]);
    found += scan_dt(&mut out[found..]);
    found
}

fn scan_pci(out: &mut [ModemDevice]) -> usize {
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
        let is_modem = (devs[i].class == 0x07)
            || (devs[i].class == 0x02 && devs[i].subclass == 0x80)
            || (devs[i].class == 0x0D);
        if is_modem {
            let gen = infer_pci_modem_gen(devs[i].vendor_id, devs[i].class, devs[i].subclass);
            out[found] = ModemDevice {
                generation: gen,
                interface: ModemInterface::Pci,
                vendor_id: devs[i].vendor_id,
                device_id: devs[i].device_id,
                reg_base: 0,
                compat: [0u8; 64],
                compat_len: 0,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn infer_pci_modem_gen(vendor: u16, class: u8, subclass: u8) -> ModemGeneration {
    if class == 0x0D {
        return match subclass {
            0x00 => ModemGeneration::Unknown,
            0x01 => ModemGeneration::Unknown,
            _ => ModemGeneration::Lte,
        };
    }
    match vendor {
        0x8086 => ModemGeneration::Lte,
        0x1AE9 => ModemGeneration::Gen5G,
        0x17CB => ModemGeneration::Lte,
        _ => ModemGeneration::Unknown,
    }
}

fn scan_dt(out: &mut [ModemDevice]) -> usize {
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
        let name = &entries[i].name[..entries[i].name_len];
        let gen = classify_modem(compat, name);
        if !matches!(gen, ModemGeneration::Unknown) {
            let mut compat_buf = [0u8; 64];
            let clen = copy_min(compat, &mut compat_buf);
            out[found] = ModemDevice {
                generation: gen,
                interface: ModemInterface::Platform,
                vendor_id: 0,
                device_id: 0,
                reg_base: entries[i].reg_base,
                compat: compat_buf,
                compat_len: clen,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn classify_modem(compat: &[u8], name: &[u8]) -> ModemGeneration {
    if contains(compat, b"5g") || contains(name, b"5g") {
        return ModemGeneration::Gen5G;
    }
    if contains(compat, b"lte") || contains(name, b"lte") {
        return ModemGeneration::Lte;
    }
    if contains(compat, b"modem")
        || contains(name, b"modem")
        || contains(compat, b"wwan")
        || contains(name, b"wwan")
    {
        return ModemGeneration::Lte;
    }
    if contains(compat, b"mediatek,mt68")
        || contains(compat, b"mediatek,mt67")
        || contains(compat, b"qcom,sdx")
        || contains(compat, b"sierra,")
        || contains(compat, b"quectel,")
    {
        return ModemGeneration::Lte;
    }
    ModemGeneration::Unknown
}

fn copy_min(src: &[u8], dst: &mut [u8; 64]) -> usize {
    let n = if src.len() < 64 { src.len() } else { 64 };
    let mut i = 0usize;
    while i < n {
        dst[i] = src[i];
        i += 1;
    }
    n
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
