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
        let mut count = self.counter.load(Ordering::Acquire);
        while count < self.max_ops {
            match self.counter.compare_exchange_weak(
                count,
                count + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(observed) => count = observed,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn exhausted_counter_never_wraps_to_allowed() {
        let guard = Guard::new(10);
        guard.counter.store(u32::MAX, Ordering::Relaxed);
        assert!(!guard.allow());
        assert!(!guard.allow());
    }

    #[test]
    fn zero_and_maximum_limits() {
        assert!(!Guard::new(0).allow());
        let guard = Guard::new(u32::MAX);
        guard.counter.store(u32::MAX - 1, Ordering::Relaxed);
        assert!(guard.allow());
        assert!(!guard.allow());
        assert!(!guard.allow());
    }

    #[test]
    fn concurrent_callers_share_exact_budget() {
        let guard = std::sync::Arc::new(Guard::new(1000));
        let handles: std::vec::Vec<_> = (0..4)
            .map(|_| {
                let guard = std::sync::Arc::clone(&guard);
                std::thread::spawn(move || (0..1000).filter(|_| guard.allow()).count())
            })
            .collect();
        let allowed: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
        assert_eq!(allowed, 1000);
        assert!(!guard.allow());
    }
}
