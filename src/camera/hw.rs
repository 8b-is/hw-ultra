use core::ptr;

pub const CSI2_VERSION: usize = 0x00;
pub const CSI2_N_LANES: usize = 0x04;
pub const CSI2_PHY_SHUTDOWNZ: usize = 0x08;
pub const CSI2_DPHY_RSTZ: usize = 0x0C;
pub const CSI2_RESETN: usize = 0x10;
pub const CSI2_PHY_STATE: usize = 0x14;
pub const CSI2_DATA_IDS_1: usize = 0x18;
pub const CSI2_ERR1: usize = 0x20;
pub const CSI2_ERR2: usize = 0x24;
pub const CSI2_MSK1: usize = 0x28;
pub const CSI2_MSK2: usize = 0x2C;

pub const ISP_CTRL: usize = 0x00;
pub const ISP_STATUS: usize = 0x04;
pub const ISP_IN_SIZE: usize = 0x08;
pub const ISP_OUT_SIZE: usize = 0x0C;
pub const ISP_GAIN_R: usize = 0x10;
pub const ISP_GAIN_G: usize = 0x14;
pub const ISP_GAIN_B: usize = 0x18;
pub const ISP_EXPOSURE: usize = 0x1C;
pub const ISP_GAMMA: usize = 0x20;

pub const ISP_CTRL_ENABLE: u32 = 1 << 0;
pub const ISP_CTRL_STREAM: u32 = 1 << 1;
pub const ISP_CTRL_AWB: u32 = 1 << 4;
pub const ISP_CTRL_AE: u32 = 1 << 5;
pub const ISP_CTRL_AF: u32 = 1 << 6;

pub fn read_reg(base: usize, offset: usize) -> u32 {
    unsafe { ptr::read_volatile((base + offset) as *const u32) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    unsafe { ptr::write_volatile((base + offset) as *mut u32, val) }
}

pub fn csi2_version(base: usize) -> u32 {
    read_reg(base, CSI2_VERSION)
}

pub fn csi2_set_lanes(base: usize, lanes: u32) {
    write_reg(base, CSI2_N_LANES, lanes - 1);
}

pub fn csi2_phy_enable(base: usize) {
    write_reg(base, CSI2_PHY_SHUTDOWNZ, 1);
    write_reg(base, CSI2_DPHY_RSTZ, 1);
    write_reg(base, CSI2_RESETN, 1);
}

pub fn csi2_phy_disable(base: usize) {
    write_reg(base, CSI2_RESETN, 0);
    write_reg(base, CSI2_DPHY_RSTZ, 0);
    write_reg(base, CSI2_PHY_SHUTDOWNZ, 0);
}

pub fn csi2_phy_state(base: usize) -> u32 {
    read_reg(base, CSI2_PHY_STATE)
}

pub fn isp_enable(base: usize) {
    let ctrl = read_reg(base, ISP_CTRL);
    write_reg(base, ISP_CTRL, ctrl | ISP_CTRL_ENABLE);
}

pub fn isp_disable(base: usize) {
    let ctrl = read_reg(base, ISP_CTRL);
    write_reg(base, ISP_CTRL, ctrl & !ISP_CTRL_ENABLE);
}

pub fn isp_start_stream(base: usize) {
    let ctrl = read_reg(base, ISP_CTRL);
    write_reg(base, ISP_CTRL, ctrl | ISP_CTRL_STREAM);
}

pub fn isp_stop_stream(base: usize) {
    let ctrl = read_reg(base, ISP_CTRL);
    write_reg(base, ISP_CTRL, ctrl & !ISP_CTRL_STREAM);
}

pub fn isp_set_white_balance(base: usize, r: u32, g: u32, b: u32) {
    write_reg(base, ISP_GAIN_R, r);
    write_reg(base, ISP_GAIN_G, g);
    write_reg(base, ISP_GAIN_B, b);
}

pub fn isp_set_exposure(base: usize, exposure: u32) {
    write_reg(base, ISP_EXPOSURE, exposure);
}

pub fn isp_set_gamma(base: usize, gamma: u32) {
    write_reg(base, ISP_GAMMA, gamma);
}
