use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static CAMERA_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static CAMERA_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static CAMERA_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86CameraContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
}

pub fn init_camera(bus: u8, dev: u8, func: u8) -> Option<X86CameraContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    CAMERA_MMIO_BASE.store(bar_base, Ordering::Release);
    CAMERA_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x70u8;
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);
    super::registers::enable(bar_base);

    CAMERA_INITIALIZED.store(true, Ordering::Release);

    Some(X86CameraContext {
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

pub fn camera_mmio_base() -> usize {
    CAMERA_MMIO_BASE.load(Ordering::Acquire)
}

pub fn camera_mmio_size() -> usize {
    CAMERA_MMIO_SIZE.load(Ordering::Acquire)
}

pub fn is_initialized() -> bool {
    CAMERA_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_camera_reg(offset: usize) -> u32 {
    let base = CAMERA_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_camera_reg(offset: usize, val: u32) {
    let base = CAMERA_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = camera_mmio_base();
    let mut sig = base ^ camera_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_camera_reg(0) as usize;
    write_camera_reg(0, read_camera_reg(4));

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
    sig ^= super::registers::read_irq_status(base) as usize;
    super::registers::clear_irq(base, 0xFFFF_FFFF);
    super::registers::set_resolution(base, 1920, 1080);
    super::registers::set_format(base, 0);
    super::registers::set_stride(base, 0);
    super::registers::enable_csi(base);
    super::registers::reset_csi(base);
    super::registers::set_csi_lanes(base, 4);
    super::registers::set_csi_data_rate(base, 0);
    super::registers::start_capture(base);
    super::registers::start_continuous(base);
    super::registers::setup_dma(base, 0, 0);

    sig ^= super::registers::REG_ISP_CTRL as usize
        ^ super::registers::REG_ISP_STATUS as usize
        ^ super::registers::REG_ISP_VERSION as usize
        ^ super::registers::REG_ISP_FORMAT as usize
        ^ super::registers::REG_ISP_WIDTH as usize
        ^ super::registers::REG_ISP_HEIGHT as usize
        ^ super::registers::REG_ISP_STRIDE as usize;
    sig ^= super::registers::REG_CSI_CTRL as usize
        ^ super::registers::REG_CSI_LANES as usize
        ^ super::registers::REG_CSI_DATA_RATE as usize
        ^ super::registers::REG_CSI_STATUS as usize;
    sig ^= super::registers::REG_DMA_CTRL as usize
        ^ super::registers::REG_DMA_ADDR_LO as usize
        ^ super::registers::REG_DMA_ADDR_HI as usize
        ^ super::registers::REG_DMA_LENGTH as usize
        ^ super::registers::REG_DMA_STATUS as usize;
    sig ^= super::registers::REG_IRQ_STATUS as usize ^ super::registers::REG_IRQ_MASK as usize;
    sig ^= super::registers::ISP_CTRL_ENABLE as usize
        ^ super::registers::ISP_CTRL_RESET as usize
        ^ super::registers::ISP_CTRL_CAPTURE as usize
        ^ super::registers::ISP_CTRL_CONTINUOUS as usize;
    sig ^= super::registers::CSI_CTRL_ENABLE as usize ^ super::registers::CSI_CTRL_RESET as usize;

    sig
}
