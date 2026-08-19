use core::slice;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::x86_64::mmu::paging::phys_to_virt_addr;

fn read_bytes(paddr: usize, buf: &mut [u8]) {
    let va = phys_to_virt_addr(paddr) as *const u8;
    unsafe {
        let src = slice::from_raw_parts(va, buf.len());
        buf.copy_from_slice(src);
    }
}

fn read_u8(paddr: usize) -> u8 {
    let mut b = [0u8; 1];
    read_bytes(paddr, &mut b);
    b[0]
}

fn read_u16(paddr: usize) -> u16 {
    let mut b = [0u8; 2];
    read_bytes(paddr, &mut b);
    u16::from_le_bytes(b)
}

fn read_u32(paddr: usize) -> u32 {
    let mut b = [0u8; 4];
    read_bytes(paddr, &mut b);
    u32::from_le_bytes(b)
}

fn read_u64(paddr: usize) -> u64 {
    let mut b = [0u8; 8];
    read_bytes(paddr, &mut b);
    u64::from_le_bytes(b)
}

static RSDP_ADDR: AtomicUsize = AtomicUsize::new(0);
static RSDT_ADDR: AtomicUsize = AtomicUsize::new(0);
static XSDT_ADDR: AtomicUsize = AtomicUsize::new(0);
static ACPI_REVISION: AtomicUsize = AtomicUsize::new(0);

fn find_rsdp() -> usize {
    let cached = RSDP_ADDR.load(Ordering::Acquire);
    if cached != 0 {
        return cached;
    }
    const START: usize = 0xE0000;
    const END: usize = 0x100000;
    let mut sig = [0u8; 8];
    let mut p = START;
    while p < END {
        read_bytes(p, &mut sig);
        if &sig == b"RSD PTR " {
            let mut sum: u8 = 0;
            let mut i = 0;
            while i < 20 {
                sum = sum.wrapping_add(read_u8(p + i));
                i += 1;
            }
            if sum == 0 {
                RSDP_ADDR.store(p, Ordering::Release);
                let revision = read_u8(p + 15) as usize;
                ACPI_REVISION.store(revision, Ordering::Release);
                let rsdt = read_u32(p + 16) as usize;
                RSDT_ADDR.store(rsdt, Ordering::Release);
                if revision >= 2 {
                    let xsdt = read_u64(p + 24) as usize;
                    if xsdt != 0 {
                        XSDT_ADDR.store(xsdt, Ordering::Release);
                    }
                }
                return p;
            }
        }
        p += 16;
    }
    0
}

const MAX_ACPI_TABLES: usize = 64;

fn find_table(signature: &[u8; 4]) -> Option<usize> {
    if find_rsdp() == 0 {
        return None;
    }
    let xsdt = XSDT_ADDR.load(Ordering::Acquire);
    if xsdt != 0 {
        let len = read_u32(xsdt + 4) as usize;
        if len > 36 {
            let entries = (len - 36) / 8;
            let count = if entries > MAX_ACPI_TABLES {
                MAX_ACPI_TABLES
            } else {
                entries
            };
            let mut i = 0;
            while i < count {
                let addr = read_u64(xsdt + 36 + i * 8) as usize;
                if addr != 0 {
                    let mut sig = [0u8; 4];
                    read_bytes(addr, &mut sig);
                    if &sig == signature {
                        return Some(addr);
                    }
                }
                i += 1;
            }
        }
    }
    let rsdt = RSDT_ADDR.load(Ordering::Acquire);
    if rsdt != 0 {
        let len = read_u32(rsdt + 4) as usize;
        if len > 36 {
            let entries = (len - 36) / 4;
            let count = if entries > MAX_ACPI_TABLES {
                MAX_ACPI_TABLES
            } else {
                entries
            };
            let mut i = 0;
            while i < count {
                let addr = read_u32(rsdt + 36 + i * 4) as usize;
                if addr != 0 {
                    let mut sig = [0u8; 4];
                    read_bytes(addr, &mut sig);
                    if &sig == signature {
                        return Some(addr);
                    }
                }
                i += 1;
            }
        }
    }
    None
}

pub fn acpi_revision() -> usize {
    if RSDP_ADDR.load(Ordering::Acquire) == 0 {
        find_rsdp();
    }
    ACPI_REVISION.load(Ordering::Acquire)
}

pub fn is_present() -> bool {
    find_rsdp() != 0
}

pub fn find_ioapic_base() -> Option<usize> {
    let madt = find_table(b"APIC")?;
    let len = read_u32(madt + 4) as usize;
    let mut off = 44;
    while off + 2 <= len {
        let entry_type = read_u8(madt + off);
        let entry_len = read_u8(madt + off + 1) as usize;
        if entry_type == 1 && entry_len >= 12 {
            return Some(read_u32(madt + off + 4) as usize);
        }
        if entry_len == 0 {
            break;
        }
        off += entry_len;
    }
    None
}

