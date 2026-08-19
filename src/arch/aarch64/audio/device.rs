use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static AUDIO_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static AUDIO_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static AUDIO_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmAudioContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
    pub dma_region: usize,
}

pub fn init_audio(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmAudioContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    AUDIO_MMIO_BASE.store(mmio_base, Ordering::Release);
    AUDIO_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0x400);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    let dma_region = super::smmu::map_dma_for_audio(mmio_base, mmio_size);

    AUDIO_INITIALIZED.store(true, Ordering::Release);

    Some(ArmAudioContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
        dma_region,
    })
}

pub fn is_initialized() -> bool {
    AUDIO_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_audio_reg(offset: usize) -> u32 {
    let base = AUDIO_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_audio_reg(offset: usize, val: u32) {
    let base = AUDIO_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_audio_reg(0) as usize;
    write_audio_reg(0, read_audio_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::map_dma_for_audio(0, 0);
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::set_i2s_format(mmio_base, 0);
    super::platform::set_sample_rate(mmio_base, 0);
    super::platform::set_codec_volume(mmio_base, 0);
    super::platform::configure_dma(mmio_base, 0, 0);
    sig ^= super::platform::REG_I2S_CTRL
        ^ super::platform::REG_I2S_STATUS
        ^ super::platform::REG_I2S_FORMAT
        ^ super::platform::REG_I2S_RATE
        ^ super::platform::REG_CODEC_CTRL
        ^ super::platform::REG_CODEC_STATUS
        ^ super::platform::REG_CODEC_VOLUME
        ^ super::platform::REG_DMA_CTRL
        ^ super::platform::REG_DMA_ADDR
        ^ super::platform::REG_DMA_LEN
        ^ super::platform::REG_DMA_STATUS;
    sig
}
