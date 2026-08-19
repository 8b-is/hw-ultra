use core::sync::atomic::{AtomicUsize, Ordering};

static DT_PRESENT: AtomicUsize = AtomicUsize::new(0);
static DT_SMMU_BASE: AtomicUsize = AtomicUsize::new(0);
static DT_FDT_ADDR: AtomicUsize = AtomicUsize::new(0);
static DT_FDT_SIZE: AtomicUsize = AtomicUsize::new(0);
static DT_ADDR_CELLS: AtomicUsize = AtomicUsize::new(0);
static DT_SIZE_CELLS: AtomicUsize = AtomicUsize::new(0);

const FDT_MAGIC: u32 = 0xD00DFEED;
const FDT_BEGIN_NODE: u32 = 1;
const FDT_END_NODE: u32 = 2;
const FDT_PROP: u32 = 3;
const FDT_NOP: u32 = 4;
const FDT_END: u32 = 9;

#[derive(Copy, Clone)]
pub struct FdtHeader {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[derive(Copy, Clone)]
pub struct FdtNode {
    pub name: [u8; 64],
    pub name_len: usize,
    pub depth: usize,
    pub offset: usize,
}

#[derive(Copy, Clone)]
pub struct DtDeviceEntry {
    pub name: [u8; 64],
    pub name_len: usize,
    pub reg_base: u64,
    pub reg_size: u64,
    pub irq: u32,
    pub compatible: [u8; 128],
    pub compatible_len: usize,
}

fn fdt_u32(blob: &[u8], offset: usize) -> u32 {
    if offset + 3 < blob.len() {
        u32::from_be_bytes([
            blob[offset],
            blob[offset + 1],
            blob[offset + 2],
            blob[offset + 3],
        ])
    } else {
        0
    }
}

fn fdt_u64(blob: &[u8], offset: usize) -> u64 {
    (fdt_u32(blob, offset) as u64) << 32 | fdt_u32(blob, offset + 4) as u64
}

fn fdt_str_at(blob: &[u8], strings_off: usize, nameoff: usize) -> ([u8; 64], usize) {
    let mut name = [0u8; 64];
    let base = strings_off + nameoff;
    let mut i = 0;
    while i < 63 && base + i < blob.len() && blob[base + i] != 0 {
        name[i] = blob[base + i];
        i += 1;
    }
    (name, i)
}

fn align4(v: usize) -> usize {
    (v + 3) & !3
}

pub fn parse_fdt_header(blob: &[u8]) -> Option<FdtHeader> {
    if blob.len() < 40 {
        return None;
    }
    let magic = fdt_u32(blob, 0);
    if magic != FDT_MAGIC {
        return None;
    }
    Some(FdtHeader {
        magic,
        totalsize: fdt_u32(blob, 4),
        off_dt_struct: fdt_u32(blob, 8),
        off_dt_strings: fdt_u32(blob, 12),
        off_mem_rsvmap: fdt_u32(blob, 16),
        version: fdt_u32(blob, 20),
        last_comp_version: fdt_u32(blob, 24),
        boot_cpuid_phys: fdt_u32(blob, 28),
        size_dt_strings: fdt_u32(blob, 32),
        size_dt_struct: fdt_u32(blob, 36),
    })
}

const MAX_FDT_NODES: usize = 128;

pub fn enumerate_nodes(blob: &[u8], out: &mut [FdtNode; MAX_FDT_NODES]) -> usize {
    let hdr = match parse_fdt_header(blob) {
        Some(h) => h,
        None => return 0,
    };
    let struct_off = hdr.off_dt_struct as usize;
    let struct_end = struct_off + hdr.size_dt_struct as usize;
    let mut pos = struct_off;
    let mut depth: usize = 0;
    let mut count: usize = 0;
    while pos + 4 <= struct_end && pos + 4 <= blob.len() && count < MAX_FDT_NODES {
        let token = fdt_u32(blob, pos);
        pos += 4;
        match token {
            FDT_BEGIN_NODE => {
                let mut name = [0u8; 64];
                let mut i = 0;
                while i < 63 && pos + i < blob.len() && blob[pos + i] != 0 {
                    name[i] = blob[pos + i];
                    i += 1;
                }
                out[count] = FdtNode {
                    name,
                    name_len: i,
                    depth,
                    offset: pos - 4,
                };
                count += 1;
                pos = align4(pos + i + 1);
                depth += 1;
            }
            FDT_END_NODE => {
                depth = depth.saturating_sub(1);
            }
            FDT_PROP => {
                if pos + 8 > blob.len() {
                    break;
                }
                let val_len = fdt_u32(blob, pos) as usize;
                pos += 8;
                pos = align4(pos + val_len);
            }
            FDT_NOP => {}
            FDT_END => break,
            _ => break,
        }
    }
    count
}

const MAX_FDT_DEVICES: usize = 64;

pub fn enumerate_devices(blob: &[u8], out: &mut [DtDeviceEntry; MAX_FDT_DEVICES]) -> usize {
    let hdr = match parse_fdt_header(blob) {
        Some(h) => h,
        None => return 0,
    };
    let struct_off = hdr.off_dt_struct as usize;
    let struct_end = struct_off + hdr.size_dt_struct as usize;
    let strings_off = hdr.off_dt_strings as usize;
    let mut pos = struct_off;
    let mut depth: usize = 0;
    let mut count: usize = 0;
    let mut cur_name = [0u8; 64];
    let mut cur_name_len: usize = 0;
    let mut cur_reg_base: u64 = 0;
    let mut cur_reg_size: u64 = 0;
    let mut cur_irq: u32 = 0;
    let mut cur_compat = [0u8; 128];
    let mut cur_compat_len: usize = 0;
    let mut has_compat = false;

    while pos + 4 <= struct_end && pos + 4 <= blob.len() {
        let token = fdt_u32(blob, pos);
        pos += 4;
        match token {
            FDT_BEGIN_NODE => {
                let mut name = [0u8; 64];
                let mut i = 0;
                while i < 63 && pos + i < blob.len() && blob[pos + i] != 0 {
                    name[i] = blob[pos + i];
                    i += 1;
                }
                pos = align4(pos + i + 1);
                depth += 1;
                cur_name = name;
                cur_name_len = i;
                cur_reg_base = 0;
                cur_reg_size = 0;
                cur_irq = 0;
                cur_compat = [0u8; 128];
                cur_compat_len = 0;
                has_compat = false;
            }
            FDT_END_NODE => {
                if has_compat && count < MAX_FDT_DEVICES {
                    out[count] = DtDeviceEntry {
                        name: cur_name,
                        name_len: cur_name_len,
                        reg_base: cur_reg_base,
                        reg_size: cur_reg_size,
                        irq: cur_irq,
                        compatible: cur_compat,
                        compatible_len: cur_compat_len,
                    };
                    count += 1;
                }
                depth = depth.saturating_sub(1);
            }
            FDT_PROP => {
                if pos + 8 > blob.len() {
                    break;
                }
                let val_len = fdt_u32(blob, pos) as usize;
                let nameoff = fdt_u32(blob, pos + 4) as usize;
                pos += 8;
                let val_start = pos;
                pos = align4(pos + val_len);
                let (pname, pname_len) = fdt_str_at(blob, strings_off, nameoff);
                if pname_len == 10 && pname[..10] == *b"compatible" && val_len > 0 {
                    has_compat = true;
                    let copy_len = if val_len > 128 { 128 } else { val_len };
                    let mut j = 0;
                    while j < copy_len && val_start + j < blob.len() {
                        cur_compat[j] = blob[val_start + j];
                        j += 1;
                    }
                    cur_compat_len = copy_len;
                } else if pname_len == 3 && pname[..3] == *b"reg" && val_len >= 8 {
                    cur_reg_base = fdt_u32(blob, val_start) as u64;
                    if val_len >= 16 {
                        cur_reg_base = fdt_u64(blob, val_start);
                        cur_reg_size = fdt_u64(blob, val_start + 8);
                    } else {
                        cur_reg_size = fdt_u32(blob, val_start + 4) as u64;
                    }
                } else if pname_len == 10 && pname[..10] == *b"interrupts" && val_len >= 4 {
                    cur_irq = fdt_u32(blob, val_start);
                }
            }
            FDT_NOP => {}
            FDT_END => break,
            _ => break,
        }
    }
    count
}

pub fn parse_devicetree() {
    let addr = DT_FDT_ADDR.load(Ordering::Acquire);
    let size = DT_FDT_SIZE.load(Ordering::Acquire);
    if addr == 0 || size == 0 {
        return;
    }
    let mut buf = [0u8; 4096];
    let copy_len = load_fdt_blob(&mut buf);
    if copy_len < 40 {
        return;
    }
    let blob = &buf[..copy_len];
    if parse_fdt_header(blob).is_none() {
        return;
    }
    // Extract #address-cells and #size-cells from root properties
    let hdr = match parse_fdt_header(blob) {
        Some(h) => h,
        None => return,
    };
    let struct_off = hdr.off_dt_struct as usize;
    let strings_off = hdr.off_dt_strings as usize;
    let struct_end = struct_off + hdr.size_dt_struct as usize;
    let mut pos = struct_off;
    while pos + 4 <= struct_end && pos + 4 <= blob.len() {
        let token = fdt_u32(blob, pos);
        pos += 4;
        match token {
            FDT_BEGIN_NODE => {
                let mut i = 0;
                while i < 63 && pos + i < blob.len() && blob[pos + i] != 0 {
                    i += 1;
                }
                pos = align4(pos + i + 1);
            }
            FDT_PROP => {
                if pos + 8 > blob.len() {
                    break;
                }
                let val_len = fdt_u32(blob, pos) as usize;
                let nameoff = fdt_u32(blob, pos + 4) as usize;
                pos += 8;
                let val_start = pos;
                pos = align4(pos + val_len);
                let (pname, pname_len) = fdt_str_at(blob, strings_off, nameoff);
                if pname_len == 14 && pname[..14] == *b"#address-cells" && val_len >= 4 {
                    DT_ADDR_CELLS.store(fdt_u32(blob, val_start) as usize, Ordering::Release);
                } else if pname_len == 11 && pname[..11] == *b"#size-cells" && val_len >= 4 {
                    DT_SIZE_CELLS.store(fdt_u32(blob, val_start) as usize, Ordering::Release);
                }
            }
            FDT_END_NODE | FDT_NOP => {}
            _ => break,
        }
    }
    // Scan for SMMU node
    let mut devices = [DtDeviceEntry {
        name: [0u8; 64],
        name_len: 0,
        reg_base: 0,
        reg_size: 0,
        irq: 0,
        compatible: [0u8; 128],
        compatible_len: 0,
    }; 64];
    let dev_count = enumerate_devices(blob, &mut devices);
    let mut i = 0;
    while i < dev_count {
        let compat = &devices[i].compatible[..devices[i].compatible_len];
        // Check for SMMU compatible strings
        let mut j = 0;
        while j + 9 <= compat.len() {
            if compat[j..j + 9] == *b"arm,smmu" {
                DT_SMMU_BASE.store(devices[i].reg_base as usize, Ordering::Release);
                break;
            }
            j += 1;
        }
        i += 1;
    }
    DT_PRESENT.store(1, Ordering::Release);
}

pub fn set_fdt(addr: usize, size: usize) {
    DT_FDT_ADDR.store(addr, Ordering::Release);
    DT_FDT_SIZE.store(size, Ordering::Release);
    DT_PRESENT.store(1, Ordering::Release);
}

pub fn load_fdt_blob(buf: &mut [u8]) -> usize {
    let addr = DT_FDT_ADDR.load(Ordering::Acquire);
    let size = DT_FDT_SIZE.load(Ordering::Acquire);
    if addr == 0 || size == 0 {
        return 0;
    }
    let copy_len = size.min(buf.len());
    let mut i = 0;
    while i < copy_len {
        buf[i] = crate::hardware_access::mmio_read32(addr + i)
            .map(|v| v as u8)
            .unwrap_or(0);
        i += 1;
    }
    copy_len
}

pub fn fdt_address() -> Option<usize> {
    let addr = DT_FDT_ADDR.load(Ordering::Acquire);
    if addr != 0 {
        Some(addr)
    } else {
        None
    }
}

pub fn address_cells() -> u32 {
    DT_ADDR_CELLS.load(Ordering::Acquire) as u32
}

pub fn size_cells() -> u32 {
    DT_SIZE_CELLS.load(Ordering::Acquire) as u32
}

pub fn is_present() -> bool {
    if DT_PRESENT.load(Ordering::Acquire) != 0 {
        return true;
    }
    parse_devicetree();
    DT_PRESENT.load(Ordering::Acquire) != 0
}

pub fn find_smmu_base() -> Option<usize> {
    if DT_PRESENT.load(Ordering::Acquire) == 0 {
        parse_devicetree();
    }
    let base = DT_SMMU_BASE.load(Ordering::Acquire);
    if base != 0 {
        Some(base)
    } else {
        None
    }
}

/// Search the FDT for a device whose compatible string contains `needle`.
/// Returns `(reg_base, reg_size, irq)` or `None`.
pub fn find_device_by_compatible(needle: &[u8]) -> Option<(usize, usize, u32)> {
    if DT_PRESENT.load(Ordering::Acquire) == 0 {
        parse_devicetree();
    }
    let mut fdt_buf = [0u8; 65536];
    let fdt_len = load_fdt_blob(&mut fdt_buf);
    if fdt_len == 0 {
        return None;
    }
    let blob = &fdt_buf[..fdt_len];
    let mut devices = [DtDeviceEntry {
        name: [0u8; 64],
        name_len: 0,
        reg_base: 0,
        reg_size: 0,
        irq: 0,
        compatible: [0u8; 128],
        compatible_len: 0,
    }; 64];
    let count = enumerate_devices(blob, &mut devices);
    let mut i = 0;
    while i < count {
        let compat = &devices[i].compatible[..devices[i].compatible_len];
        if contains_bytes(compat, needle) && devices[i].reg_base != 0 {
            return Some((
                devices[i].reg_base as usize,
                if devices[i].reg_size != 0 {
                    devices[i].reg_size as usize
                } else {
                    0
                },
                devices[i].irq,
            ));
        }
        i += 1;
    }
    None
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > haystack.len() {
        return false;
    }
    let mut i = 0;
    while i + needle.len() <= haystack.len() {
        let mut matched = true;
        let mut j = 0;
        while j < needle.len() {
            if haystack[i + j] != needle[j] {
                matched = false;
                break;
            }
            j += 1;
        }
        if matched {
            return true;
        }
        i += 1;
    }
    false
}
