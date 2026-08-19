#[derive(Copy, Clone, PartialEq)]
pub enum DisplayType {
    Vga,
    Xga,
    ThreeD,
    Dsi,
    Hdmi,
    Dp,
    Lvds,
    Edp,
    Lcdc,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct DisplayController {
    pub vendor_id: u16,
    pub device_id: u16,
    pub display_type: DisplayType,
    pub framebuffer_bar: u32,
    pub bus: u8,
    pub dev: u8,
    pub func: u8,
    pub pci: bool,
    pub reg_base: u64,
    pub reg_size: u64,
}

pub fn detect(out: &mut [DisplayController]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut found = 0usize;
    found += scan_pci(&mut out[found..]);
    found += scan_dt(&mut out[found..]);
    found
}

fn scan_pci(out: &mut [DisplayController]) -> usize {
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
        if devs[i].class == 0x03 {
            let bar0 = crate::bus::pci::read_config_u32(
                devs[i].bus,
                devs[i].device,
                devs[i].function,
                0x10,
            )
            .unwrap_or(0);
            let dt = match devs[i].subclass {
                0x00 => DisplayType::Vga,
                0x01 => DisplayType::Xga,
                0x02 => DisplayType::ThreeD,
                _ => DisplayType::Unknown,
            };
            out[found] = DisplayController {
                vendor_id: devs[i].vendor_id,
                device_id: devs[i].device_id,
                display_type: dt,
                framebuffer_bar: bar0,
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

fn scan_dt(out: &mut [DisplayController]) -> usize {
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
        let dt = classify_display_compat(compat);
        if !matches!(dt, DisplayType::Unknown) {
            out[found] = DisplayController {
                vendor_id: 0,
                device_id: 0,
                display_type: dt,
                framebuffer_bar: 0,
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

fn classify_display_compat(compat: &[u8]) -> DisplayType {
    if contains(compat, b"dsi") {
        return DisplayType::Dsi;
    }
    if contains(compat, b"hdmi") {
        return DisplayType::Hdmi;
    }
    if contains(compat, b"dp") || contains(compat, b"displayport") {
        return DisplayType::Dp;
    }
    if contains(compat, b"edp") {
        return DisplayType::Edp;
    }
    if contains(compat, b"lvds") {
        return DisplayType::Lvds;
    }
    if contains(compat, b"panel") || contains(compat, b"lcd") || contains(compat, b"lcdc") {
        return DisplayType::Lcdc;
    }
    if contains(compat, b"display") || contains(compat, b"framebuffer") {
        return DisplayType::Vga;
    }
    DisplayType::Unknown
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
