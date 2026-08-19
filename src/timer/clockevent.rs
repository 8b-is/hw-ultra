use core::sync::atomic::{AtomicUsize, Ordering};

pub struct ClockEvent {
    pub interval_ns: u64,
    pub handler: fn(),
}

static INTERVAL: AtomicUsize = AtomicUsize::new(0);
static EVENT_COUNT: AtomicUsize = AtomicUsize::new(0);

static ACTIVE_EVENT: crate::common::once::Once<ClockEvent> = crate::common::once::Once::new();

pub fn register(event: ClockEvent) -> bool {
    INTERVAL.store(event.interval_ns as usize, Ordering::Release);
    ACTIVE_EVENT.set(event)
}

pub fn fire() {
    if let Some(ev) = ACTIVE_EVENT.get() {
        EVENT_COUNT.fetch_add(1, Ordering::AcqRel);
        (ev.handler)();
    }
}

pub fn event_count() -> usize {
    EVENT_COUNT.load(Ordering::Acquire)
}

pub fn interval() -> u64 {
    INTERVAL.load(Ordering::Acquire) as u64
}
