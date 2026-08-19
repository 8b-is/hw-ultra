use core::sync::atomic::{AtomicU8, Ordering};

pub const MAX_BUTTONS: usize = 16;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonState {
    Released,
    Pressed,
}

static BUTTON_STATES: AtomicU16Arr = AtomicU16Arr::new();

struct AtomicU16Arr {
    bits: core::sync::atomic::AtomicU16,
}

impl AtomicU16Arr {
    const fn new() -> Self {
        AtomicU16Arr {
            bits: core::sync::atomic::AtomicU16::new(0),
        }
    }
}

static DEBOUNCE_MS: AtomicU8 = AtomicU8::new(20);

pub fn set_debounce_ms(ms: u8) {
    DEBOUNCE_MS.store(ms, Ordering::Release);
}

pub fn debounce_ms() -> u8 {
    DEBOUNCE_MS.load(Ordering::Acquire)
}

pub fn set_button_state(button: u8, state: ButtonState) {
    if button as usize >= MAX_BUTTONS {
        return;
    }
    let mut bits = BUTTON_STATES.bits.load(Ordering::Acquire);
    match state {
        ButtonState::Pressed => bits |= 1 << button,
        ButtonState::Released => bits &= !(1 << button),
    }
    BUTTON_STATES.bits.store(bits, Ordering::Release);
}

pub fn get_button_state(button: u8) -> ButtonState {
    if button as usize >= MAX_BUTTONS {
        return ButtonState::Released;
    }
    let bits = BUTTON_STATES.bits.load(Ordering::Acquire);
    if bits & (1 << button) != 0 {
        ButtonState::Pressed
    } else {
        ButtonState::Released
    }
}

pub fn any_button_pressed() -> bool {
    BUTTON_STATES.bits.load(Ordering::Acquire) != 0
}

pub fn pressed_mask() -> u16 {
    BUTTON_STATES.bits.load(Ordering::Acquire)
}

pub fn should_accept(last_change_us: u64, now_us: u64) -> bool {
    let threshold = debounce_ms() as u64 * 1000;
    now_us.wrapping_sub(last_change_us) >= threshold
}
