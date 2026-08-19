use crate::lpu::device::LpuDevice;
use crate::lpu::inference::{Inference, InferenceRequest, InferenceResult};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

const SCHED_CAPACITY: usize = 32;

pub struct LpuScheduler {
    queue: UnsafeCell<[Option<InferenceRequest>; SCHED_CAPACITY]>,
    head: AtomicUsize,
    tail: AtomicUsize,
    count: AtomicUsize,
}

unsafe impl Sync for LpuScheduler {}

impl Default for LpuScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl LpuScheduler {
    pub const fn new() -> Self {
        LpuScheduler {
            queue: UnsafeCell::new([None; SCHED_CAPACITY]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            count: AtomicUsize::new(0),
        }
    }

    pub fn enqueue(&self, req: InferenceRequest) -> bool {
        let count = self.count.load(Ordering::Acquire);
        if count >= SCHED_CAPACITY {
            return false;
        }
        let head = self.head.load(Ordering::Acquire);
        unsafe {
            (*self.queue.get())[head] = Some(req);
        }
        self.head
            .store((head + 1) % SCHED_CAPACITY, Ordering::Release);
        self.count.fetch_add(1, Ordering::AcqRel);
        true
    }

    pub fn dequeue(&self) -> Option<InferenceRequest> {
        let count = self.count.load(Ordering::Acquire);
        if count == 0 {
            return None;
        }
        let tail = self.tail.load(Ordering::Acquire);
        let req = unsafe {
            let queue = &mut *self.queue.get();
            queue[tail].take()
        };
        self.tail
            .store((tail + 1) % SCHED_CAPACITY, Ordering::Release);
        self.count.fetch_sub(1, Ordering::AcqRel);
        req
    }

    pub fn pending(&self) -> usize {
        self.count.load(Ordering::Acquire)
    }

    pub fn run_one(
        &self,
        engine: &Inference,
        device: &LpuDevice,
    ) -> Result<InferenceResult, &'static str> {
        let req = self.dequeue().ok_or("scheduler empty")?;
        if !engine.submit(req) {
            return Err("pipeline submit failed");
        }
        engine.run_next(device)
    }
}
