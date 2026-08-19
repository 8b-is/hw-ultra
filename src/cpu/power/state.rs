use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CpuPowerState {
    Active,
    Idle,
    DeepIdle,
    Offline,
}

static STATE: AtomicU8 = AtomicU8::new(0);

pub fn set_state(s: CpuPowerState) {
    let val = match s {
        CpuPowerState::Active => 0,
        CpuPowerState::Idle => 1,
        CpuPowerState::DeepIdle => 2,
        CpuPowerState::Offline => 3,
    };
    STATE.store(val, Ordering::Release);
}

pub fn get_state() -> CpuPowerState {
    match STATE.load(Ordering::Acquire) {
        1 => CpuPowerState::Idle,
        2 => CpuPowerState::DeepIdle,
        3 => CpuPowerState::Offline,
        _ => CpuPowerState::Active,
    }
}
