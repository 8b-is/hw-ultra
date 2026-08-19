use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static STORAGE_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static STORAGE_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static STORAGE_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86StorageContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
}

pub fn init_storage(bus: u8, dev: u8, func: u8) -> Option<X86StorageContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    STORAGE_MMIO_BASE.store(bar_base, Ordering::Release);
    STORAGE_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x50u8.wrapping_add(bus);
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);

    STORAGE_INITIALIZED.store(true, Ordering::Release);

    Some(X86StorageContext {
        bus,
        device: dev,
        function: func,
        vendor_id: vendor,
        device_id,
        bar0_base: bar_base,
        bar0_size: bar_size,
        msi_vector: vector,
    })
}

pub fn storage_mmio_base() -> usize {
    STORAGE_MMIO_BASE.load(Ordering::Acquire)
}

pub fn storage_mmio_size() -> usize {
    STORAGE_MMIO_SIZE.load(Ordering::Acquire)
}

pub fn is_initialized() -> bool {
    STORAGE_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_storage_reg(offset: usize) -> u32 {
    let base = STORAGE_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_storage_reg(offset: usize, val: u32) {
    let base = STORAGE_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = storage_mmio_base();
    let mut sig = base ^ storage_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_storage_reg(0) as usize;
    write_storage_reg(0, read_storage_reg(4));
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
    sig ^= super::registers::read_version(base) as usize;
    sig ^= super::registers::read_caps(base) as usize;
    super::registers::enable(base);
    super::registers::setup_admin_queues(base, 0, 0, 1);
    super::registers::ring_doorbell(base, 0, 0);
    sig ^= super::registers::read_vs(base) as usize;
    sig ^= super::registers::read_cq_head(base, 0) as usize;
    sig
}
