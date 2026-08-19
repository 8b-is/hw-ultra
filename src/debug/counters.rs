use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_COUNTERS: usize = 32;

static COUNTER_VALUES: [AtomicUsize; MAX_COUNTERS] = [const { AtomicUsize::new(0) }; MAX_COUNTERS];
static COUNTER_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct Counters {
    pub id: usize,
}

pub fn allocate() -> Option<Counters> {
    let id = COUNTER_COUNT.fetch_add(1, Ordering::AcqRel);
    if id >= MAX_COUNTERS {
        COUNTER_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    Some(Counters { id })
}

pub fn increment(c: &Counters) {
    if c.id < MAX_COUNTERS {
        COUNTER_VALUES[c.id].fetch_add(1, Ordering::Relaxed);
    }
}

pub fn read(c: &Counters) -> usize {
    if c.id < MAX_COUNTERS {
        COUNTER_VALUES[c.id].load(Ordering::Relaxed)
    } else {
        0
    }
}

pub fn reset(c: &Counters) {
    if c.id < MAX_COUNTERS {
        COUNTER_VALUES[c.id].store(0, Ordering::Relaxed);
    }
}

pub fn total_counters() -> usize {
    COUNTER_COUNT.load(Ordering::Acquire)
}
