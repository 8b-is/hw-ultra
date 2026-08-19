use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static DISPLAY_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static DISPLAY_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static DISPLAY_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86DisplayContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
    pub vram_iova: usize,
}

pub fn init_display(bus: u8, dev: u8, func: u8) -> Option<X86DisplayContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    DISPLAY_MMIO_BASE.store(bar_base, Ordering::Release);
    DISPLAY_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x30u8.wrapping_add(bus);
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    let vram_iova = super::vram::map_vram_region(bar_base, bar_size);
    super::vram::set_write_combining(bar_base, bar_size);

    DISPLAY_INITIALIZED.store(true, Ordering::Release);

    Some(X86DisplayContext {
        bus,
        device: dev,
        function: func,
        vendor_id: vendor,
        device_id,
        bar0_base: bar_base,
        bar0_size: bar_size,
        msi_vector: vector,
        vram_iova,
    })
}

pub fn display_mmio_base() -> usize {
    DISPLAY_MMIO_BASE.load(Ordering::Acquire)
}

pub fn display_mmio_size() -> usize {
    DISPLAY_MMIO_SIZE.load(Ordering::Acquire)
}

pub fn is_initialized() -> bool {
    DISPLAY_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_display_reg(offset: usize) -> u32 {
    let base = DISPLAY_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_display_reg(offset: usize, val: u32) {
    let base = DISPLAY_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let mut sig = display_mmio_base() ^ display_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_display_reg(0) as usize;
    write_display_reg(0, read_display_reg(4));
    let (cls, sub, pi) = super::pci::read_class(bus, dev, func);
    sig ^= cls as usize ^ sub as usize ^ pi as usize;
    sig ^= super::pci::read_irq_line(bus, dev, func) as usize;
    sig ^= super::pci::CAP_MSIX as usize
        ^ super::pci::CAP_PCIE as usize
        ^ super::pci::CAP_PM as usize
        ^ super::pci::CMD_IO_SPACE as usize;
    super::msi::disable_msi(bus, dev, func, 0);
    sig ^= super::msi::allocated_vectors(bus, dev, func, 0);
    sig ^= super::msi::MSI_CTRL_OFFSET as usize;
    sig ^= super::vram::read_pat() as usize;
    sig ^= super::vram::vram_iova() ^ super::vram::vram_size();
    sig ^= super::vram::allocate_framebuffer(0, 0).unwrap_or(0);
    sig
}
