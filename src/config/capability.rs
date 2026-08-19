use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_CAPS: usize = 16;

static CAPS: [AtomicUsize; MAX_CAPS] = [const { AtomicUsize::new(0) }; MAX_CAPS];

static CAP_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn register_capability(id: usize, value: usize) -> bool {
    if id >= MAX_CAPS {
        return false;
    }
    CAPS[id].store(value, Ordering::Release);
    let cur = CAP_COUNT.load(Ordering::Acquire);
    if id >= cur {
        CAP_COUNT.store(id + 1, Ordering::Release);
    }
    true
}

pub fn read_capability(id: usize) -> Option<usize> {
    if id >= MAX_CAPS {
        return None;
    }
    if id >= CAP_COUNT.load(Ordering::Acquire) {
        return None;
    }
    Some(CAPS[id].load(Ordering::Acquire))
}

pub fn capability_count() -> usize {
    CAP_COUNT.load(Ordering::Acquire)
}
