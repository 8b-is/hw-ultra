use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static DISPLAY_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static DISPLAY_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static DISPLAY_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmDisplayContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
    pub fb_iova: usize,
}

pub fn init_display(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmDisplayContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    DISPLAY_MMIO_BASE.store(mmio_base, Ordering::Release);
    DISPLAY_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0x300);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    let fb_iova = super::smmu::map_framebuffer(mmio_base, mmio_size);

    DISPLAY_INITIALIZED.store(true, Ordering::Release);

    Some(ArmDisplayContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
        fb_iova,
    })
}

pub fn is_initialized() -> bool {
    DISPLAY_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_display_reg(offset: usize) -> u32 {
    let base = DISPLAY_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_display_reg(offset: usize, val: u32) {
    let base = DISPLAY_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_display_reg(0) as usize;
    write_display_reg(0, read_display_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::map_framebuffer(0, 0);
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::set_framebuffer_base(mmio_base, 0);
    super::platform::set_framebuffer_stride(mmio_base, 0);
    super::platform::set_timing(mmio_base, 0, 0);
    super::platform::set_pixel_format(mmio_base, 0);
    sig
}
