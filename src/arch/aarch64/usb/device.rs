use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static USB_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static USB_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static USB_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmUsbContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
    pub dma_region: usize,
}

pub fn init_usb(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmUsbContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    USB_MMIO_BASE.store(mmio_base, Ordering::Release);
    USB_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0x600);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    let dma_region = super::smmu::map_dma_for_usb(mmio_base, mmio_size);

    USB_INITIALIZED.store(true, Ordering::Release);

    Some(ArmUsbContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
        dma_region,
    })
}

pub fn is_initialized() -> bool {
    USB_INITIALIZED.load(Ordering::Acquire)
}

pub fn read_usb_reg(offset: usize) -> u32 {
    let base = USB_MMIO_BASE.load(Ordering::Acquire);
    if base == 0 {
        return 0;
    }
    unsafe { super::super::mmio::mmio_read32(base + offset) }
}

pub fn write_usb_reg(offset: usize, val: u32) {
    let base = USB_MMIO_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            super::super::mmio::mmio_write32(base + offset, val);
        }
    }
}

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_usb_reg(0) as usize;
    write_usb_reg(0, read_usb_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::map_dma_for_usb(0, 0);
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::init_phy(mmio_base);
    super::platform::set_otg_mode(mmio_base, false);
    super::platform::configure_endpoint(mmio_base, 0, 0, 0);
    sig ^= super::platform::REG_PHY_CTRL
        ^ super::platform::REG_PHY_STATUS
        ^ super::platform::REG_UTMI_CTRL
        ^ super::platform::REG_OTG_CTRL
        ^ super::platform::REG_OTG_STATUS
        ^ super::platform::REG_EP_CTRL
        ^ super::platform::REG_EP_STATUS
        ^ super::platform::REG_EP_BUFADDR
        ^ super::platform::REG_EP_BUFLEN
        ^ super::platform::REG_DMA_CTRL
        ^ super::platform::REG_DMA_ADDR
        ^ super::platform::REG_DMA_LEN;
    sig
}
