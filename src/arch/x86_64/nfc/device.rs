use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static NFC_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static NFC_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static NFC_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86NfcContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
}

pub fn init_nfc(bus: u8, dev: u8, func: u8) -> Option<X86NfcContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    NFC_MMIO_BASE.store(bar_base, Ordering::Release);
    NFC_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0xB0u8;
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);
    super::registers::enable(bar_base);

    NFC_INITIALIZED.store(true, Ordering::Release);

    Some(X86NfcContext {
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

pub fn nfc_mmio_base() -> usize {
    NFC_MMIO_BASE.load(Ordering::Acquire)
}

pub fn nfc_mmio_size() -> usize {
    NFC_MMIO_SIZE.load(Ordering::Acquire)
}

pub fn is_initialized() -> bool {
    NFC_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_nfc_reg(offset: usize) -> u32 {
    let base = NFC_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_nfc_reg(offset: usize, val: u32) {
    let base = NFC_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = nfc_mmio_base();
    let mut sig = base ^ nfc_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_nfc_reg(0) as usize;
    write_nfc_reg(0, read_nfc_reg(4));

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
    super::registers::enable_rf_field(base);
    super::registers::disable_rf_field(base);
    sig ^= super::registers::rf_field_detected(base) as usize;
    super::registers::enable_polling(base);
    sig ^= super::registers::tag_present(base) as usize;
    sig ^= super::registers::read_tag_type(base) as usize;
    sig ^= super::registers::read_tag_data(base) as usize;
    sig ^= super::registers::read_tag_length(base) as usize;
    super::registers::transmit(base, 0, 0);
    sig ^= super::registers::rx_available(base) as usize;
    sig ^= super::registers::read_rx_data(base) as usize;
    sig ^= super::registers::read_rx_length(base) as usize;

    sig ^= super::registers::REG_CTRL as usize
        ^ super::registers::REG_STATUS as usize
        ^ super::registers::REG_VERSION as usize
        ^ super::registers::REG_RF_CTRL as usize
        ^ super::registers::REG_RF_STATUS as usize
        ^ super::registers::REG_RF_FIELD as usize;
    sig ^= super::registers::REG_TAG_TYPE as usize
        ^ super::registers::REG_TAG_DATA as usize
        ^ super::registers::REG_TAG_LENGTH as usize
        ^ super::registers::REG_TAG_STATUS as usize;
    sig ^= super::registers::REG_TX_DATA as usize
        ^ super::registers::REG_TX_LENGTH as usize
        ^ super::registers::REG_TX_CTRL as usize;
    sig ^= super::registers::REG_RX_DATA as usize
        ^ super::registers::REG_RX_LENGTH as usize
        ^ super::registers::REG_RX_STATUS as usize;
    sig ^= super::registers::REG_IRQ_STATUS as usize ^ super::registers::REG_IRQ_MASK as usize;
    sig ^= super::registers::CTRL_ENABLE as usize
        ^ super::registers::CTRL_RESET as usize
        ^ super::registers::CTRL_RF_EN as usize
        ^ super::registers::CTRL_POLL_EN as usize;

    sig
}
