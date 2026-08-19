use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone)]
pub struct PipelineJob {
    pub bytes: usize,
    pub flags: u32,
    pub align: usize,
}

const PIPELINE_DEPTH: usize = 32;

pub struct LpuPipeline {
    slots: UnsafeCell<[Option<PipelineJob>; PIPELINE_DEPTH]>,
    head: AtomicUsize,
    tail: AtomicUsize,
    count: AtomicUsize,
}

unsafe impl Sync for LpuPipeline {}

impl Default for LpuPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl LpuPipeline {
    pub const fn new() -> Self {
        LpuPipeline {
            slots: UnsafeCell::new([None; PIPELINE_DEPTH]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            count: AtomicUsize::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.count.load(Ordering::Acquire)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&self, job: PipelineJob) -> bool {
        let count = self.count.load(Ordering::Acquire);
        if count >= PIPELINE_DEPTH {
            return false;
        }
        let head = self.head.load(Ordering::Acquire);
        unsafe {
            (*self.slots.get())[head] = Some(job);
        }
        self.head
            .store((head + 1) % PIPELINE_DEPTH, Ordering::Release);
        self.count.fetch_add(1, Ordering::AcqRel);
        true
    }

    pub fn pop(&self) -> Option<PipelineJob> {
        let count = self.count.load(Ordering::Acquire);
        if count == 0 {
            return None;
        }
        let tail = self.tail.load(Ordering::Acquire);
        let item = unsafe {
            let slots = &mut *self.slots.get();
            slots[tail].take()
        };
        self.tail
            .store((tail + 1) % PIPELINE_DEPTH, Ordering::Release);
        self.count.fetch_sub(1, Ordering::AcqRel);
        item
    }
}
