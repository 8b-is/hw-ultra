use core::sync::atomic::{AtomicU16, AtomicU8, Ordering};

pub const MAX_TOUCH_POINTS: usize = 10;

#[derive(Clone, Copy, PartialEq)]
pub enum TouchState {
    Released,
    Pressed,
    Moving,
}

#[derive(Clone, Copy)]
pub struct TouchPoint {
    pub id: u8,
    pub x: u16,
    pub y: u16,
    pub pressure: u16,
    pub state: TouchState,
}

impl TouchPoint {
    pub const fn empty() -> Self {
        TouchPoint {
            id: 0,
            x: 0,
            y: 0,
            pressure: 0,
            state: TouchState::Released,
        }
    }
}

static SCREEN_WIDTH: AtomicU16 = AtomicU16::new(0);
static SCREEN_HEIGHT: AtomicU16 = AtomicU16::new(0);
static ACTIVE_TOUCHES: AtomicU8 = AtomicU8::new(0);

pub fn set_screen_size(width: u16, height: u16) {
    SCREEN_WIDTH.store(width, Ordering::Release);
    SCREEN_HEIGHT.store(height, Ordering::Release);
}

pub fn screen_width() -> u16 {
    SCREEN_WIDTH.load(Ordering::Acquire)
}

pub fn screen_height() -> u16 {
    SCREEN_HEIGHT.load(Ordering::Acquire)
}

pub fn calibrate_point(raw_x: u16, raw_y: u16, raw_max_x: u16, raw_max_y: u16) -> (u16, u16) {
    let sw = screen_width() as u32;
    let sh = screen_height() as u32;
    if raw_max_x == 0 || raw_max_y == 0 {
        return (raw_x, raw_y);
    }
    let x = ((raw_x as u32) * sw) / raw_max_x as u32;
    let y = ((raw_y as u32) * sh) / raw_max_y as u32;
    (x as u16, y as u16)
}

pub fn set_active_touches(count: u8) {
    let c = if count as usize > MAX_TOUCH_POINTS {
        MAX_TOUCH_POINTS as u8
    } else {
        count
    };
    ACTIVE_TOUCHES.store(c, Ordering::Release);
}

pub fn active_touch_count() -> u8 {
    ACTIVE_TOUCHES.load(Ordering::Acquire)
}

pub fn distance_squared(p1: &TouchPoint, p2: &TouchPoint) -> u32 {
    let dx = p1.x as i32 - p2.x as i32;
    let dy = p1.y as i32 - p2.y as i32;
    (dx * dx + dy * dy) as u32
}
