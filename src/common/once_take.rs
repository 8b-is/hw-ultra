use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const TAKEN: u8 = 3;

/// Accepts one owned value and transfers it to at most one caller.
/// There are no borrowed-value getters and no reset operation.
pub struct OnceTake<T> {
    state: AtomicU8,
    value: UnsafeCell<MaybeUninit<T>>,
}

// Shared callers only compete for atomic ownership transitions. The winning
// writer initializes storage before publishing READY with release ordering.
// The winning taker acquires READY and moves the value out. No caller receives
// a reference into storage. T need only be Send because ownership is transferred.
unsafe impl<T: Send> Sync for OnceTake<T> {}

impl<T> OnceTake<T> {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(EMPTY),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
    pub fn set(&self, value: T) -> Result<(), T> {
        if self
            .state
            .compare_exchange(EMPTY, WRITING, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(value);
        }
        // This is the only writer, and no taker can claim WRITING storage.
        unsafe {
            (*self.value.get()).write(value);
        }
        self.state.store(READY, Ordering::Release);
        Ok(())
    }
    /// Returns None if empty, initialization is in progress, or already taken.
    /// It never waits. A later call may retry if initialization is in progress.
    pub fn take(&self) -> Option<T> {
        if self
            .state
            .compare_exchange(READY, TAKEN, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return None;
        }
        // READY published a fully initialized value, and TAKEN prevents another
        // reader, writer, or Drop from consuming these same bytes again.
        Some(unsafe { (*self.value.get()).assume_init_read() })
    }
}

impl<T> Drop for OnceTake<T> {
    fn drop(&mut self) {
        // Exclusive access means no method can still be using this instance.
        if *self.state.get_mut() == READY {
            unsafe {
                self.value.get_mut().assume_init_drop();
            }
        }
    }
}

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::AtomicUsize;
    use std::sync::{Arc, Barrier};
    struct Counted {
        id: usize,
        drops: Arc<AtomicUsize>,
    }
    impl Drop for Counted {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
    #[test]
    fn empty_take_and_no_reset() {
        let slot = OnceTake::new();
        assert_eq!(slot.take(), None);
        assert_eq!(slot.set(7), Ok(()));
        assert_eq!(slot.take(), Some(7));
        assert_eq!(slot.take(), None);
        assert_eq!(slot.set(8), Err(8));
        assert_eq!(slot.take(), None);
    }
    #[test]
    fn retained_value_is_dropped() {
        let drops = Arc::new(AtomicUsize::new(0));
        {
            let slot = OnceTake::new();
            assert!(slot
                .set(Counted {
                    id: 1,
                    drops: drops.clone()
                })
                .is_ok());
        }
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn rejected_and_taken_values_keep_single_ownership() {
        let drops = Arc::new(AtomicUsize::new(0));
        let slot = OnceTake::new();
        assert!(slot
            .set(Counted {
                id: 1,
                drops: drops.clone()
            })
            .is_ok());
        let rejected = slot
            .set(Counted {
                id: 2,
                drops: drops.clone(),
            })
            .err()
            .unwrap();
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(rejected);
        let taken = slot.take().unwrap();
        assert_eq!(taken.id, 1);
        drop(slot);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        drop(taken);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }
    #[test]
    fn competing_setters_publish_one_value() {
        let slot = OnceTake::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let wins = AtomicUsize::new(0);
        let winner = AtomicUsize::new(usize::MAX);
        let barrier = Barrier::new(16);
        std::thread::scope(|s| {
            for id in 0..16 {
                let slot = &slot;
                let drops = drops.clone();
                let wins = &wins;
                let winner = &winner;
                let barrier = &barrier;
                s.spawn(move || {
                    barrier.wait();
                    if slot.set(Counted { id, drops }).is_ok() {
                        wins.fetch_add(1, Ordering::SeqCst);
                        winner.store(id, Ordering::SeqCst);
                    }
                });
            }
        });
        assert_eq!(wins.load(Ordering::SeqCst), 1);
        assert_eq!(drops.load(Ordering::SeqCst), 15);
        let taken = slot.take().unwrap();
        assert_eq!(taken.id, winner.load(Ordering::SeqCst));
        drop(taken);
        drop(slot);
        assert_eq!(drops.load(Ordering::SeqCst), 16);
    }
    #[test]
    fn competing_takers_receive_one_value() {
        let slot = OnceTake::new();
        let drops = Arc::new(AtomicUsize::new(0));
        assert!(slot
            .set(Counted {
                id: 42,
                drops: drops.clone()
            })
            .is_ok());
        let wins = AtomicUsize::new(0);
        let barrier = Barrier::new(16);
        std::thread::scope(|s| {
            for _ in 0..16 {
                let slot = &slot;
                let wins = &wins;
                let barrier = &barrier;
                s.spawn(move || {
                    barrier.wait();
                    if let Some(value) = slot.take() {
                        assert_eq!(value.id, 42);
                        wins.fetch_add(1, Ordering::SeqCst);
                    }
                });
            }
        });
        assert_eq!(wins.load(Ordering::SeqCst), 1);
        drop(slot);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn send_but_non_sync_payload_transfers() {
        let slot = OnceTake::new();
        assert!(slot.set(core::cell::Cell::new(3)).is_ok());
        std::thread::scope(|s| {
            s.spawn(|| {
                let value = slot.take().unwrap();
                value.set(4);
                assert_eq!(value.get(), 4);
            });
        });
        assert!(slot.take().is_none());
    }
}
