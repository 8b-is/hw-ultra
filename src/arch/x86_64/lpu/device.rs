use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static LPU_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static LPU_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static LPU_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub const LPU_PCI_CLASS: u8 = 0x12;
pub const LPU_PCI_SUBCLASS: u8 = 0x80;

pub struct X86LpuContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
    pub coherent_dma_base: usize,
}

pub fn init_lpu(bus: u8, dev: u8, func: u8) -> Option<X86LpuContext> {
    let (vendor, device_id) = super::super::gpu::pci::read_ids(bus, dev, func)?;

    super::super::gpu::pci::enable_bus_master(bus, dev, func);
    super::super::gpu::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::super::gpu::pci::decode_bar0(bus, dev, func)?;
    LPU_MMIO_BASE.store(bar_base, Ordering::Release);
    LPU_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x40u8.wrapping_add(bus);
    let msi_cap =
        super::super::gpu::pci::find_capability(bus, dev, func, super::super::gpu::pci::CAP_MSI);
    if msi_cap != 0 {
        super::super::gpu::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::super::gpu::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);
    let dma_base = super::dma::setup_coherent_region(bar_base);

    LPU_INITIALIZED.store(true, Ordering::Release);

    Some(X86LpuContext {
        bus,
        device: dev,
        function: func,
        vendor_id: vendor,
        device_id,
        bar0_base: bar_base,
        bar0_size: bar_size,
        msi_vector: vector,
        coherent_dma_base: dma_base,
    })
}

pub fn is_initialized() -> bool {
    LPU_INITIALIZED.load(Ordering::Acquire)
}

pub(super) fn mmio_base() -> usize {
    LPU_MMIO_BASE.load(Ordering::Acquire)
}

pub fn read_lpu_reg(offset: usize) -> u32 {
    let base = LPU_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_lpu_reg(offset: usize, val: u32) {
    let base = LPU_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = LPU_MMIO_BASE.load(Ordering::Acquire);
    let mut sig =
        (is_initialized() as usize) ^ (LPU_PCI_CLASS as usize) ^ (LPU_PCI_SUBCLASS as usize);
    sig ^= read_lpu_reg(0) as usize;
    write_lpu_reg(0, read_lpu_reg(4));
    let mut scan_buf = [(0u8, 0u8, 0u8, 0u16, 0u16); 1];
    sig ^= super::pci::scan_lpu_devices(&mut scan_buf);
    sig ^= super::pci::read_subsystem_id(bus, dev, func) as usize;
    sig ^= super::pci::read_revision(bus, dev, func) as usize;
    sig ^= super::dma::alloc_coherent(4096).unwrap_or(0);
    sig ^= super::dma::submit_inference_dma(0, 0) as usize;
    sig ^= super::dma::is_dma_complete() as usize;
    sig ^= super::dma::coherent_base();
    sig ^= super::registers::is_ready(base) as usize;
    sig ^= super::registers::is_error(base) as usize;
    super::registers::configure_inference(base, 0, 0, 0);
    super::registers::submit_prefill(base, 0, 0);
    super::registers::submit_decode(base, 0, 0);
    super::registers::submit_speculative(base, 0, 0);
    super::registers::enable_interrupts(base);
    sig ^= super::registers::clear_interrupts(base) as usize;
    sig ^= super::registers::read_version(base) as usize;
    super::registers::halt(base);
    sig
}
