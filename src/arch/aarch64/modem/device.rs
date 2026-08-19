use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static MODEM_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static MODEM_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static MODEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct ArmModemContext {
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub device_id: u32,
    pub spi_id: u32,
    pub smmu_stream_id: u32,
    pub shared_mem: usize,
}

pub fn init_modem(mmio_base: usize, mmio_size: usize, spi_id: u32) -> Option<ArmModemContext> {
    let device_id = super::platform::read_device_id(mmio_base);
    if device_id == 0 || device_id == 0xFFFF_FFFF {
        return None;
    }

    MODEM_MMIO_BASE.store(mmio_base, Ordering::Release);
    MODEM_MMIO_SIZE.store(mmio_size, Ordering::Release);

    super::platform::reset_device(mmio_base);
    super::platform::enable_clocks(mmio_base);

    let stream_id = super::smmu::configure_stream(mmio_base, 0xA00);
    super::smmu::set_attributes(
        mmio_base,
        stream_id,
        super::smmu::ATTR_CACHEABLE | super::smmu::ATTR_SHAREABLE,
    );

    super::platform::configure_gic_spi(spi_id, 0);

    let shared_mem = super::smmu::map_dma_for_modem(mmio_base, mmio_size);

    MODEM_INITIALIZED.store(true, Ordering::Release);

    Some(ArmModemContext {
        mmio_base,
        mmio_size,
        device_id,
        spi_id,
        smmu_stream_id: stream_id,
        shared_mem,
    })
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

pub fn diagnostics(mmio_base: usize) -> usize {
    let mut sig = is_initialized() as usize;
    sig ^= read_modem_reg(0) as usize;
    write_modem_reg(0, read_modem_reg(4));
    super::platform::enable_interrupts(mmio_base);
    sig ^= super::platform::clear_interrupts(mmio_base) as usize;
    sig ^= super::platform::read_status(mmio_base) as usize;
    super::platform::power_on(mmio_base);
    super::platform::power_off(mmio_base);
    sig ^= super::smmu::ATTR_READ as usize ^ super::smmu::ATTR_WRITE as usize;
    sig ^= super::smmu::map_dma_for_modem(0, 0);
    sig ^= super::smmu::get_stream_attrs(0) as usize;
    sig ^= super::smmu::stream_count();
    super::platform::set_uart_baud(mmio_base, 0);
    super::platform::uart_send(mmio_base, 0);
    sig ^= super::platform::uart_recv(mmio_base) as usize;
    super::platform::mailbox_send(mmio_base, 0);
    sig ^= super::platform::mailbox_recv(mmio_base) as usize;
    super::platform::set_rf_mode(mmio_base, 0);
    sig ^= super::platform::REG_UART_CTRL
        ^ super::platform::REG_UART_STATUS
        ^ super::platform::REG_UART_BAUD
        ^ super::platform::REG_UART_TX
        ^ super::platform::REG_UART_RX
        ^ super::platform::REG_SHMEM_BASE
        ^ super::platform::REG_SHMEM_SIZE
        ^ super::platform::REG_SHMEM_CTRL
        ^ super::platform::REG_MAILBOX_TX
        ^ super::platform::REG_MAILBOX_RX
        ^ super::platform::REG_MAILBOX_STATUS
        ^ super::platform::REG_RF_CTRL
        ^ super::platform::REG_RF_STATUS;
    sig
}
