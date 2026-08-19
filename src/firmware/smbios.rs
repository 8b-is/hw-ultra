use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static SMBIOS_BASE: AtomicUsize = AtomicUsize::new(0);
static SMBIOS_VERSION_MAJOR: AtomicU8 = AtomicU8::new(0);
static SMBIOS_VERSION_MINOR: AtomicU8 = AtomicU8::new(0);
static SMBIOS_TABLE_COUNT: AtomicUsize = AtomicUsize::new(0);
static SMBIOS_TABLE_ADDR: AtomicUsize = AtomicUsize::new(0);
static SMBIOS_TABLE_LEN: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct SmbiosHeader {
    pub entry_type: u8,
    pub length: u8,
    pub handle: u16,
}

#[derive(Copy, Clone)]
pub struct SmbiosInfo {
    pub base: usize,
    pub version_major: u8,
    pub version_minor: u8,
    pub table_count: usize,
}

#[derive(Copy, Clone)]
pub struct BiosInfo {
    pub vendor_str_idx: u8,
    pub version_str_idx: u8,
    pub release_date_str_idx: u8,
    pub rom_size_64k: u8,
    pub major_release: u8,
    pub minor_release: u8,
}

#[derive(Copy, Clone)]
pub struct CpuInfo {
    pub socket_str_idx: u8,
    pub processor_type: u8,
    pub processor_family: u8,
    pub processor_id: u64,
    pub max_speed_mhz: u16,
    pub current_speed_mhz: u16,
    pub core_count: u8,
    pub thread_count: u8,
    pub voltage: u8,
}

#[derive(Copy, Clone)]
pub struct MemModuleInfo {
    pub locator_str_idx: u8,
    pub size_mb: u16,
    pub speed_mhz: u16,
    pub memory_type: u8,
    pub form_factor: u8,
    pub data_width: u16,
    pub rank: u8,
}

fn smbios_read_u8(blob: &[u8], offset: usize) -> u8 {
    if offset < blob.len() {
        blob[offset]
    } else {
        0
    }
}

fn smbios_read_u16(blob: &[u8], offset: usize) -> u16 {
    if offset + 1 < blob.len() {
        u16::from_le_bytes([blob[offset], blob[offset + 1]])
    } else {
        0
    }
}

fn smbios_read_u64(blob: &[u8], offset: usize) -> u64 {
    if offset + 7 < blob.len() {
        u64::from_le_bytes([
            blob[offset],
            blob[offset + 1],
            blob[offset + 2],
            blob[offset + 3],
            blob[offset + 4],
            blob[offset + 5],
            blob[offset + 6],
            blob[offset + 7],
        ])
    } else {
        0
    }
}

fn skip_structure(blob: &[u8], offset: usize, header_len: usize) -> usize {
    let mut pos = offset + header_len;
    loop {
        if pos + 1 >= blob.len() {
            return blob.len();
        }
        if blob[pos] == 0 && blob[pos + 1] == 0 {
            return pos + 2;
        }
        pos += 1;
    }
}

