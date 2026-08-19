#[derive(Copy, Clone)]
pub struct TimingParams {
    pub pixel_clock_khz: u32,
    pub h_active: u16,
    pub h_front_porch: u16,
    pub h_sync_width: u16,
    pub h_back_porch: u16,
    pub v_active: u16,
    pub v_front_porch: u16,
    pub v_sync_width: u16,
    pub v_back_porch: u16,
    pub refresh_hz: u8,
}

pub fn h_total(t: &TimingParams) -> u32 {
    t.h_active as u32 + t.h_front_porch as u32 + t.h_sync_width as u32 + t.h_back_porch as u32
}

pub fn v_total(t: &TimingParams) -> u32 {
    t.v_active as u32 + t.v_front_porch as u32 + t.v_sync_width as u32 + t.v_back_porch as u32
}

pub fn compute_pixel_clock(t: &TimingParams) -> u32 {
    h_total(t) * v_total(t) * t.refresh_hz as u32 / 1000
}

pub const TIMING_640X480_60: TimingParams = TimingParams {
    pixel_clock_khz: 25175,
    h_active: 640,
    h_front_porch: 16,
    h_sync_width: 96,
    h_back_porch: 48,
    v_active: 480,
    v_front_porch: 10,
    v_sync_width: 2,
    v_back_porch: 33,
    refresh_hz: 60,
};

pub const TIMING_1280X720_60: TimingParams = TimingParams {
    pixel_clock_khz: 74250,
    h_active: 1280,
    h_front_porch: 110,
    h_sync_width: 40,
    h_back_porch: 220,
    v_active: 720,
    v_front_porch: 5,
    v_sync_width: 5,
    v_back_porch: 20,
    refresh_hz: 60,
};

pub const TIMING_1920X1080_60: TimingParams = TimingParams {
    pixel_clock_khz: 148500,
    h_active: 1920,
    h_front_porch: 88,
    h_sync_width: 44,
    h_back_porch: 148,
    v_active: 1080,
    v_front_porch: 4,
    v_sync_width: 5,
    v_back_porch: 36,
    refresh_hz: 60,
};

pub fn apply_timing(base: usize, t: &TimingParams) {
    super::hw::write_reg(base, 0x10, h_total(t));
    super::hw::write_reg(base, 0x14, v_total(t));
    super::hw::write_reg(base, 0x18, t.h_active as u32);
    super::hw::write_reg(base, 0x1C, t.v_active as u32);
    super::hw::write_reg(base, 0x20, t.h_sync_width as u32);
    super::hw::write_reg(base, 0x24, t.v_sync_width as u32);
    super::hw::write_reg(base, 0x28, t.pixel_clock_khz);
}
