#[derive(Copy, Clone, PartialEq)]
pub enum InputKind {
    Keyboard,
    Mouse,
    Touchscreen,
    Touchpad,
    GameController,
    Keys,
    Unknown,
}

#[derive(Copy, Clone, PartialEq)]
pub enum InputInterface {
    Ps2,
    Usb,
    I2c,
    Gpio,
    Spi,
    Platform,
}

#[derive(Copy, Clone)]
pub struct InputDevice {
    pub kind: InputKind,
    pub interface: InputInterface,
    pub reg_base: u64,
    pub irq: u32,
}

pub fn detect(out: &mut [InputDevice]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut found = 0usize;
    found += probe_ps2(&mut out[found..]);
    found += scan_pci_input(&mut out[found..]);
    found += scan_dt(&mut out[found..]);
    found
}

fn probe_ps2(out: &mut [InputDevice]) -> usize {
    if out.is_empty() {
        return 0;
    }
    if !matches!(
        crate::arch::detect_arch(),
        crate::arch::Architecture::X86_64
    ) {
        return 0;
    }
    let status = unsafe { crate::arch::x86_64::io::inb(0x64) };
    if status == 0xFF {
        return 0;
    }
    let mut found = 0usize;
    out[found] = InputDevice {
        kind: InputKind::Keyboard,
        interface: InputInterface::Ps2,
        reg_base: 0x60,
        irq: 1,
    };
    found += 1;
    if found < out.len() && (status & 0x20) == 0 {
        out[found] = InputDevice {
            kind: InputKind::Mouse,
            interface: InputInterface::Ps2,
            reg_base: 0x60,
            irq: 12,
        };
        found += 1;
    }
    found
}

fn scan_pci_input(out: &mut [InputDevice]) -> usize {
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
        if devs[i].class == 0x09 {
            let kind = match devs[i].subclass {
                0x00 => InputKind::Keyboard,
                0x02 => InputKind::Mouse,
                0x04 => InputKind::GameController,
                _ => InputKind::Unknown,
            };
            out[found] = InputDevice {
                kind,
                interface: InputInterface::Usb,
                reg_base: devs[i].bus as u64,
                irq: 0,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn scan_dt(out: &mut [InputDevice]) -> usize {
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
        let (kind, iface) = classify_input(compat, name);
        if !matches!(kind, InputKind::Unknown) {
            out[found] = InputDevice {
                kind,
                interface: iface,
                reg_base: entries[i].reg_base,
                irq: entries[i].irq,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn classify_input(compat: &[u8], name: &[u8]) -> (InputKind, InputInterface) {
    if contains(compat, b"touchscreen") || contains(name, b"touchscreen") {
        if contains(compat, b"i2c") {
            return (InputKind::Touchscreen, InputInterface::I2c);
        }
        if contains(compat, b"spi") {
            return (InputKind::Touchscreen, InputInterface::Spi);
        }
        return (InputKind::Touchscreen, InputInterface::I2c);
    }
    if contains(compat, b"touchpad") || contains(name, b"touchpad") {
        return (InputKind::Touchpad, InputInterface::I2c);
    }
    if contains(compat, b"gpio-keys") || contains(compat, b"gpio_keys") {
        return (InputKind::Keys, InputInterface::Gpio);
    }
    if contains(compat, b"power-key") || contains(compat, b"pwrkey") {
        return (InputKind::Keys, InputInterface::Platform);
    }
    if contains(compat, b"volume") || contains(compat, b"buttons") {
        return (InputKind::Keys, InputInterface::Gpio);
    }
    if contains(name, b"keyboard") {
        return (InputKind::Keyboard, InputInterface::Platform);
    }
    (InputKind::Unknown, InputInterface::Platform)
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
