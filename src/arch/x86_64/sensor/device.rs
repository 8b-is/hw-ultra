use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static SENSOR_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static SENSOR_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static SENSOR_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86SensorContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
}

pub fn init_sensor(bus: u8, dev: u8, func: u8) -> Option<X86SensorContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    SENSOR_MMIO_BASE.store(bar_base, Ordering::Release);
    SENSOR_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x80u8;
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);
    super::registers::enable(bar_base);

    SENSOR_INITIALIZED.store(true, Ordering::Release);

    Some(X86SensorContext {
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

pub fn sensor_mmio_base() -> usize {
    SENSOR_MMIO_BASE.load(Ordering::Acquire)
}

pub fn sensor_mmio_size() -> usize {
    SENSOR_MMIO_SIZE.load(Ordering::Acquire)
}

pub fn is_initialized() -> bool {
    SENSOR_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_sensor_reg(offset: usize) -> u32 {
    let base = SENSOR_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_sensor_reg(offset: usize, val: u32) {
    let base = SENSOR_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = sensor_mmio_base();
    let mut sig = base ^ sensor_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_sensor_reg(0) as usize;
    write_sensor_reg(0, read_sensor_reg(4));

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
    super::registers::set_sample_rate(base, 0);
    let (sx, sy, sz) = super::registers::read_data_xyz(base);
    sig ^= sx as usize ^ sy as usize ^ sz as usize;
    sig ^= super::registers::read_raw(base) as usize;
    super::registers::set_thresholds(base, 0, 0);
    super::registers::set_calibration(base, 0);
    super::registers::enable_fifo(base);
    sig ^= super::registers::read_fifo(base) as usize;
    sig ^= super::registers::fifo_level(base) as usize;
    sig ^= super::registers::read_sensor_type(base) as usize;

    sig ^= super::registers::REG_CTRL as usize
        ^ super::registers::REG_STATUS as usize
        ^ super::registers::REG_VERSION as usize
        ^ super::registers::REG_SENSOR_TYPE as usize
        ^ super::registers::REG_SAMPLE_RATE as usize;
    sig ^= super::registers::REG_DATA_X as usize
        ^ super::registers::REG_DATA_Y as usize
        ^ super::registers::REG_DATA_Z as usize
        ^ super::registers::REG_DATA_RAW as usize;
    sig ^= super::registers::REG_THRESHOLD_LO as usize
        ^ super::registers::REG_THRESHOLD_HI as usize
        ^ super::registers::REG_CALIBRATION as usize;
    sig ^= super::registers::REG_FIFO_CTRL as usize
        ^ super::registers::REG_FIFO_STATUS as usize
        ^ super::registers::REG_FIFO_DATA as usize;
    sig ^= super::registers::REG_IRQ_STATUS as usize ^ super::registers::REG_IRQ_MASK as usize;
    sig ^= super::registers::CTRL_ENABLE as usize
        ^ super::registers::CTRL_RESET as usize
        ^ super::registers::CTRL_FIFO_EN as usize
        ^ super::registers::CTRL_IRQ_EN as usize;

    sig
}
