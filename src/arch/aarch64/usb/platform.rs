pub const REG_ID: usize = 0x000;
pub const REG_CTRL: usize = 0x004;
pub const REG_STATUS: usize = 0x008;
pub const REG_CLK: usize = 0x00C;
pub const REG_POWER: usize = 0x010;
pub const REG_IRQ: usize = 0x014;
pub const REG_PHY_CTRL: usize = 0x100;
pub const REG_PHY_STATUS: usize = 0x104;
pub const REG_UTMI_CTRL: usize = 0x108;
pub const REG_OTG_CTRL: usize = 0x200;
pub const REG_OTG_STATUS: usize = 0x204;
pub const REG_EP_CTRL: usize = 0x300;
pub const REG_EP_STATUS: usize = 0x304;
pub const REG_EP_BUFADDR: usize = 0x308;
pub const REG_EP_BUFLEN: usize = 0x30C;
pub const REG_DMA_CTRL: usize = 0x400;
pub const REG_DMA_ADDR: usize = 0x404;
pub const REG_DMA_LEN: usize = 0x408;

pub const CLK_PHY: u32 = 1 << 0;
pub const CLK_AHB: u32 = 1 << 1;
pub const CLK_SUSPEND: u32 = 1 << 2;

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
        super::super::mmio::mmio_write32(mmio_base + REG_CLK, CLK_PHY | CLK_AHB | CLK_SUSPEND);
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
        super::super::mmio::mmio_write32(mmio_base + REG_IRQ, 0x1F);
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

pub fn init_phy(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_PHY_CTRL, 1);
        super::super::mmio::mmio_write32(mmio_base + REG_UTMI_CTRL, 0x1);
    }
}

pub fn set_otg_mode(mmio_base: usize, host: bool) {
    let val = if host { 1u32 } else { 0u32 };
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_OTG_CTRL, val);
    }
}

pub fn configure_endpoint(mmio_base: usize, ep: u32, buf_addr: u64, buf_len: u32) {
    let ep_base = REG_EP_CTRL + (ep as usize) * 0x10;
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + ep_base + 0x08, buf_addr as u32);
        super::super::mmio::mmio_write32(mmio_base + ep_base + 0x0C, buf_len);
        super::super::mmio::mmio_write32(mmio_base + ep_base, 1);
    }
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
