use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static SENSOR_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static SENSOR_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static SENSOR_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmSensorContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
}

pub fn init_sensor(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmSensorContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    SENSOR_MMIO_BASE.store(mmio_base, Ordering::Release);
    SENSOR_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0x800);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    SENSOR_INITIALIZED.store(true, Ordering::Release);

    Some(ArmSensorContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
    })
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

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_sensor_reg(0) as usize;
    write_sensor_reg(0, read_sensor_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::i2c_write(mmio_base, 0, 0);
    sig ^= super::platform::i2c_read(mmio_base, 0) as usize;
    super::platform::set_sample_rate(mmio_base, 0);
    super::platform::set_thresholds(mmio_base, 0, 0);
    sig ^= super::platform::REG_I2C_CTRL
        ^ super::platform::REG_I2C_STATUS
        ^ super::platform::REG_I2C_ADDR
        ^ super::platform::REG_I2C_DATA
        ^ super::platform::REG_SPI_CTRL
        ^ super::platform::REG_SPI_STATUS
        ^ super::platform::REG_SPI_DATA
        ^ super::platform::REG_SAMPLE_RATE
        ^ super::platform::REG_SAMPLE_DATA
        ^ super::platform::REG_THRESHOLD_LO
        ^ super::platform::REG_THRESHOLD_HI;
    sig
}
