pub fn read_reg(base: usize, offset: usize) -> u32 {
    let ptr = (base + offset) as *const u32;
    unsafe { core::ptr::read_volatile(ptr) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    let ptr = (base + offset) as *mut u32;
    unsafe { core::ptr::write_volatile(ptr, val) }
}

pub const GCAP: usize = 0x00;
pub const GCTL: usize = 0x08;
pub const GSTS: usize = 0x10;
pub const INTCTL: usize = 0x20;
pub const INTSTS: usize = 0x24;
pub const CORBLBASE: usize = 0x40;
pub const CORBUBASE: usize = 0x44;
pub const CORBWP: usize = 0x48;
pub const CORBRP: usize = 0x4A;
pub const RIRBLBASE: usize = 0x50;
pub const RIRBUBASE: usize = 0x54;
pub const RIRBWP: usize = 0x58;

pub fn global_capabilities(base: usize) -> u32 {
    read_reg(base, GCAP)
}

pub fn output_stream_count(gcap: u32) -> u8 {
    ((gcap >> 12) & 0xF) as u8
}

pub fn input_stream_count(gcap: u32) -> u8 {
    ((gcap >> 8) & 0xF) as u8
}

pub fn reset_controller(base: usize) {
    write_reg(base, GCTL, 0);
    let mut timeout = 1000u32;
    while (read_reg(base, GCTL) & 1) != 0 && timeout > 0 {
        timeout -= 1;
    }
    write_reg(base, GCTL, 1);
    timeout = 1000;
    while (read_reg(base, GCTL) & 1) == 0 && timeout > 0 {
        timeout -= 1;
    }
}

pub fn enable_interrupts(base: usize) {
    write_reg(base, INTCTL, 0x8000_FFFF);
}

pub fn disable_interrupts(base: usize) {
    write_reg(base, INTCTL, 0);
}

pub fn clear_status(base: usize) {
    write_reg(base, INTSTS, read_reg(base, INTSTS));
    write_reg(base, GSTS, read_reg(base, GSTS));
}
