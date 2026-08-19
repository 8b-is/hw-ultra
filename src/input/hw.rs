use core::ptr;

pub const INPUT_STATUS: usize = 0x00;
pub const INPUT_CTRL: usize = 0x04;
pub const INPUT_DATA: usize = 0x08;
pub const INPUT_SCANCODE: usize = 0x0C;
pub const INPUT_INT_CTRL: usize = 0x10;
pub const INPUT_INT_STATUS: usize = 0x14;
pub const INPUT_FIFO_STATUS: usize = 0x18;

pub const PS2_DATA: u16 = 0x60;
pub const PS2_STATUS: u16 = 0x64;
pub const PS2_COMMAND: u16 = 0x64;

pub const PS2_STS_OUTPUT_FULL: u8 = 1 << 0;
pub const PS2_STS_INPUT_FULL: u8 = 1 << 1;

pub const CTRL_ENABLE: u32 = 1 << 0;
pub const CTRL_INT_ENABLE: u32 = 1 << 1;
pub const CTRL_FIFO_ENABLE: u32 = 1 << 2;
pub const CTRL_RESET: u32 = 1 << 7;

pub fn read_reg(base: usize, offset: usize) -> u32 {
    unsafe { ptr::read_volatile((base + offset) as *const u32) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    unsafe { ptr::write_volatile((base + offset) as *mut u32, val) }
}

pub fn read_port8(port: u16) -> u8 {
    unsafe { crate::arch::x86_64::io::inb(port) }
}

pub fn write_port8(port: u16, val: u8) {
    unsafe { crate::arch::x86_64::io::outb(port, val) }
}

pub fn ps2_data_available() -> bool {
    read_port8(PS2_STATUS) & PS2_STS_OUTPUT_FULL != 0
}

pub fn ps2_read_data() -> u8 {
    read_port8(PS2_DATA)
}

pub fn ps2_send_command(cmd: u8) {
    loop {
        if read_port8(PS2_STATUS) & PS2_STS_INPUT_FULL == 0 {
            break;
        }
    }
    write_port8(PS2_COMMAND, cmd);
}

pub fn ps2_send_data(data: u8) {
    loop {
        if read_port8(PS2_STATUS) & PS2_STS_INPUT_FULL == 0 {
            break;
        }
    }
    write_port8(PS2_DATA, data);
}

pub fn mmio_enable(base: usize) {
    let ctrl = read_reg(base, INPUT_CTRL);
    write_reg(base, INPUT_CTRL, ctrl | CTRL_ENABLE | CTRL_INT_ENABLE);
}

pub fn mmio_disable(base: usize) {
    let ctrl = read_reg(base, INPUT_CTRL);
    write_reg(base, INPUT_CTRL, ctrl & !CTRL_ENABLE);
}

pub fn mmio_reset(base: usize) {
    write_reg(base, INPUT_CTRL, CTRL_RESET);
    loop {
        let ctrl = read_reg(base, INPUT_CTRL);
        if ctrl & CTRL_RESET == 0 {
            break;
        }
    }
}

pub fn mmio_read_scancode(base: usize) -> u32 {
    read_reg(base, INPUT_SCANCODE)
}

pub fn mmio_fifo_level(base: usize) -> u32 {
    read_reg(base, INPUT_FIFO_STATUS) & 0x3F
}
