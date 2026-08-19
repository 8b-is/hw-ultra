#[derive(Clone, Copy)]
pub struct CalibrationData {
    pub offset_x: i32,
    pub offset_y: i32,
    pub offset_z: i32,
    pub scale_x: i32,
    pub scale_y: i32,
    pub scale_z: i32,
    pub cross_xy: i32,
    pub cross_xz: i32,
    pub cross_yz: i32,
}

impl CalibrationData {
    pub const fn identity() -> Self {
        CalibrationData {
            offset_x: 0,
            offset_y: 0,
            offset_z: 0,
            scale_x: 1000,
            scale_y: 1000,
            scale_z: 1000,
            cross_xy: 0,
            cross_xz: 0,
            cross_yz: 0,
        }
    }
}

pub fn apply_offset(raw: i32, offset: i32) -> i32 {
    raw.wrapping_sub(offset)
}

pub fn apply_scale(corrected: i32, scale: i32) -> i32 {
    ((corrected as i64 * scale as i64) / 1000) as i32
}

pub fn calibrate_sample(
    raw: &super::data::RawSample,
    cal: &CalibrationData,
) -> super::data::CalibratedSample {
    let cx = apply_scale(apply_offset(raw.x, cal.offset_x), cal.scale_x);
    let cy = apply_scale(apply_offset(raw.y, cal.offset_y), cal.scale_y);
    let cz = apply_scale(apply_offset(raw.z, cal.offset_z), cal.scale_z);
    super::data::CalibratedSample {
        x: cx,
        y: cy,
        z: cz,
        timestamp: raw.timestamp,
    }
}

pub fn compute_offset(samples: &[i32]) -> i32 {
    if samples.is_empty() {
        return 0;
    }
    let mut sum: i64 = 0;
    let mut i = 0;
    while i < samples.len() {
        sum += samples[i] as i64;
        i += 1;
    }
    (sum / samples.len() as i64) as i32
}
