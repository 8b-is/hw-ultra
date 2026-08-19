use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum PollState {
    Idle,
    Polling,
    TargetFound,
    Activated,
    DataExchange,
}

static POLL_STATE: AtomicU8 = AtomicU8::new(0);
static POLL_INTERVAL_MS: AtomicU8 = AtomicU8::new(100);

fn state_to_u8(s: PollState) -> u8 {
    match s {
        PollState::Idle => 0,
        PollState::Polling => 1,
        PollState::TargetFound => 2,
        PollState::Activated => 3,
        PollState::DataExchange => 4,
    }
}

fn u8_to_state(v: u8) -> PollState {
    match v {
        1 => PollState::Polling,
        2 => PollState::TargetFound,
        3 => PollState::Activated,
        4 => PollState::DataExchange,
        _ => PollState::Idle,
    }
}

pub fn set_state(state: PollState) {
    POLL_STATE.store(state_to_u8(state), Ordering::Release);
}

pub fn current_state() -> PollState {
    u8_to_state(POLL_STATE.load(Ordering::Acquire))
}

pub fn is_active() -> bool {
    current_state() != PollState::Idle
}

pub fn set_interval_ms(ms: u8) {
    POLL_INTERVAL_MS.store(ms, Ordering::Release);
}

pub fn interval_ms() -> u8 {
    POLL_INTERVAL_MS.load(Ordering::Acquire)
}

pub fn poll_cycle(base: usize) -> bool {
    let present = crate::nfc::hw::target_present(base);
    if present {
        set_state(PollState::TargetFound);
    } else {
        set_state(PollState::Polling);
    }
    present
}

pub fn start(base: usize) {
    crate::nfc::hw::rf_field_on(base);
    crate::nfc::hw::start_poll(base);
    set_state(PollState::Polling);
}

pub fn stop(base: usize) {
    crate::nfc::hw::stop_poll(base);
    crate::nfc::hw::rf_field_off(base);
    set_state(PollState::Idle);
}
