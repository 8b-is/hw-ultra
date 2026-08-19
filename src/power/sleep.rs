use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SleepState {
    S0,
    S1,
    S3,
    S5,
}

static CURRENT_STATE: AtomicU8 = AtomicU8::new(0);

pub fn suspend(state: SleepState) {
    let val = match state {
        SleepState::S0 => 0,
        SleepState::S1 => 1,
        SleepState::S3 => 3,
        SleepState::S5 => 5,
    };
    CURRENT_STATE.store(val, Ordering::Release);
    match state {
        SleepState::S1 => {
            crate::sys::sleep_ns(100_000_000);
        }
        SleepState::S3 => {
            crate::sys::sleep_secs(1);
        }
        SleepState::S5 => {
            crate::power::core::shutdown();
        }
        SleepState::S0 => {}
    }
}

pub fn current_state() -> SleepState {
    match CURRENT_STATE.load(Ordering::Acquire) {
        1 => SleepState::S1,
        3 => SleepState::S3,
        5 => SleepState::S5,
        _ => SleepState::S0,
    }
}
