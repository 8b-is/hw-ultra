use core::sync::atomic::{AtomicUsize, Ordering};

const TRACE_BUFFER_SIZE: usize = 64;

static TRACE_BUFFER: [AtomicUsize; TRACE_BUFFER_SIZE] =
    [const { AtomicUsize::new(0) }; TRACE_BUFFER_SIZE];
static TRACE_HEAD: AtomicUsize = AtomicUsize::new(0);
static TRACE_TOTAL: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct TraceEntry {
    pub event_id: usize,
    pub timestamp: u64,
}

pub fn trace_event_with_id(event_id: usize) {
    let ts = crate::debug::perf::read_timestamp();
    let combined = (event_id & 0xFFFF) | (((ts as usize) & 0xFFFF_FFFF_FFFF) << 16);
    let idx = TRACE_HEAD.fetch_add(1, Ordering::AcqRel) % TRACE_BUFFER_SIZE;
    TRACE_BUFFER[idx].store(combined, Ordering::Release);
    TRACE_TOTAL.fetch_add(1, Ordering::Relaxed);
}

pub fn trace_event() {
    trace_event_with_id(0);
}

pub fn read_trace(idx: usize) -> Option<TraceEntry> {
    if idx >= TRACE_BUFFER_SIZE {
        return None;
    }
    let raw = TRACE_BUFFER[idx].load(Ordering::Acquire);
    if raw == 0 {
        return None;
    }
    Some(TraceEntry {
        event_id: raw & 0xFFFF,
        timestamp: ((raw >> 16) & 0xFFFF_FFFF_FFFF) as u64,
    })
}

pub fn total_events() -> usize {
    TRACE_TOTAL.load(Ordering::Acquire)
}

pub fn current_head() -> usize {
    TRACE_HEAD.load(Ordering::Acquire) % TRACE_BUFFER_SIZE
}
