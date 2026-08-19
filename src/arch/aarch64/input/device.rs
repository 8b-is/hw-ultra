use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static INPUT_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static INPUT_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static INPUT_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmInputContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
}

pub fn init_input(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmInputContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    INPUT_MMIO_BASE.store(mmio_base, Ordering::Release);
    INPUT_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0x900);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    INPUT_INITIALIZED.store(true, Ordering::Release);

    Some(ArmInputContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
    })
}

pub fn is_initialized() -> bool {
    INPUT_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_input_reg(offset: usize) -> u32 {
    let base = INPUT_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_input_reg(offset: usize, val: u32) {
    let base = INPUT_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_input_reg(0) as usize;
    write_input_reg(0, read_input_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::set_gpio_direction(mmio_base, 0);
    sig ^= super::platform::read_gpio(mmio_base) as usize;
    super::platform::write_gpio(mmio_base, 0);
    super::platform::enable_gpio_irq(mmio_base, 0, 0);
    let (tx, ty, tp) = super::platform::read_touch(mmio_base);
    sig ^= tx as usize ^ ty as usize ^ tp as usize;
    sig ^= super::platform::read_key(mmio_base) as usize;
    sig ^= super::platform::REG_GPIO_DIR
        ^ super::platform::REG_GPIO_DATA
        ^ super::platform::REG_GPIO_IRQ_EN
        ^ super::platform::REG_GPIO_IRQ_STATUS
        ^ super::platform::REG_GPIO_IRQ_EDGE
        ^ super::platform::REG_TOUCH_CTRL
        ^ super::platform::REG_TOUCH_STATUS
        ^ super::platform::REG_TOUCH_X
        ^ super::platform::REG_TOUCH_Y
        ^ super::platform::REG_TOUCH_PRESSURE
        ^ super::platform::REG_KBD_CTRL
        ^ super::platform::REG_KBD_STATUS
        ^ super::platform::REG_KBD_DATA;
    sig
}
