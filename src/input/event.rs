#[derive(Clone, Copy, PartialEq)]
pub enum EventType {
    KeyPress,
    KeyRelease,
    TouchDown,
    TouchUp,
    TouchMove,
    ButtonPress,
    ButtonRelease,
    AbsoluteAxis,
    RelativeAxis,
}

#[derive(Clone, Copy)]
pub struct InputEvent {
    pub event_type: EventType,
    pub code: u16,
    pub value: i32,
    pub timestamp: u64,
}

impl InputEvent {
    pub const fn new(event_type: EventType, code: u16, value: i32, timestamp: u64) -> Self {
        InputEvent {
            event_type,
            code,
            value,
            timestamp,
        }
    }

    pub fn key_press(code: u16, timestamp: u64) -> Self {
        InputEvent::new(EventType::KeyPress, code, 1, timestamp)
    }

    pub fn key_release(code: u16, timestamp: u64) -> Self {
        InputEvent::new(EventType::KeyRelease, code, 0, timestamp)
    }

    pub fn touch_down(x: i32, y: i32, id: u16, timestamp: u64) -> Self {
        let _ = y;
        InputEvent::new(EventType::TouchDown, id, x, timestamp)
    }

    pub fn touch_move(x: i32, y: i32, id: u16, timestamp: u64) -> Self {
        let _ = y;
        InputEvent::new(EventType::TouchMove, id, x, timestamp)
    }

    pub fn is_key_event(&self) -> bool {
        matches!(self.event_type, EventType::KeyPress | EventType::KeyRelease)
    }

    pub fn is_touch_event(&self) -> bool {
        matches!(
            self.event_type,
            EventType::TouchDown | EventType::TouchUp | EventType::TouchMove
        )
    }
}
