const REG_ID: usize = 0x000;
const REG_CTRL: usize = 0x004;
const REG_STATUS: usize = 0x008;
const REG_CLK_ENABLE: usize = 0x00C;
const REG_POWER_CTRL: usize = 0x010;
const REG_IRQ_STATUS: usize = 0x020;
const REG_IRQ_ENABLE: usize = 0x024;
const REG_IRQ_CLEAR: usize = 0x028;

const CTRL_RESET: u32 = 1 << 0;
const CTRL_ENABLE: u32 = 1 << 1;
const CLK_CORE: u32 = 1 << 0;
const CLK_MEM: u32 = 1 << 1;
const CLK_AXI: u32 = 1 << 2;

const GIC_DIST_BASE: usize = 0x0800_0000;
const GIC_ISENABLER: usize = 0x100;
const GIC_ITARGETSR: usize = 0x800;
const GIC_ICFGR: usize = 0xC00;

pub fn read_device_id(mmio_base: usize) -> u32 {
    crate::hardware_access::mmio_read32(mmio_base + REG_ID).unwrap_or(0)
}

pub fn reset_device(mmio_base: usize) {
    crate::hardware_access::mmio_write32(mmio_base + REG_CTRL, CTRL_RESET);
    let mut timeout = 10000u32;
    while timeout > 0 {
        let status = crate::hardware_access::mmio_read32(mmio_base + REG_STATUS).unwrap_or(0);
        if status & 0x01 != 0 {
            break;
        }
        timeout -= 1;
    }
    crate::hardware_access::mmio_write32(mmio_base + REG_CTRL, CTRL_ENABLE);
}

pub fn enable_clocks(mmio_base: usize) {
    crate::hardware_access::mmio_write32(mmio_base + REG_CLK_ENABLE, CLK_CORE | CLK_MEM | CLK_AXI);
    crate::hardware_access::mmio_write32(mmio_base + REG_POWER_CTRL, 0x01);
}

pub fn enable_interrupts(mmio_base: usize) {
    crate::hardware_access::mmio_write32(mmio_base + REG_IRQ_ENABLE, 0x0F);
}

pub fn clear_interrupts(mmio_base: usize) -> u32 {
    let status = crate::hardware_access::mmio_read32(mmio_base + REG_IRQ_STATUS).unwrap_or(0);
    crate::hardware_access::mmio_write32(mmio_base + REG_IRQ_CLEAR, status);
    status
}

pub fn read_status(mmio_base: usize) -> u32 {
    crate::hardware_access::mmio_read32(mmio_base + REG_STATUS).unwrap_or(0)
}

pub fn configure_gic_spi(spi_id: u32, target_cpu: u32) {
    let reg_index = (spi_id / 32) as usize;
    let bit_index = spi_id % 32;

    crate::hardware_access::mmio_write32(
        GIC_DIST_BASE + GIC_ISENABLER + reg_index * 4,
        1u32 << bit_index,
    );

    let target_index = spi_id as usize;
    crate::hardware_access::mmio_write32(
        GIC_DIST_BASE + GIC_ITARGETSR + target_index * 4,
        1u32 << target_cpu,
    );

    let cfg_index = (spi_id / 16) as usize;
    let cfg_shift = (spi_id % 16) * 2;
    let cfg =
        crate::hardware_access::mmio_read32(GIC_DIST_BASE + GIC_ICFGR + cfg_index * 4).unwrap_or(0);
    let new_cfg = cfg | (0x02 << cfg_shift);
    crate::hardware_access::mmio_write32(GIC_DIST_BASE + GIC_ICFGR + cfg_index * 4, new_cfg);
}

pub fn power_on(mmio_base: usize) {
    crate::hardware_access::mmio_write32(mmio_base + REG_POWER_CTRL, 0x03);
}

pub fn power_off(mmio_base: usize) {
    crate::hardware_access::mmio_write32(mmio_base + REG_POWER_CTRL, 0x00);
}
