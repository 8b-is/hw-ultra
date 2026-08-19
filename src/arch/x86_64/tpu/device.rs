use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static TPU_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static TPU_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static TPU_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub const TPU_PCI_CLASS: u8 = 0x12;
pub const TPU_PCI_SUBCLASS: u8 = 0x00;

pub struct X86TpuContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
    pub dma_ring_iova: usize,
}

pub fn init_tpu(bus: u8, dev: u8, func: u8) -> Option<X86TpuContext> {
    let (vendor, device_id) = super::super::gpu::pci::read_ids(bus, dev, func)?;

    super::super::gpu::pci::enable_bus_master(bus, dev, func);
    super::super::gpu::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::super::gpu::pci::decode_bar0(bus, dev, func)?;
    TPU_MMIO_BASE.store(bar_base, Ordering::Release);
    TPU_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x30u8.wrapping_add(bus);
    let msi_cap =
        super::super::gpu::pci::find_capability(bus, dev, func, super::super::gpu::pci::CAP_MSI);
    if msi_cap != 0 {
        super::super::gpu::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::super::gpu::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);
    let ring_iova = super::dma::setup_descriptor_ring(bar_base);

    TPU_INITIALIZED.store(true, Ordering::Release);

    Some(X86TpuContext {
        bus,
        device: dev,
        function: func,
        vendor_id: vendor,
        device_id,
        bar0_base: bar_base,
        bar0_size: bar_size,
        msi_vector: vector,
        dma_ring_iova: ring_iova,
    })
}

pub fn is_initialized() -> bool {
    TPU_INITIALIZED.load(Ordering::Acquire)
}

pub(super) fn mmio_base() -> usize {
    TPU_MMIO_BASE.load(Ordering::Acquire)
}

pub fn read_tpu_reg(offset: usize) -> u32 {
    let base = TPU_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_tpu_reg(offset: usize, val: u32) {
    let base = TPU_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = TPU_MMIO_BASE.load(Ordering::Acquire);
    let mut sig =
        (is_initialized() as usize) ^ (TPU_PCI_CLASS as usize) ^ (TPU_PCI_SUBCLASS as usize);
    sig ^= read_tpu_reg(0) as usize;
    write_tpu_reg(0, read_tpu_reg(4));
    let mut scan_buf = [(0u8, 0u8, 0u8, 0u16, 0u16); 1];
    sig ^= super::pci::scan_tpu_devices(&mut scan_buf);
    sig ^= super::pci::read_subsystem_id(bus, dev, func) as usize;
    sig ^= super::pci::read_revision(bus, dev, func) as usize;
    sig ^= super::pci::latency_timer(bus, dev, func) as usize;
    super::pci::set_latency_timer(bus, dev, func, 0);
    sig ^= super::dma::submit_transfer(0, 0, 0, 0) as usize;
    sig ^= super::dma::ring_iova();
    let desc = super::dma::TpuDmaDescriptor {
        src_phys: 0,
        dst_phys: 0,
        length: 0,
        flags: 0,
    };
    sig ^= desc.length as usize
        ^ desc.flags as usize
        ^ desc.src_phys as usize
        ^ desc.dst_phys as usize;
    sig ^= super::registers::is_ready(base) as usize;
    sig ^= super::registers::is_error(base) as usize;
    super::registers::submit_matmul(base, 0, 0);
    super::registers::submit_conv2d(base, 0, 0);
    super::registers::submit_activation(base, 0, 0);
    super::registers::enable_interrupts(base);
    sig ^= super::registers::clear_interrupts(base) as usize;
    sig ^= super::registers::read_version(base) as usize;
    super::registers::halt(base);
    sig
}
