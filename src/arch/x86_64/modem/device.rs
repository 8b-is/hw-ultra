use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static MODEM_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static MODEM_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static MODEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86ModemContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
}

pub fn init_modem(bus: u8, dev: u8, func: u8) -> Option<X86ModemContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    MODEM_MMIO_BASE.store(bar_base, Ordering::Release);
    MODEM_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0xA0u8;
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);
    super::registers::enable(bar_base);

    MODEM_INITIALIZED.store(true, Ordering::Release);

    Some(X86ModemContext {
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

pub fn modem_mmio_base() -> usize {
    MODEM_MMIO_BASE.load(Ordering::Acquire)
}

pub fn modem_mmio_size() -> usize {
    MODEM_MMIO_SIZE.load(Ordering::Acquire)
}

pub fn is_initialized() -> bool {
    MODEM_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_modem_reg(offset: usize) -> u32 {
    let base = MODEM_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_modem_reg(offset: usize, val: u32) {
    let base = MODEM_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = modem_mmio_base();
    let mut sig = base ^ modem_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_modem_reg(0) as usize;
    write_modem_reg(0, read_modem_reg(4));

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
    super::registers::set_network_mode(base, 0);
    super::registers::set_band(base, 0);
    sig ^= super::registers::read_signal_strength(base) as usize;
    sig ^= super::registers::read_link_status(base) as usize;
    sig ^= super::registers::link_up(base) as usize;
    super::registers::enable_tx(base);
    super::registers::enable_rx(base);
    sig ^= super::registers::tx_ready(base) as usize;
    sig ^= super::registers::rx_available(base) as usize;
    super::registers::setup_tx_dma(base, 0, 0);
    super::registers::setup_rx_dma(base, 0, 0);
    super::registers::enable_dma(base);

    sig ^= super::registers::REG_CTRL as usize
        ^ super::registers::REG_STATUS as usize
        ^ super::registers::REG_VERSION as usize
        ^ super::registers::REG_LINK_STATUS as usize
        ^ super::registers::REG_SIGNAL_STRENGTH as usize
        ^ super::registers::REG_NETWORK_MODE as usize
        ^ super::registers::REG_BAND_SELECT as usize;
    sig ^= super::registers::REG_TX_CTRL as usize
        ^ super::registers::REG_TX_DATA as usize
        ^ super::registers::REG_TX_STATUS as usize
        ^ super::registers::REG_RX_CTRL as usize
        ^ super::registers::REG_RX_DATA as usize
        ^ super::registers::REG_RX_STATUS as usize;
    sig ^= super::registers::REG_DMA_TX_ADDR_LO as usize
        ^ super::registers::REG_DMA_TX_ADDR_HI as usize
        ^ super::registers::REG_DMA_TX_LEN as usize
        ^ super::registers::REG_DMA_RX_ADDR_LO as usize
        ^ super::registers::REG_DMA_RX_ADDR_HI as usize
        ^ super::registers::REG_DMA_RX_LEN as usize
        ^ super::registers::REG_DMA_CTRL as usize;
    sig ^= super::registers::REG_IRQ_STATUS as usize ^ super::registers::REG_IRQ_MASK as usize;
    sig ^= super::registers::CTRL_ENABLE as usize
        ^ super::registers::CTRL_RESET as usize
        ^ super::registers::CTRL_TX_EN as usize
        ^ super::registers::CTRL_RX_EN as usize
        ^ super::registers::CTRL_DMA_EN as usize;

    sig
}
