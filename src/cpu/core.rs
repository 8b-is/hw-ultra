use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Core {
    pub id: u32,
    pub online: bool,
    pub frequency_hz: u64,
}

const MAX_CORES: usize = 64;

struct CoreTable {
    ids: [AtomicUsize; MAX_CORES],
    count: AtomicUsize,
}

unsafe impl Sync for CoreTable {}

static CORES: CoreTable = CoreTable {
    ids: [const { AtomicUsize::new(0) }; MAX_CORES],
    count: AtomicUsize::new(0),
};

pub fn register_core(id: u32) -> bool {
    let idx = CORES.count.load(Ordering::Acquire);
    if idx >= MAX_CORES {
        return false;
    }
    CORES.ids[idx].store(id as usize, Ordering::Release);
    CORES.count.store(idx + 1, Ordering::Release);
    true
}

pub fn core_count() -> usize {
    CORES.count.load(Ordering::Acquire)
}

pub fn core_id(index: usize) -> Option<u32> {
    if index >= CORES.count.load(Ordering::Acquire) {
        return None;
    }
    Some(CORES.ids[index].load(Ordering::Acquire) as u32)
}