pub fn find_vtd_base() -> Option<usize> {
    let dmar = find_table(b"DMAR")?;
    let len = read_u32(dmar + 4) as usize;
    let host_addr_width = read_u8(dmar + 36);
    if host_addr_width == 0 {
        return None;
    }
    let mut off = 48;
    while off + 16 <= len {
        let entry_type = read_u16(dmar + off);
        let entry_len = read_u16(dmar + off + 2) as usize;
        if entry_type == 0 && entry_len >= 16 {
            let base = read_u64(dmar + off + 8) as usize;
            if base != 0 {
                return Some(base);
            }
        }
        if entry_len < 4 {
            break;
        }
        off += entry_len;
    }
    None
}

#[derive(Copy, Clone)]
pub struct FadtInfo {
    pub sci_interrupt: u16,
    pub smi_cmd_port: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub pm1a_evt_blk: u32,
    pub pm1a_cnt_blk: u32,
    pub pm_timer_blk: u32,
    pub pm_timer_len: u8,
    pub reset_reg_addr: u64,
    pub reset_value: u8,
    pub century: u8,
    pub flags: u32,
}

static FADT_CACHED: AtomicUsize = AtomicUsize::new(0);
static FADT_ADDR: AtomicUsize = AtomicUsize::new(0);

pub fn parse_fadt() -> Option<FadtInfo> {
    let fadt = find_table(b"FACP")?;
    let len = read_u32(fadt + 4) as usize;
    if len < 116 {
        return None;
    }
    FADT_ADDR.store(fadt, Ordering::Release);
    FADT_CACHED.store(1, Ordering::Release);
    Some(FadtInfo {
        sci_interrupt: read_u16(fadt + 46),
        smi_cmd_port: read_u32(fadt + 48),
        acpi_enable: read_u8(fadt + 52),
        acpi_disable: read_u8(fadt + 53),
        pm1a_evt_blk: read_u32(fadt + 56),
        pm1a_cnt_blk: read_u32(fadt + 64),
        pm_timer_blk: read_u32(fadt + 76),
        pm_timer_len: read_u8(fadt + 89),
        reset_reg_addr: if len >= 129 { read_u64(fadt + 116) } else { 0 },
        reset_value: if len >= 129 { read_u8(fadt + 128) } else { 0 },
        century: read_u8(fadt + 108),
        flags: read_u32(fadt + 112),
    })
}

pub fn fadt_dsdt_addr() -> Option<usize> {
    let fadt = find_table(b"FACP")?;
    let len = read_u32(fadt + 4) as usize;
    if len >= 148 {
        let x_dsdt = read_u64(fadt + 140) as usize;
        if x_dsdt != 0 {
            return Some(x_dsdt);
        }
    }
    let dsdt = read_u32(fadt + 40) as usize;
    if dsdt != 0 {
        Some(dsdt)
    } else {
        None
    }
}

#[derive(Copy, Clone)]
pub struct HpetInfo {
    pub event_timer_block_id: u32,
    pub base_address: u64,
    pub hpet_number: u8,
    pub min_tick: u16,
}

pub fn parse_hpet() -> Option<HpetInfo> {
    let hpet = find_table(b"HPET")?;
    let len = read_u32(hpet + 4) as usize;
    if len < 56 {
        return None;
    }
    let block_id = read_u32(hpet + 36);
    let addr_space = read_u8(hpet + 40);
    if addr_space != 0 {
        return None;
    }
    let base = read_u64(hpet + 44);
    let hpet_number = read_u8(hpet + 52);
    let min_tick = read_u16(hpet + 53);
    Some(HpetInfo {
        event_timer_block_id: block_id,
        base_address: base,
        hpet_number,
        min_tick,
    })
}

#[derive(Copy, Clone)]
pub struct McfgEntry {
    pub base_address: u64,
    pub segment_group: u16,
    pub start_bus: u8,
    pub end_bus: u8,
}

const MAX_MCFG_ENTRIES: usize = 8;

pub fn parse_mcfg(out: &mut [McfgEntry; MAX_MCFG_ENTRIES]) -> usize {
    let mcfg = match find_table(b"MCFG") {
        Some(a) => a,
        None => return 0,
    };
    let len = read_u32(mcfg + 4) as usize;
    if len < 44 {
        return 0;
    }
    let data_len = len - 44;
    let entry_count = data_len / 16;
    let count = if entry_count > MAX_MCFG_ENTRIES {
        MAX_MCFG_ENTRIES
    } else {
        entry_count
    };
    let mut i = 0;
    while i < count {
        let base = 44 + i * 16;
        out[i] = McfgEntry {
            base_address: read_u64(mcfg + base),
            segment_group: read_u16(mcfg + base + 8),
            start_bus: read_u8(mcfg + base + 10),
            end_bus: read_u8(mcfg + base + 11),
        };
        i += 1;
    }
    count
}

pub fn table_count() -> usize {
    if find_rsdp() == 0 {
        return 0;
    }
    let mut total = 0;
    let xsdt = XSDT_ADDR.load(Ordering::Acquire);
    if xsdt != 0 {
        let len = read_u32(xsdt + 4) as usize;
        if len > 36 {
            total = (len - 36) / 8;
        }
    } else {
        let rsdt = RSDT_ADDR.load(Ordering::Acquire);
        if rsdt != 0 {
            let len = read_u32(rsdt + 4) as usize;
            if len > 36 {
                total = (len - 36) / 4;
            }
        }
    }
    if total > MAX_ACPI_TABLES {
        MAX_ACPI_TABLES
    } else {
        total
    }
}
