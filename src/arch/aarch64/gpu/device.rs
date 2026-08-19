use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static GPU_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static GPU_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static GPU_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmGpuContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
    pub vram_iova: usize,
}

pub fn init_gpu(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmGpuContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    GPU_MMIO_BASE.store(mmio_base, Ordering::Release);
    GPU_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0x100);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    let vram_iova = super::vram::map_vram_region(mmio_base, mmio_size);

    GPU_INITIALIZED.store(true, Ordering::Release);

    Some(ArmGpuContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
        vram_iova,
    })
}

pub fn is_initialized() -> bool {
    GPU_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_gpu_reg(offset: usize) -> u32 {
    let base = GPU_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_gpu_reg(offset: usize, val: u32) {
    let base = GPU_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_gpu_reg(0) as usize;
    write_gpu_reg(0, read_gpu_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::map_dma_for_device(0, 0);
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::vram::clean_cache_range(mmio_base, 64);
    super::vram::invalidate_cache_range(mmio_base, 64);
    sig ^= super::vram::vram_iova() ^ super::vram::vram_size();
    sig ^= super::vram::allocate_framebuffer(0, 0).unwrap_or(0);
    sig
}