pub fn parse_smbios() {
    let scan_regions: [(usize, usize); 2] =
        [(0x000F_0000, 0x000F_FFFF), (0x000E_0000, 0x000E_FFFF)];
    for (start, end) in scan_regions {
        let mut addr = start;
        while addr < end {
            let sig = crate::hardware_access::mmio_read32(addr).unwrap_or(0);
            if sig == 0x5F4D535F {
                SMBIOS_BASE.store(addr, Ordering::Release);
                let version_byte = crate::hardware_access::mmio_read32(addr + 6).unwrap_or(0);
                SMBIOS_VERSION_MAJOR.store((version_byte & 0xFF) as u8, Ordering::Release);
                SMBIOS_VERSION_MINOR.store(((version_byte >> 8) & 0xFF) as u8, Ordering::Release);
                let table_addr_raw = crate::hardware_access::mmio_read32(addr + 24).unwrap_or(0);
                let table_len =
                    crate::hardware_access::mmio_read32(addr + 22).unwrap_or(0) & 0xFFFF;
                SMBIOS_TABLE_ADDR.store(table_addr_raw as usize, Ordering::Release);
                SMBIOS_TABLE_LEN.store(table_len as usize, Ordering::Release);
                if table_addr_raw != 0 {
                    let mut offset: usize = 0;
                    let mut count: usize = 0;
                    while offset < table_len as usize {
                        let hdr_raw =
                            crate::hardware_access::mmio_read32(table_addr_raw as usize + offset)
                                .unwrap_or(0);
                        let length = ((hdr_raw >> 8) & 0xFF) as usize;
                        if length < 4 {
                            break;
                        }
                        offset += length;
                        loop {
                            let pair = crate::hardware_access::mmio_read32(
                                table_addr_raw as usize + offset,
                            )
                            .unwrap_or(0)
                                & 0xFFFF;
                            offset += 1;
                            if pair == 0 {
                                offset += 1;
                                break;
                            }
                        }
                        count += 1;
                    }
                    SMBIOS_TABLE_COUNT.store(count, Ordering::Release);
                }
                return;
            }
            addr += 16;
        }
    }
}

const SMBIOS_BLOB_MAX: usize = 16384;

fn read_table_blob(out: &mut [u8; SMBIOS_BLOB_MAX]) -> usize {
    let addr = SMBIOS_TABLE_ADDR.load(Ordering::Acquire);
    let len = SMBIOS_TABLE_LEN.load(Ordering::Acquire);
    if addr == 0 || len == 0 {
        return 0;
    }
    let n = if len > SMBIOS_BLOB_MAX {
        SMBIOS_BLOB_MAX
    } else {
        len
    };
    let mut i = 0;
    while i + 3 < n {
        let val = crate::hardware_access::mmio_read32(addr + i).unwrap_or(0);
        let bytes = val.to_le_bytes();
        out[i] = bytes[0];
        out[i + 1] = bytes[1];
        out[i + 2] = bytes[2];
        out[i + 3] = bytes[3];
        i += 4;
    }
    while i < n {
        let val = crate::hardware_access::mmio_read32(addr + i).unwrap_or(0);
        out[i] = val as u8;
        i += 1;
    }
    n
}

pub fn find_bios_info() -> Option<BiosInfo> {
    let mut blob = [0u8; SMBIOS_BLOB_MAX];
    let n = read_table_blob(&mut blob);
    if n < 4 {
        return None;
    }
    let mut off = 0;
    while off + 4 <= n {
        let entry_type = blob[off];
        let length = blob[off + 1] as usize;
        if length < 4 {
            break;
        }
        if entry_type == 0 && length >= 18 {
            let info = BiosInfo {
                vendor_str_idx: smbios_read_u8(&blob, off + 4),
                version_str_idx: smbios_read_u8(&blob, off + 5),
                release_date_str_idx: smbios_read_u8(&blob, off + 8),
                rom_size_64k: smbios_read_u8(&blob, off + 9),
                major_release: if length >= 22 {
                    smbios_read_u8(&blob, off + 20)
                } else {
                    0
                },
                minor_release: if length >= 23 {
                    smbios_read_u8(&blob, off + 21)
                } else {
                    0
                },
            };
            return Some(info);
        }
        off = skip_structure(&blob, off, length);
    }
    None
}

const MAX_CPU_ENTRIES: usize = 8;

