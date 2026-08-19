use core::ptr;

pub const SENSOR_ID_REG: usize = 0x00;
pub const SENSOR_STATUS: usize = 0x04;
pub const SENSOR_CTRL: usize = 0x08;
pub const SENSOR_DATA_OUT: usize = 0x0C;
pub const SENSOR_DATA_OUT_H: usize = 0x10;
pub const SENSOR_CONFIG: usize = 0x14;
pub const SENSOR_RATE: usize = 0x18;
pub const SENSOR_FIFO_CTRL: usize = 0x1C;
pub const SENSOR_FIFO_STATUS: usize = 0x20;
pub const SENSOR_INT_CTRL: usize = 0x24;
pub const SENSOR_INT_STATUS: usize = 0x28;
pub const SENSOR_OFFSET_X: usize = 0x2C;
pub const SENSOR_OFFSET_Y: usize = 0x30;
pub const SENSOR_OFFSET_Z: usize = 0x34;
pub const SENSOR_SCALE: usize = 0x38;

pub const CTRL_POWER_ON: u32 = 1 << 0;
pub const CTRL_STANDBY: u32 = 1 << 1;
pub const CTRL_CONTINUOUS: u32 = 1 << 2;
pub const CTRL_SINGLE_SHOT: u32 = 1 << 3;
pub const CTRL_FIFO_EN: u32 = 1 << 4;
pub const CTRL_INT_EN: u32 = 1 << 5;
pub const CTRL_RESET: u32 = 1 << 7;

pub fn read_reg(base: usize, offset: usize) -> u32 {
    unsafe { ptr::read_volatile((base + offset) as *const u32) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    unsafe { ptr::write_volatile((base + offset) as *mut u32, val) }
}

pub fn sensor_id(base: usize) -> u32 {
    read_reg(base, SENSOR_ID_REG)
}

pub fn enable(base: usize) {
    let ctrl = read_reg(base, SENSOR_CTRL);
    write_reg(base, SENSOR_CTRL, ctrl | CTRL_POWER_ON);
}

pub fn disable(base: usize) {
    let ctrl = read_reg(base, SENSOR_CTRL);
    write_reg(base, SENSOR_CTRL, ctrl & !CTRL_POWER_ON);
}

pub fn reset(base: usize) {
    write_reg(base, SENSOR_CTRL, CTRL_RESET);
    loop {
        let ctrl = read_reg(base, SENSOR_CTRL);
        if ctrl & CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn set_continuous(base: usize) {
    let ctrl = read_reg(base, SENSOR_CTRL);
    write_reg(
        base,
        SENSOR_CTRL,
        (ctrl & !CTRL_SINGLE_SHOT) | CTRL_CONTINUOUS,
    );
}

pub fn set_single_shot(base: usize) {
    let ctrl = read_reg(base, SENSOR_CTRL);
    write_reg(
        base,
        SENSOR_CTRL,
        (ctrl & !CTRL_CONTINUOUS) | CTRL_SINGLE_SHOT,
    );
}

pub fn enable_fifo(base: usize) {
    let ctrl = read_reg(base, SENSOR_CTRL);
    write_reg(base, SENSOR_CTRL, ctrl | CTRL_FIFO_EN);
}

pub fn fifo_level(base: usize) -> u32 {
    read_reg(base, SENSOR_FIFO_STATUS) & 0x3F
}

pub fn read_data(base: usize) -> u32 {
    read_reg(base, SENSOR_DATA_OUT)
}

pub fn read_data_high(base: usize) -> u32 {
    read_reg(base, SENSOR_DATA_OUT_H)
}

pub fn set_sample_rate(base: usize, rate_code: u32) {
    write_reg(base, SENSOR_RATE, rate_code);
}

pub fn set_offsets(base: usize, x: u32, y: u32, z: u32) {
    write_reg(base, SENSOR_OFFSET_X, x);
    write_reg(base, SENSOR_OFFSET_Y, y);
    write_reg(base, SENSOR_OFFSET_Z, z);
}

pub fn set_scale(base: usize, scale: u32) {
    write_reg(base, SENSOR_SCALE, scale);
}
