use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static UEFI_SYSTEM_TABLE: AtomicUsize = AtomicUsize::new(0);
static UEFI_REVISION_MAJOR: AtomicU8 = AtomicU8::new(0);
static UEFI_REVISION_MINOR: AtomicU8 = AtomicU8::new(0);
static UEFI_MEMORY_MAP_BASE: AtomicUsize = AtomicUsize::new(0);
static UEFI_MEMORY_MAP_SIZE: AtomicUsize = AtomicUsize::new(0);
static UEFI_MMAP_DESC_SIZE: AtomicUsize = AtomicUsize::new(0);
static UEFI_MMAP_DESC_COUNT: AtomicUsize = AtomicUsize::new(0);
static UEFI_RUNTIME_SERVICES: AtomicUsize = AtomicUsize::new(0);
static UEFI_GOP_FB_BASE: AtomicUsize = AtomicUsize::new(0);
static UEFI_GOP_FB_SIZE: AtomicUsize = AtomicUsize::new(0);
static UEFI_GOP_H_RES: AtomicUsize = AtomicUsize::new(0);
static UEFI_GOP_V_RES: AtomicUsize = AtomicUsize::new(0);
static UEFI_GOP_STRIDE: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct UefiInfo {
    pub system_table: usize,
    pub revision_major: u8,
    pub revision_minor: u8,
    pub memory_map_base: usize,
    pub memory_map_size: usize,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum UefiMemoryType {
    Reserved = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    Conventional = 7,
    Unusable = 8,
    AcpiReclaim = 9,
    AcpiNvs = 10,
    Mmio = 11,
    MmioPortSpace = 12,
    PalCode = 13,
    Persistent = 14,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct UefiMemoryDescriptor {
    pub memory_type: u32,
    pub padding: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[derive(Copy, Clone)]
pub struct GopInfo {
    pub framebuffer_base: usize,
    pub framebuffer_size: usize,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixels_per_scan_line: u32,
}

#[derive(Copy, Clone)]
pub struct RuntimeServicesTable {
    pub address: usize,
    pub get_time: usize,
    pub set_time: usize,
    pub get_variable: usize,
    pub set_variable: usize,
    pub reset_system: usize,
}

pub fn parse_uefi() {
    let candidate = 0x8000_0000usize;
    let sig = crate::hardware_access::mmio_read32(candidate).unwrap_or(0);
    if sig != 0x20494249 {
        return;
    }
    UEFI_SYSTEM_TABLE.store(candidate, Ordering::Release);
    let revision = crate::hardware_access::mmio_read32(candidate + 8).unwrap_or(0);
    UEFI_REVISION_MAJOR.store(((revision >> 16) & 0xFF) as u8, Ordering::Release);
    UEFI_REVISION_MINOR.store((revision & 0xFF) as u8, Ordering::Release);
    let mmap_ptr = crate::hardware_access::mmio_read32(candidate + 56).unwrap_or(0) as usize;
    let mmap_size = crate::hardware_access::mmio_read32(candidate + 60).unwrap_or(0) as usize;
    if mmap_ptr != 0 {
        UEFI_MEMORY_MAP_BASE.store(mmap_ptr, Ordering::Release);
        UEFI_MEMORY_MAP_SIZE.store(mmap_size, Ordering::Release);
    }
}

pub fn set_memory_map(base: usize, size: usize, desc_size: usize) {
    UEFI_MEMORY_MAP_BASE.store(base, Ordering::Release);
    UEFI_MEMORY_MAP_SIZE.store(size, Ordering::Release);
    UEFI_MMAP_DESC_SIZE.store(desc_size, Ordering::Release);
    if desc_size > 0 {
        UEFI_MMAP_DESC_COUNT.store(size / desc_size, Ordering::Release);
    }
}

pub fn memory_map_descriptor_count() -> usize {
    UEFI_MMAP_DESC_COUNT.load(Ordering::Acquire)
}

pub fn read_memory_descriptor(index: usize) -> Option<UefiMemoryDescriptor> {
    let base = UEFI_MEMORY_MAP_BASE.load(Ordering::Acquire);
    let count = UEFI_MMAP_DESC_COUNT.load(Ordering::Acquire);
    let desc_size = UEFI_MMAP_DESC_SIZE.load(Ordering::Acquire);
    if base == 0 || index >= count || desc_size < 40 {
        return None;
    }
    let addr = base + index * desc_size;
    let mem_type = crate::hardware_access::mmio_read32(addr).unwrap_or(0);
    let phys_start_lo = crate::hardware_access::mmio_read32(addr + 8).unwrap_or(0) as u64;
    let phys_start_hi = crate::hardware_access::mmio_read32(addr + 12).unwrap_or(0) as u64;
    let virt_start_lo = crate::hardware_access::mmio_read32(addr + 16).unwrap_or(0) as u64;
    let virt_start_hi = crate::hardware_access::mmio_read32(addr + 20).unwrap_or(0) as u64;
    let pages_lo = crate::hardware_access::mmio_read32(addr + 24).unwrap_or(0) as u64;
    let pages_hi = crate::hardware_access::mmio_read32(addr + 28).unwrap_or(0) as u64;
    let attr_lo = crate::hardware_access::mmio_read32(addr + 32).unwrap_or(0) as u64;
    let attr_hi = crate::hardware_access::mmio_read32(addr + 36).unwrap_or(0) as u64;
    Some(UefiMemoryDescriptor {
        memory_type: mem_type,
        padding: 0,
        physical_start: phys_start_lo | (phys_start_hi << 32),
        virtual_start: virt_start_lo | (virt_start_hi << 32),
        number_of_pages: pages_lo | (pages_hi << 32),
        attribute: attr_lo | (attr_hi << 32),
    })
}

pub fn total_conventional_memory() -> u64 {
    let count = UEFI_MMAP_DESC_COUNT.load(Ordering::Acquire);
    let mut total_pages: u64 = 0;
    let mut i = 0;
    while i < count {
        if let Some(desc) = read_memory_descriptor(i) {
            if desc.memory_type == UefiMemoryType::Conventional as u32 {
                total_pages += desc.number_of_pages;
            }
        }
        i += 1;
    }
    total_pages * 4096
}

pub fn set_runtime_services(addr: usize) {
    UEFI_RUNTIME_SERVICES.store(addr, Ordering::Release);
}

pub fn runtime_services() -> Option<RuntimeServicesTable> {
    let addr = UEFI_RUNTIME_SERVICES.load(Ordering::Acquire);
    if addr == 0 {
        return None;
    }
    let get_time = crate::hardware_access::mmio_read32(addr + 24).unwrap_or(0) as usize;
    let set_time = crate::hardware_access::mmio_read32(addr + 32).unwrap_or(0) as usize;
    let get_variable = crate::hardware_access::mmio_read32(addr + 72).unwrap_or(0) as usize;
    let set_variable = crate::hardware_access::mmio_read32(addr + 80).unwrap_or(0) as usize;
    let reset_system = crate::hardware_access::mmio_read32(addr + 104).unwrap_or(0) as usize;
    Some(RuntimeServicesTable {
        address: addr,
        get_time,
        set_time,
        get_variable,
        set_variable,
        reset_system,
    })
}

pub fn set_gop(fb_base: usize, fb_size: usize, h_res: u32, v_res: u32, stride: u32) {
    UEFI_GOP_FB_BASE.store(fb_base, Ordering::Release);
    UEFI_GOP_FB_SIZE.store(fb_size, Ordering::Release);
    UEFI_GOP_H_RES.store(h_res as usize, Ordering::Release);
    UEFI_GOP_V_RES.store(v_res as usize, Ordering::Release);
    UEFI_GOP_STRIDE.store(stride as usize, Ordering::Release);
}

pub fn gop_info() -> Option<GopInfo> {
    let base = UEFI_GOP_FB_BASE.load(Ordering::Acquire);
    if base == 0 {
        return None;
    }
    Some(GopInfo {
        framebuffer_base: base,
        framebuffer_size: UEFI_GOP_FB_SIZE.load(Ordering::Acquire),
        horizontal_resolution: UEFI_GOP_H_RES.load(Ordering::Acquire) as u32,
        vertical_resolution: UEFI_GOP_V_RES.load(Ordering::Acquire) as u32,
        pixels_per_scan_line: UEFI_GOP_STRIDE.load(Ordering::Acquire) as u32,
    })
}

pub fn uefi_info() -> UefiInfo {
    UefiInfo {
        system_table: UEFI_SYSTEM_TABLE.load(Ordering::Acquire),
        revision_major: UEFI_REVISION_MAJOR.load(Ordering::Acquire),
        revision_minor: UEFI_REVISION_MINOR.load(Ordering::Acquire),
        memory_map_base: UEFI_MEMORY_MAP_BASE.load(Ordering::Acquire),
        memory_map_size: UEFI_MEMORY_MAP_SIZE.load(Ordering::Acquire),
    }
}

pub fn is_present() -> bool {
    if UEFI_SYSTEM_TABLE.load(Ordering::Acquire) != 0 {
        return true;
    }
    parse_uefi();
    UEFI_SYSTEM_TABLE.load(Ordering::Acquire) != 0
}
