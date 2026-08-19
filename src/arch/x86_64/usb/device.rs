use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static USB_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static USB_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static USB_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86UsbContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
}

pub fn init_usb(bus: u8, dev: u8, func: u8) -> Option<X86UsbContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    USB_MMIO_BASE.store(bar_base, Ordering::Release);
    USB_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x60u8.wrapping_add(bus);
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);

    USB_INITIALIZED.store(true, Ordering::Release);

    Some(X86UsbContext {
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

pub fn usb_mmio_base() -> usize {
    USB_MMIO_BASE.load(Ordering::Acquire)
}

pub fn usb_mmio_size() -> usize {
    USB_MMIO_SIZE.load(Ordering::Acquire)
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

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = usb_mmio_base();
    let mut sig = base ^ usb_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_usb_reg(0) as usize;
    write_usb_reg(0, read_usb_reg(4));
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
    sig ^= super::registers::read_cap_length(base) as usize;
    sig ^= super::registers::operational_base(base);
    let (slots, intrs, ports) = super::registers::read_hcsparams1(base);
    sig ^= slots as usize ^ intrs as usize ^ ports as usize;
    sig ^= super::registers::read_usbsts(base) as usize;
    sig ^= super::registers::controller_ready(base) as usize;
    super::registers::enable(base);
    super::registers::set_max_slots(base, 0);
    super::registers::set_dcbaap(base, 0);
    super::registers::set_command_ring(base, 0);
    sig ^= super::registers::read_doorbell_offset(base) as usize;
    super::registers::ring_doorbell(base, 0, 0);
    sig ^= super::registers::read_page_size(base) as usize;
    sig ^= super::registers::REG_CAPLENGTH as usize
        ^ super::registers::REG_HCSPARAMS1 as usize
        ^ super::registers::REG_HCSPARAMS2 as usize
        ^ super::registers::REG_HCSPARAMS3 as usize
        ^ super::registers::REG_HCCPARAMS1 as usize
        ^ super::registers::REG_DBOFF as usize
        ^ super::registers::REG_RTSOFF as usize
        ^ super::registers::REG_HCCPARAMS2 as usize;
    sig ^= super::registers::REG_USBCMD as usize
        ^ super::registers::REG_USBSTS as usize
        ^ super::registers::REG_PAGESIZE as usize
        ^ super::registers::REG_DNCTRL as usize
        ^ super::registers::REG_CRCR as usize
        ^ super::registers::REG_DCBAAP as usize
        ^ super::registers::REG_CONFIG as usize;
    sig ^= super::registers::USBCMD_RUN as usize
        ^ super::registers::USBCMD_HCRST as usize
        ^ super::registers::USBCMD_INTE as usize
        ^ super::registers::USBCMD_HSEE as usize;
    sig ^= super::registers::USBSTS_HCH as usize
        ^ super::registers::USBSTS_HSE as usize
        ^ super::registers::USBSTS_EINT as usize
        ^ super::registers::USBSTS_PCD as usize
        ^ super::registers::USBSTS_CNR as usize;
    sig
}
