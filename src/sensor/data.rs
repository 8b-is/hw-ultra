#[derive(Clone, Copy, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
pub struct RawSample {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub timestamp: u64,
}

#[derive(Clone, Copy)]
pub struct CalibratedSample {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub timestamp: u64,
}

#[derive(Clone, Copy)]
pub struct ScalarSample {
    pub value: i32,
    pub timestamp: u64,
}

impl RawSample {
    pub const fn zero() -> Self {
        RawSample {
            x: 0,
            y: 0,
            z: 0,
            timestamp: 0,
        }
    }

    pub fn magnitude_squared(&self) -> i64 {
        self.x as i64 * self.x as i64
            + self.y as i64 * self.y as i64
            + self.z as i64 * self.z as i64
    }

    pub fn axis(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

pub fn sign_extend_12(val: u32) -> i32 {
    let v = (val & 0xFFF) as i32;
    if v & 0x800 != 0 {
        v | !0xFFF
    } else {
        v
    }
}

pub fn sign_extend_16(val: u32) -> i32 {
    (val & 0xFFFF) as i16 as i32
}

pub fn combine_bytes(high: u8, low: u8) -> i16 {
    ((high as u16) << 8 | low as u16) as i16
}

pub fn scale_raw(raw: i32, scale_num: i32, scale_den: i32) -> i32 {
    if scale_den == 0 {
        return 0;
    }
    ((raw as i64 * scale_num as i64) / scale_den as i64) as i32
}
