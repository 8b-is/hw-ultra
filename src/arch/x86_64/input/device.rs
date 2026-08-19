use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static INPUT_MMIO_BASE: AtomicUsize = AtomicUsize::new(0);
static INPUT_MMIO_SIZE: AtomicUsize = AtomicUsize::new(0);
static INPUT_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct X86InputContext {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_base: usize,
    pub bar0_size: usize,
    pub msi_vector: u8,
}

pub fn init_input(bus: u8, dev: u8, func: u8) -> Option<X86InputContext> {
    let (vendor, device_id) = super::pci::read_ids(bus, dev, func)?;

    super::pci::enable_bus_master(bus, dev, func);
    super::pci::enable_memory_space(bus, dev, func);

    let (bar_base, bar_size) = super::pci::decode_bar0(bus, dev, func)?;
    INPUT_MMIO_BASE.store(bar_base, Ordering::Release);
    INPUT_MMIO_SIZE.store(bar_size, Ordering::Release);

    let vector = 0x90u8;
    let msi_cap = super::pci::find_capability(bus, dev, func, super::pci::CAP_MSI);
    if msi_cap != 0 {
        super::msi::program_msi(bus, dev, func, msi_cap, vector, 0);
        super::msi::enable_msi(bus, dev, func, msi_cap);
    }

    super::registers::reset(bar_base);
    super::registers::enable(bar_base);

    INPUT_INITIALIZED.store(true, Ordering::Release);

    Some(X86InputContext {
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

pub fn input_mmio_base() -> usize {
    INPUT_MMIO_BASE.load(Ordering::Acquire)
}

pub fn input_mmio_size() -> usize {
    INPUT_MMIO_SIZE.load(Ordering::Acquire)
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

pub fn diagnostics(bus: u8, dev: u8, func: u8) -> usize {
    let base = input_mmio_base();
    let mut sig = base ^ input_mmio_size() ^ (is_initialized() as usize);
    sig ^= read_input_reg(0) as usize;
    write_input_reg(0, read_input_reg(4));

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
    sig ^= super::registers::read_irq_status(base) as usize;
    super::registers::clear_irq(base, 0xFFFF_FFFF);
    super::registers::enable_keyboard(base);
    super::registers::enable_mouse(base);
    super::registers::enable_touch(base);
    sig ^= super::registers::read_scancode(base) as usize;
    sig ^= super::registers::keyboard_ready(base) as usize;
    let (mx, my) = super::registers::read_mouse_position(base);
    sig ^= mx as usize ^ my as usize;
    sig ^= super::registers::read_mouse_buttons(base) as usize;
    sig ^= super::registers::mouse_ready(base) as usize;
    let (tx, ty, tp) = super::registers::read_touch(base);
    sig ^= tx as usize ^ ty as usize ^ tp as usize;
    sig ^= super::registers::touch_active(base) as usize;

    sig ^= super::registers::REG_CTRL as usize
        ^ super::registers::REG_STATUS as usize
        ^ super::registers::REG_VERSION as usize
        ^ super::registers::REG_KBD_DATA as usize
        ^ super::registers::REG_KBD_STATUS as usize
        ^ super::registers::REG_KBD_CTRL as usize
        ^ super::registers::REG_KBD_SCANCODE as usize;
    sig ^= super::registers::REG_MOUSE_DATA as usize
        ^ super::registers::REG_MOUSE_STATUS as usize
        ^ super::registers::REG_MOUSE_CTRL as usize
        ^ super::registers::REG_MOUSE_X as usize
        ^ super::registers::REG_MOUSE_Y as usize
        ^ super::registers::REG_MOUSE_BUTTONS as usize;
    sig ^= super::registers::REG_TOUCH_CTRL as usize
        ^ super::registers::REG_TOUCH_X as usize
        ^ super::registers::REG_TOUCH_Y as usize
        ^ super::registers::REG_TOUCH_PRESSURE as usize
        ^ super::registers::REG_TOUCH_STATUS as usize;
    sig ^= super::registers::REG_IRQ_STATUS as usize ^ super::registers::REG_IRQ_MASK as usize;
    sig ^= super::registers::CTRL_ENABLE as usize
        ^ super::registers::CTRL_RESET as usize
        ^ super::registers::CTRL_KBD_EN as usize
        ^ super::registers::CTRL_MOUSE_EN as usize
        ^ super::registers::CTRL_TOUCH_EN as usize;

    sig
}
