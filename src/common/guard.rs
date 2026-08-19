use core::sync::atomic::{AtomicU32, Ordering};

pub struct Guard {
    max_ops: u32,
    counter: AtomicU32,
}

impl Guard {
    pub const fn new(max_ops: u32) -> Self {
        Self {
            max_ops,
            counter: AtomicU32::new(0),
        }
    }
    pub fn allow(&self) -> bool {
        let prev = self.counter.fetch_add(1, Ordering::AcqRel);
        prev < self.max_ops
    }
}
