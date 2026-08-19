use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum SimState {
    Absent,
    Ready,
    PinRequired,
    PukRequired,
    Locked,
    Error,
}

static SIM_STATE: AtomicU8 = AtomicU8::new(0);

fn state_to_u8(s: SimState) -> u8 {
    match s {
        SimState::Absent => 0,
        SimState::Ready => 1,
        SimState::PinRequired => 2,
        SimState::PukRequired => 3,
        SimState::Locked => 4,
        SimState::Error => 5,
    }
}

fn u8_to_state(v: u8) -> SimState {
    match v {
        1 => SimState::Ready,
        2 => SimState::PinRequired,
        3 => SimState::PukRequired,
        4 => SimState::Locked,
        5 => SimState::Error,
        _ => SimState::Absent,
    }
}

pub fn set_state(state: SimState) {
    SIM_STATE.store(state_to_u8(state), Ordering::Release);
}

pub fn current_state() -> SimState {
    u8_to_state(SIM_STATE.load(Ordering::Acquire))
}

pub fn is_ready() -> bool {
    current_state() == SimState::Ready
}

pub fn is_present() -> bool {
    current_state() != SimState::Absent
}

pub fn needs_pin() -> bool {
    current_state() == SimState::PinRequired
}

pub fn parse_cpin_response(response: &[u8]) -> SimState {
    let mut i = 0;
    while i + 5 <= response.len() {
        if &response[i..i + 5] == b"READY" {
            return SimState::Ready;
        }
        i += 1;
    }
    i = 0;
    while i + 6 <= response.len() {
        if &response[i..i + 6] == b"SIM PI" {
            return SimState::PinRequired;
        }
        i += 1;
    }
    i = 0;
    while i + 6 <= response.len() {
        if &response[i..i + 6] == b"SIM PU" {
            return SimState::PukRequired;
        }
        i += 1;
    }
    SimState::Error
}
