#[derive(Copy, Clone, PartialEq)]
pub enum NfcChip {
    Nxp,
    St,
    Broadcom,
    Ti,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct NfcController {
    pub chip: NfcChip,
    pub reg_base: u64,
    pub irq: u32,
    pub compat: [u8; 64],
    pub compat_len: usize,
}

pub fn detect(out: &mut [NfcController]) -> usize {
    if out.is_empty() {
        return 0;
    }
    scan_dt(out)
}

fn scan_dt(out: &mut [NfcController]) -> usize {
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
        let chip = classify_nfc(compat, name);
        if !matches!(chip, NfcChip::Unknown) {
            let mut compat_buf = [0u8; 64];
            let clen = copy_min(compat, &mut compat_buf);
            out[found] = NfcController {
                chip,
                reg_base: entries[i].reg_base,
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

fn classify_nfc(compat: &[u8], name: &[u8]) -> NfcChip {
    if contains(compat, b"nxp,pn5")
        || contains(compat, b"nxp,pn7")
        || contains(compat, b"nxp,nq")
        || contains(compat, b"nxp,sn")
    {
        return NfcChip::Nxp;
    }
    if contains(compat, b"st,st21") || contains(compat, b"st,st54") || contains(compat, b"st,st25")
    {
        return NfcChip::St;
    }
    if contains(compat, b"brcm,bcm2079") || contains(compat, b"brcm,nfc") {
        return NfcChip::Broadcom;
    }
    if contains(compat, b"ti,trf7") {
        return NfcChip::Ti;
    }
    if contains(compat, b"nfc") || contains(name, b"nfc") {
        return NfcChip::Nxp;
    }
    NfcChip::Unknown
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
