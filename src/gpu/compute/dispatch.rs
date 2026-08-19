use core::sync::atomic::{AtomicUsize, Ordering};

static DISPATCH_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn dispatch_kernel() {
    DISPATCH_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub fn dispatch_count() -> usize {
    DISPATCH_COUNT.load(Ordering::Acquire)
}
