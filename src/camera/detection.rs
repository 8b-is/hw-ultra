#[derive(Copy, Clone, PartialEq)]
pub enum CameraInterface {
    Csi,
    Mipi,
    Usb,
    Platform,
}

#[derive(Copy, Clone)]
pub struct CameraDevice {
    pub interface: CameraInterface,
    pub reg_base: u64,
    pub reg_size: u64,
    pub irq: u32,
    pub compat: [u8; 64],
    pub compat_len: usize,
}

pub fn detect(out: &mut [CameraDevice]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut found = 0usize;
    found += scan_pci_imaging(&mut out[found..]);
    found += scan_dt(&mut out[found..]);
    found
}

fn scan_pci_imaging(out: &mut [CameraDevice]) -> usize {
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
        if devs[i].class == 0x0E {
            out[found] = CameraDevice {
                interface: CameraInterface::Usb,
                reg_base: devs[i].bus as u64,
                reg_size: 0,
                irq: 0,
                compat: [0u8; 64],
                compat_len: 0,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn scan_dt(out: &mut [CameraDevice]) -> usize {
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
        let iface = classify_camera_compat(compat);
        if !matches!(iface, CameraInterface::Platform)
            || is_camera_node(&entries[i].name[..entries[i].name_len])
        {
            let real_iface = if matches!(iface, CameraInterface::Platform) {
                if is_camera_node(&entries[i].name[..entries[i].name_len]) {
                    CameraInterface::Csi
                } else {
                    i += 1;
                    continue;
                }
            } else {
                iface
            };
            let mut compat_buf = [0u8; 64];
            let clen = copy_min(compat, &mut compat_buf);
            out[found] = CameraDevice {
                interface: real_iface,
                reg_base: entries[i].reg_base,
                reg_size: entries[i].reg_size,
                irq: entries[i].irq,
                compat: compat_buf,
                compat_len: clen,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn classify_camera_compat(compat: &[u8]) -> CameraInterface {
    if contains(compat, b"csi") || contains(compat, b"csiphy") {
        return CameraInterface::Csi;
    }
    if contains(compat, b"mipi") {
        return CameraInterface::Mipi;
    }
    if contains(compat, b"camera")
        || contains(compat, b"imx")
        || contains(compat, b"ov5640")
        || contains(compat, b"ov8856")
        || contains(compat, b"ov13b")
        || contains(compat, b"s5k")
        || contains(compat, b"gc2375")
        || contains(compat, b"gc5035")
        || contains(compat, b"hi846")
        || contains(compat, b"ar0234")
    {
        return CameraInterface::Csi;
    }
    CameraInterface::Platform
}

fn is_camera_node(name: &[u8]) -> bool {
    contains(name, b"camera") || contains(name, b"cam") || contains(name, b"sensor")
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