pub fn find_cpu_info(out: &mut [CpuInfo; MAX_CPU_ENTRIES]) -> usize {
    let mut blob = [0u8; SMBIOS_BLOB_MAX];
    let n = read_table_blob(&mut blob);
    if n < 4 {
        return 0;
    }
    let mut off = 0;
    let mut count = 0;
    while off + 4 <= n && count < MAX_CPU_ENTRIES {
        let entry_type = blob[off];
        let length = blob[off + 1] as usize;
        if length < 4 {
            break;
        }
        if entry_type == 4 && length >= 26 {
            out[count] = CpuInfo {
                socket_str_idx: smbios_read_u8(&blob, off + 4),
                processor_type: smbios_read_u8(&blob, off + 5),
                processor_family: smbios_read_u8(&blob, off + 6),
                processor_id: smbios_read_u64(&blob, off + 8),
                max_speed_mhz: smbios_read_u16(&blob, off + 20),
                current_speed_mhz: smbios_read_u16(&blob, off + 22),
                core_count: if length >= 36 {
                    smbios_read_u8(&blob, off + 35)
                } else {
                    1
                },
                thread_count: if length >= 38 {
                    smbios_read_u8(&blob, off + 37)
                } else {
                    1
                },
                voltage: smbios_read_u8(&blob, off + 17),
            };
            count += 1;
        }
        off = skip_structure(&blob, off, length);
    }
    count
}

const MAX_MEM_MODULES: usize = 32;

pub fn find_memory_modules(out: &mut [MemModuleInfo; MAX_MEM_MODULES]) -> usize {
    let mut blob = [0u8; SMBIOS_BLOB_MAX];
    let n = read_table_blob(&mut blob);
    if n < 4 {
        return 0;
    }
    let mut off = 0;
    let mut count = 0;
    while off + 4 <= n && count < MAX_MEM_MODULES {
        let entry_type = blob[off];
        let length = blob[off + 1] as usize;
        if length < 4 {
            break;
        }
        if entry_type == 17 && length >= 21 {
            let raw_size = smbios_read_u16(&blob, off + 12);
            let size_mb = if raw_size == 0 || raw_size == 0xFFFF {
                0
            } else if raw_size & 0x8000 != 0 {
                (raw_size & 0x7FFF) / 1024
            } else {
                raw_size
            };
            out[count] = MemModuleInfo {
                locator_str_idx: smbios_read_u8(&blob, off + 16),
                size_mb,
                speed_mhz: if length >= 23 {
                    smbios_read_u16(&blob, off + 21)
                } else {
                    0
                },
                memory_type: if length >= 19 {
                    smbios_read_u8(&blob, off + 18)
                } else {
                    0
                },
                form_factor: smbios_read_u8(&blob, off + 14),
                data_width: smbios_read_u16(&blob, off + 10),
                rank: if length >= 28 {
                    smbios_read_u8(&blob, off + 27)
                } else {
                    0
                },
            };
            if size_mb > 0 {
                count += 1;
            }
        }
        off = skip_structure(&blob, off, length);
    }
    count
}

pub fn total_installed_ram_mb() -> u64 {
    let mut modules = [MemModuleInfo {
        locator_str_idx: 0,
        size_mb: 0,
        speed_mhz: 0,
        memory_type: 0,
        form_factor: 0,
        data_width: 0,
        rank: 0,
    }; MAX_MEM_MODULES];
    let n = find_memory_modules(&mut modules);
    let mut total: u64 = 0;
    let mut i = 0;
    while i < n {
        total += modules[i].size_mb as u64;
        i += 1;
    }
    total
}

pub fn smbios_info() -> SmbiosInfo {
    SmbiosInfo {
        base: SMBIOS_BASE.load(Ordering::Acquire),
        version_major: SMBIOS_VERSION_MAJOR.load(Ordering::Acquire),
        version_minor: SMBIOS_VERSION_MINOR.load(Ordering::Acquire),
        table_count: SMBIOS_TABLE_COUNT.load(Ordering::Acquire),
    }
}

pub fn is_present() -> bool {
    if SMBIOS_BASE.load(Ordering::Acquire) != 0 {
        return true;
    }
    parse_smbios();
    SMBIOS_BASE.load(Ordering::Acquire) != 0
}
