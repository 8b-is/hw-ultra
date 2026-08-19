pub const REG_ID: usize = 0x000;
pub const REG_CTRL: usize = 0x004;
pub const REG_STATUS: usize = 0x008;
pub const REG_CLK: usize = 0x00C;
pub const REG_POWER: usize = 0x010;
pub const REG_IRQ: usize = 0x014;
pub const REG_GPIO_DIR: usize = 0x100;
pub const REG_GPIO_DATA: usize = 0x104;
pub const REG_GPIO_IRQ_EN: usize = 0x108;
pub const REG_GPIO_IRQ_STATUS: usize = 0x10C;
pub const REG_GPIO_IRQ_EDGE: usize = 0x110;
pub const REG_TOUCH_CTRL: usize = 0x200;
pub const REG_TOUCH_STATUS: usize = 0x204;
pub const REG_TOUCH_X: usize = 0x208;
pub const REG_TOUCH_Y: usize = 0x20C;
pub const REG_TOUCH_PRESSURE: usize = 0x210;
pub const REG_KBD_CTRL: usize = 0x300;
pub const REG_KBD_STATUS: usize = 0x304;
pub const REG_KBD_DATA: usize = 0x308;

pub const CLK_GPIO: u32 = 1 << 0;
pub const CLK_TOUCH: u32 = 1 << 1;
pub const CLK_AHB: u32 = 1 << 2;

const GIC_DIST_BASE: usize = 0x0800_0000;

pub fn read_device_id(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_ID) }
}

pub fn reset_device(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CTRL, 1);
    }
    let mut timeout = 1000u32;
    while timeout > 0 {
        let status = unsafe { super::super::mmio::mmio_read32(mmio_base + REG_STATUS) };
        if status & 1 != 0 {
            break;
        }
        timeout -= 1;
    }
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CTRL, 0);
    }
}

pub fn enable_clocks(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CLK, CLK_GPIO | CLK_TOUCH | CLK_AHB);
    }
}

pub fn configure_gic_spi(spi_id: u32, target_cpu: u32) {
    let irq = spi_id + 32;
    let enable_reg = GIC_DIST_BASE + 0x100 + ((irq / 32) as usize) * 4;
    let target_reg = GIC_DIST_BASE + 0x800 + (irq as usize);
    let cfg_reg = GIC_DIST_BASE + 0xC00 + ((irq / 16) as usize) * 4;
    unsafe {
        super::super::mmio::mmio_write32(enable_reg, 1 << (irq % 32));
        let current = super::super::mmio::mmio_read32(target_reg & !0x3);
        let shift = (irq % 4) * 8;
        let mask = !(0xFF << shift);
        let val = (current & mask) | ((1u32 << target_cpu) << shift);
        super::super::mmio::mmio_write32(target_reg & !0x3, val);
        let cfg_current = super::super::mmio::mmio_read32(cfg_reg);
        let cfg_shift = (irq % 16) * 2;
        let cfg_mask = !(0x3 << cfg_shift);
        super::super::mmio::mmio_write32(cfg_reg, (cfg_current & cfg_mask) | (0x1 << cfg_shift));
    }
}

pub fn enable_interrupts(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_IRQ, 0xF);
    }
}

pub fn clear_interrupts(mmio_base: usize) -> u32 {
    let pending = unsafe { super::super::mmio::mmio_read32(mmio_base + REG_IRQ) };
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_IRQ, pending);
    }
    pending
}

pub fn read_status(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_STATUS) }
}

pub fn set_gpio_direction(mmio_base: usize, mask: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_GPIO_DIR, mask);
    }
}

pub fn read_gpio(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_GPIO_DATA) }
}

pub fn write_gpio(mmio_base: usize, val: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_GPIO_DATA, val);
    }
}

pub fn enable_gpio_irq(mmio_base: usize, mask: u32, edge: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_GPIO_IRQ_EDGE, edge);
        super::super::mmio::mmio_write32(mmio_base + REG_GPIO_IRQ_EN, mask);
    }
}

pub fn read_touch(mmio_base: usize) -> (u32, u32, u32) {
    unsafe {
        let x = super::super::mmio::mmio_read32(mmio_base + REG_TOUCH_X);
        let y = super::super::mmio::mmio_read32(mmio_base + REG_TOUCH_Y);
        let p = super::super::mmio::mmio_read32(mmio_base + REG_TOUCH_PRESSURE);
        (x, y, p)
    }
}

pub fn read_key(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_KBD_DATA) }
}

pub fn power_on(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_POWER, 1);
    }
}

pub fn power_off(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_POWER, 0);
    }
}
