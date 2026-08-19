pub const REG_ID: usize = 0x000;
pub const REG_CTRL: usize = 0x004;
pub const REG_STATUS: usize = 0x008;
pub const REG_CLK: usize = 0x00C;
pub const REG_POWER: usize = 0x010;
pub const REG_IRQ: usize = 0x014;
pub const REG_CMD: usize = 0x100;
pub const REG_CMD_ARG: usize = 0x104;
pub const REG_RESP0: usize = 0x108;
pub const REG_RESP1: usize = 0x10C;
pub const REG_RESP2: usize = 0x110;
pub const REG_RESP3: usize = 0x114;
pub const REG_DATA_PORT: usize = 0x200;
pub const REG_BLOCK_SIZE: usize = 0x204;
pub const REG_BLOCK_COUNT: usize = 0x208;
pub const REG_DMA_ADDR: usize = 0x300;
pub const REG_DMA_LEN: usize = 0x304;
pub const REG_DMA_CTRL: usize = 0x308;

pub const CLK_AHB: u32 = 1 << 0;
pub const CLK_CORE: u32 = 1 << 1;
pub const CLK_UFS_PHY: u32 = 1 << 2;

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
        super::super::mmio::mmio_write32(mmio_base + REG_CLK, CLK_AHB | CLK_CORE | CLK_UFS_PHY);
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

pub fn send_command(mmio_base: usize, cmd: u32, arg: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CMD_ARG, arg);
        super::super::mmio::mmio_write32(mmio_base + REG_CMD, cmd);
    }
}

pub fn read_response(mmio_base: usize) -> [u32; 4] {
    unsafe {
        [
            super::super::mmio::mmio_read32(mmio_base + REG_RESP0),
            super::super::mmio::mmio_read32(mmio_base + REG_RESP1),
            super::super::mmio::mmio_read32(mmio_base + REG_RESP2),
            super::super::mmio::mmio_read32(mmio_base + REG_RESP3),
        ]
    }
}

pub fn set_block_size(mmio_base: usize, size: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_BLOCK_SIZE, size);
    }
}

pub fn set_block_count(mmio_base: usize, count: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_BLOCK_COUNT, count);
    }
}

pub fn configure_dma(mmio_base: usize, addr: u64, len: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_DMA_ADDR, addr as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_DMA_LEN, len);
        super::super::mmio::mmio_write32(mmio_base + REG_DMA_CTRL, 1);
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
