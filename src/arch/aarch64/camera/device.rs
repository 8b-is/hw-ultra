use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static CAMERA_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static CAMERA_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static CAMERA_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmCameraContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
    pub dma_region: usize,
}

pub fn init_camera(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmCameraContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    CAMERA_MMIO_BASE.store(mmio_base, Ordering::Release);
    CAMERA_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0x700);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    let dma_region = super::smmu::map_dma_for_camera(mmio_base, mmio_size);

    CAMERA_INITIALIZED.store(true, Ordering::Release);

    Some(ArmCameraContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
        dma_region,
    })
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

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_camera_reg(0) as usize;
    write_camera_reg(0, read_camera_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::map_dma_for_camera(0, 0);
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::set_csi_lanes(mmio_base, 0);
    super::platform::set_csi_rate(mmio_base, 0);
    super::platform::set_isp_resolution(mmio_base, 0, 0);
    super::platform::set_isp_format(mmio_base, 0);
    super::platform::configure_dma(mmio_base, 0, 0, 0);
    sig ^= super::platform::REG_CSI_CTRL
        ^ super::platform::REG_CSI_STATUS
        ^ super::platform::REG_CSI_LANES
        ^ super::platform::REG_CSI_RATE
        ^ super::platform::REG_ISP_CTRL
        ^ super::platform::REG_ISP_STATUS
        ^ super::platform::REG_ISP_FORMAT
        ^ super::platform::REG_ISP_WIDTH
        ^ super::platform::REG_ISP_HEIGHT
        ^ super::platform::REG_DMA_CTRL
        ^ super::platform::REG_DMA_ADDR
        ^ super::platform::REG_DMA_LEN
        ^ super::platform::REG_DMA_STRIDE;
    sig
}
