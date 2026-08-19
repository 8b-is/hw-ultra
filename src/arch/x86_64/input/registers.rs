pub const REG_CTRL: u32 = 0x00;
pub const REG_STATUS: u32 = 0x04;
pub const REG_VERSION: u32 = 0x08;
pub const REG_KBD_DATA: u32 = 0x10;
pub const REG_KBD_STATUS: u32 = 0x14;
pub const REG_KBD_CTRL: u32 = 0x18;
pub const REG_KBD_SCANCODE: u32 = 0x1C;
pub const REG_MOUSE_DATA: u32 = 0x20;
pub const REG_MOUSE_STATUS: u32 = 0x24;
pub const REG_MOUSE_CTRL: u32 = 0x28;
pub const REG_MOUSE_X: u32 = 0x30;
pub const REG_MOUSE_Y: u32 = 0x34;
pub const REG_MOUSE_BUTTONS: u32 = 0x38;
pub const REG_TOUCH_CTRL: u32 = 0x40;
pub const REG_TOUCH_X: u32 = 0x44;
pub const REG_TOUCH_Y: u32 = 0x48;
pub const REG_TOUCH_PRESSURE: u32 = 0x4C;
pub const REG_TOUCH_STATUS: u32 = 0x50;
pub const REG_IRQ_STATUS: u32 = 0x60;
pub const REG_IRQ_MASK: u32 = 0x64;

pub const CTRL_ENABLE: u32 = 1 << 0;
pub const CTRL_RESET: u32 = 1 << 1;
pub const CTRL_KBD_EN: u32 = 1 << 2;
pub const CTRL_MOUSE_EN: u32 = 1 << 3;
pub const CTRL_TOUCH_EN: u32 = 1 << 4;

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

pub fn enable_keyboard(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_KBD_EN);
}

pub fn enable_mouse(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_MOUSE_EN);
}

pub fn enable_touch(base: usize) {
    let val = read_reg(base, REG_CTRL);
    write_reg(base, REG_CTRL, val | CTRL_TOUCH_EN);
}

pub fn read_scancode(base: usize) -> u32 {
    read_reg(base, REG_KBD_SCANCODE)
}

pub fn keyboard_ready(base: usize) -> bool {
    read_reg(base, REG_KBD_STATUS) & 0x01 != 0
}

pub fn read_mouse_position(base: usize) -> (u32, u32) {
    let x = read_reg(base, REG_MOUSE_X);
    let y = read_reg(base, REG_MOUSE_Y);
    (x, y)
}

pub fn read_mouse_buttons(base: usize) -> u32 {
    read_reg(base, REG_MOUSE_BUTTONS)
}

pub fn mouse_ready(base: usize) -> bool {
    read_reg(base, REG_MOUSE_STATUS) & 0x01 != 0
}

pub fn read_touch(base: usize) -> (u32, u32, u32) {
    let x = read_reg(base, REG_TOUCH_X);
    let y = read_reg(base, REG_TOUCH_Y);
    let p = read_reg(base, REG_TOUCH_PRESSURE);
    (x, y, p)
}

pub fn touch_active(base: usize) -> bool {
    read_reg(base, REG_TOUCH_STATUS) & 0x01 != 0
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
