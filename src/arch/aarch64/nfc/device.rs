use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static NFC_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static NFC_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static NFC_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmNfcContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
}

pub fn init_nfc(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmNfcContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    NFC_MMIO_BASE.store(mmio_base, Ordering::Release);
    NFC_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0xB00);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    NFC_INITIALIZED.store(true, Ordering::Release);

    Some(ArmNfcContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
    })
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

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_nfc_reg(0) as usize;
    write_nfc_reg(0, read_nfc_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::enable_rf_field(mmio_base);
    super::platform::disable_rf_field(mmio_base);
    let (td, tl) = super::platform::read_tag_data(mmio_base);
    sig ^= td as usize ^ tl as usize;
    sig ^= super::platform::get_tag_type(mmio_base) as usize;
    sig ^= super::platform::REG_I2C_CTRL
        ^ super::platform::REG_I2C_STATUS
        ^ super::platform::REG_I2C_ADDR
        ^ super::platform::REG_I2C_DATA
        ^ super::platform::REG_SPI_CTRL
        ^ super::platform::REG_SPI_STATUS
        ^ super::platform::REG_SPI_DATA
        ^ super::platform::REG_RF_CTRL
        ^ super::platform::REG_RF_STATUS
        ^ super::platform::REG_RF_FIELD
        ^ super::platform::REG_TAG_DATA
        ^ super::platform::REG_TAG_LEN
        ^ super::platform::REG_TAG_TYPE;
    sig
}
