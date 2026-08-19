pub const REG_CTRL: u32 = 0x00;
pub const REG_STATUS: u32 = 0x04;
pub const REG_VERSION: u32 = 0x08;
pub const REG_SENSOR_TYPE: u32 = 0x0C;
pub const REG_SAMPLE_RATE: u32 = 0x10;
pub const REG_DATA_X: u32 = 0x20;
pub const REG_DATA_Y: u32 = 0x24;
pub const REG_DATA_Z: u32 = 0x28;
pub const REG_DATA_RAW: u32 = 0x2C;
pub const REG_THRESHOLD_LO: u32 = 0x30;
pub const REG_THRESHOLD_HI: u32 = 0x34;
pub const REG_CALIBRATION: u32 = 0x38;
pub const REG_FIFO_CTRL: u32 = 0x40;
pub const REG_FIFO_STATUS: u32 = 0x44;
pub const REG_FIFO_DATA: u32 = 0x48;
pub const REG_IRQ_STATUS: u32 = 0x60;
pub const REG_IRQ_MASK: u32 = 0x64;

pub const CTRL_ENABLE: u32 = 1 << 0;
pub const CTRL_RESET: u32 = 1 << 1;
pub const CTRL_FIFO_EN: u32 = 1 << 2;
pub const CTRL_IRQ_EN: u32 = 1 << 3;

fn read_reg(base: usize, offset: u32) -> u32 {
    unsafe { super::super::mmio::mmio_read32(base + offset as usize) }
}

fn write_reg(base: usize, offset: u32, val: u32) {
    unsafe { super::super::mmio::mmio_write32(base + offset as usize, val) }
}

pub fn reset(base: usize) {
    write_reg(base, REG_CTRL, CTRL_RESET);
    for _ in 0..1000 {
        if read_reg(base, REG_CTRL) & CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn enable(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_ENABLE);
}

pub fn set_sample_rate(base: usize, rate: u32) {
    write_reg(base, REG_SAMPLE_RATE, rate);
}

pub fn read_data_xyz(base: usize) -> (u32, u32, u32) {
    let x = read_reg(base, REG_DATA_X);
    let y = read_reg(base, REG_DATA_Y);
    let z = read_reg(base, REG_DATA_Z);
    (x, y, z)
}

pub fn read_raw(base: usize) -> u32 {
    read_reg(base, REG_DATA_RAW)
}

pub fn set_thresholds(base: usize, lo: u32, hi: u32) {
    write_reg(base, REG_THRESHOLD_LO, lo);
    write_reg(base, REG_THRESHOLD_HI, hi);
}

pub fn set_calibration(base: usize, cal: u32) {
    write_reg(base, REG_CALIBRATION, cal);
}

pub fn enable_fifo(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_FIFO_EN);
}

pub fn read_fifo(base: usize) -> u32 {
    read_reg(base, REG_FIFO_DATA)
}

pub fn fifo_level(base: usize) -> u32 {
    read_reg(base, REG_FIFO_STATUS) & 0xFFFF
}

pub fn read_irq_status(base: usize) -> u32 {
    read_reg(base, REG_IRQ_STATUS)
}

pub fn clear_irq(base: usize, bits: u32) {
    write_reg(base, REG_IRQ_STATUS, bits);
}

pub fn read_version(base: usize) -> u32 {
    read_reg(base, REG_VERSION)
}

pub fn read_sensor_type(base: usize) -> u32 {
    read_reg(base, REG_SENSOR_TYPE)
}
