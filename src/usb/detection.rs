#[derive(Copy, Clone, PartialEq)]
pub enum HostController {
    Uhci,
    Ohci,
    Ehci,
    Xhci,
    Dwc2,
    Dwc3,
    Musb,
    Unknown,
}

#[derive(Copy, Clone, PartialEq)]
pub enum UsbSpeed {
    LowFull,
    High,
    Super,
    SuperPlus,
}

#[derive(Copy, Clone)]
pub struct UsbController {
    pub vendor_id: u16,
    pub device_id: u16,
    pub hc: HostController,
    pub speed: UsbSpeed,
    pub bar: u32,
    pub bus: u8,
    pub dev: u8,
    pub func: u8,
    pub pci: bool,
    pub reg_base: u64,
    pub reg_size: u64,
}

pub fn detect(out: &mut [UsbController]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut found = 0usize;
    found += scan_pci(&mut out[found..]);
    found += scan_dt(&mut out[found..]);
    found
}

fn scan_pci(out: &mut [UsbController]) -> usize {
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
        if devs[i].class == 0x0C && devs[i].subclass == 0x03 {
            let reg08 = crate::bus::pci::read_config_u32(
                devs[i].bus,
                devs[i].device,
                devs[i].function,
                0x08,
            )
            .unwrap_or(0);
            let prog_if = ((reg08 >> 8) & 0xFF) as u8;

            let bar0 = crate::bus::pci::read_config_u32(
                devs[i].bus,
                devs[i].device,
                devs[i].function,
                0x10,
            )
            .unwrap_or(0);

            let (hc, speed) = classify_usb_prog_if(prog_if);
            out[found] = UsbController {
                vendor_id: devs[i].vendor_id,
                device_id: devs[i].device_id,
                hc,
                speed,
                bar: bar0,
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

fn classify_usb_prog_if(prog_if: u8) -> (HostController, UsbSpeed) {
    match prog_if {
        0x00 => (HostController::Uhci, UsbSpeed::LowFull),
        0x10 => (HostController::Ohci, UsbSpeed::LowFull),
        0x20 => (HostController::Ehci, UsbSpeed::High),
        0x30 => (HostController::Xhci, UsbSpeed::Super),
        0xFE => (HostController::Unknown, UsbSpeed::LowFull),
        _ => (HostController::Unknown, UsbSpeed::LowFull),
    }
}

fn scan_dt(out: &mut [UsbController]) -> usize {
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
        let (hc, speed) = classify_usb_compat(compat);
        if !matches!(hc, HostController::Unknown) {
            out[found] = UsbController {
                vendor_id: 0,
                device_id: 0,
                hc,
                speed,
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

fn classify_usb_compat(compat: &[u8]) -> (HostController, UsbSpeed) {
    if contains(compat, b"xhci") {
        return (HostController::Xhci, UsbSpeed::Super);
    }
    if contains(compat, b"dwc3") {
        return (HostController::Dwc3, UsbSpeed::Super);
    }
    if contains(compat, b"dwc2") {
        return (HostController::Dwc2, UsbSpeed::High);
    }
    if contains(compat, b"ehci") {
        return (HostController::Ehci, UsbSpeed::High);
    }
    if contains(compat, b"ohci") {
        return (HostController::Ohci, UsbSpeed::LowFull);
    }
    if contains(compat, b"musb") {
        return (HostController::Musb, UsbSpeed::High);
    }
    if contains(compat, b"usb") {
        return (HostController::Unknown, UsbSpeed::High);
    }
    (HostController::Unknown, UsbSpeed::LowFull)
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
