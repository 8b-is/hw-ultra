#[derive(Copy, Clone, PartialEq)]
pub enum StreamDirection {
    Output,
    Input,
}

#[derive(Copy, Clone)]
pub struct StreamDescriptor {
    pub index: u8,
    pub direction: StreamDirection,
    pub base_reg: usize,
    pub buffer_addr: u64,
    pub buffer_len: u32,
    pub format: u32,
    pub running: bool,
}

pub const SD_CTL: usize = 0x00;
pub const SD_STS: usize = 0x03;
pub const SD_LPIB: usize = 0x04;
pub const SD_CBL: usize = 0x08;
pub const SD_LVI: usize = 0x0C;
pub const SD_FMT: usize = 0x12;
pub const SD_BDPL: usize = 0x18;
pub const SD_BDPU: usize = 0x1C;

pub fn stream_offset(index: u8) -> usize {
    0x80 + (index as usize) * 0x20
}

pub fn start(base: usize, sd: &mut StreamDescriptor) {
    let off = stream_offset(sd.index);
    let ctl = super::hw::read_reg(base, off + SD_CTL);
    super::hw::write_reg(base, off + SD_CTL, ctl | 0x02);
    sd.running = true;
}

pub fn stop(base: usize, sd: &mut StreamDescriptor) {
    let off = stream_offset(sd.index);
    let ctl = super::hw::read_reg(base, off + SD_CTL);
    super::hw::write_reg(base, off + SD_CTL, ctl & !0x02);
    sd.running = false;
}

pub fn reset(base: usize, sd: &mut StreamDescriptor) {
    let off = stream_offset(sd.index);
    super::hw::write_reg(base, off + SD_CTL, 0x01);
    let mut timeout = 1000u32;
    while (super::hw::read_reg(base, off + SD_CTL) & 0x01) == 0 && timeout > 0 {
        timeout -= 1;
    }
    super::hw::write_reg(base, off + SD_CTL, 0x00);
    timeout = 1000;
    while (super::hw::read_reg(base, off + SD_CTL) & 0x01) != 0 && timeout > 0 {
        timeout -= 1;
    }
    sd.running = false;
}

pub fn position(base: usize, sd: &StreamDescriptor) -> u32 {
    super::hw::read_reg(base, stream_offset(sd.index) + SD_LPIB)
}

pub fn set_format(base: usize, sd: &mut StreamDescriptor, fmt: u32) {
    let off = stream_offset(sd.index);
    super::hw::write_reg(base, off + SD_FMT, fmt);
    sd.format = fmt;
}

pub fn set_buffer(base: usize, sd: &mut StreamDescriptor, addr: u64, len: u32) {
    let off = stream_offset(sd.index);
    super::hw::write_reg(base, off + SD_BDPL, addr as u32);
    super::hw::write_reg(base, off + SD_BDPU, (addr >> 32) as u32);
    super::hw::write_reg(base, off + SD_CBL, len);
    sd.buffer_addr = addr;
    sd.buffer_len = len;
}
