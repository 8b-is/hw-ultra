pub const REG_ISP_CTRL: u32 = 0x00;
pub const REG_ISP_STATUS: u32 = 0x04;
pub const REG_ISP_VERSION: u32 = 0x08;
pub const REG_ISP_FORMAT: u32 = 0x0C;
pub const REG_ISP_WIDTH: u32 = 0x10;
pub const REG_ISP_HEIGHT: u32 = 0x14;
pub const REG_ISP_STRIDE: u32 = 0x18;
pub const REG_CSI_CTRL: u32 = 0x20;
pub const REG_CSI_LANES: u32 = 0x24;
pub const REG_CSI_DATA_RATE: u32 = 0x28;
pub const REG_CSI_STATUS: u32 = 0x2C;
pub const REG_DMA_CTRL: u32 = 0x40;
pub const REG_DMA_ADDR_LO: u32 = 0x44;
pub const REG_DMA_ADDR_HI: u32 = 0x48;
pub const REG_DMA_LENGTH: u32 = 0x4C;
pub const REG_DMA_STATUS: u32 = 0x50;
pub const REG_IRQ_STATUS: u32 = 0x60;
pub const REG_IRQ_MASK: u32 = 0x64;

pub const ISP_CTRL_ENABLE: u32 = 1 << 0;
pub const ISP_CTRL_RESET: u32 = 1 << 1;
pub const ISP_CTRL_CAPTURE: u32 = 1 << 2;
pub const ISP_CTRL_CONTINUOUS: u32 = 1 << 3;

pub const CSI_CTRL_ENABLE: u32 = 1 << 0;
pub const CSI_CTRL_RESET: u32 = 1 << 1;

fn read_reg(base: usize, offset: u32) -> u32 {
    unsafe { super::super::mmio::mmio_read32(base + offset as usize) }
}

fn write_reg(base: usize, offset: u32, val: u32) {
    unsafe { super::super::mmio::mmio_write32(base + offset as usize, val) }
}

pub fn reset(base: usize) {
    write_reg(base, REG_ISP_CTRL, ISP_CTRL_RESET);
    for _ in 0..1000 {
        if read_reg(base, REG_ISP_CTRL) & ISP_CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn enable(base: usize) {
    let val = read_reg(base, REG_ISP_CTRL);
    write_reg(base, REG_ISP_CTRL, val | ISP_CTRL_ENABLE);
}

pub fn set_resolution(base: usize, width: u32, height: u32) {
    write_reg(base, REG_ISP_WIDTH, width);
    write_reg(base, REG_ISP_HEIGHT, height);
}

pub fn set_format(base: usize, format: u32) {
    write_reg(base, REG_ISP_FORMAT, format);
}

pub fn set_stride(base: usize, stride: u32) {
    write_reg(base, REG_ISP_STRIDE, stride);
}

pub fn enable_csi(base: usize) {
    write_reg(base, REG_CSI_CTRL, CSI_CTRL_ENABLE);
}

pub fn reset_csi(base: usize) {
    write_reg(base, REG_CSI_CTRL, CSI_CTRL_RESET);
    for _ in 0..1000 {
        if read_reg(base, REG_CSI_CTRL) & CSI_CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn set_csi_lanes(base: usize, lanes: u32) {
    write_reg(base, REG_CSI_LANES, lanes);
}

pub fn set_csi_data_rate(base: usize, rate: u32) {
    write_reg(base, REG_CSI_DATA_RATE, rate);
}

pub fn start_capture(base: usize) {
    let val = read_reg(base, REG_ISP_CTRL);
    write_reg(base, REG_ISP_CTRL, val | ISP_CTRL_CAPTURE);
}

pub fn start_continuous(base: usize) {
    let val = read_reg(base, REG_ISP_CTRL);
    write_reg(base, REG_ISP_CTRL, val | ISP_CTRL_CONTINUOUS);
}

pub fn setup_dma(base: usize, addr: u64, length: u32) {
    write_reg(base, REG_DMA_ADDR_LO, addr as u32);
    write_reg(base, REG_DMA_ADDR_HI, (addr >> 32) as u32);
    write_reg(base, REG_DMA_LENGTH, length);
    write_reg(base, REG_DMA_CTRL, 0x01);
}

pub fn read_irq_status(base: usize) -> u32 {
    read_reg(base, REG_IRQ_STATUS)
}

pub fn clear_irq(base: usize, bits: u32) {
    write_reg(base, REG_IRQ_STATUS, bits);
}

pub fn read_version(base: usize) -> u32 {
    read_reg(base, REG_ISP_VERSION)
}
