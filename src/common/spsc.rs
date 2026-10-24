use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

struct Inner<T: Copy, const N: usize> {
    slots: [UnsafeCell<Option<T>>; N],
    head: AtomicUsize,
    tail: AtomicUsize,
}

// Only the uniquely borrowed producer writes the head slot; only the uniquely
// borrowed consumer reads the tail slot. Acquire/release cursor publication
// prevents either endpoint from reusing a slot still accessed by the other.
// Separate UnsafeCells avoid creating references to the entire live ring.
unsafe impl<T: Copy + Send, const N: usize> Sync for Inner<T, N> {}

pub struct Queue<T: Copy, const N: usize> {
    inner: Inner<T, N>,
}
pub struct Producer<'a, T: Copy, const N: usize> {
    inner: &'a Inner<T, N>,
}
pub struct Consumer<'a, T: Copy, const N: usize> {
    inner: &'a Inner<T, N>,
}

impl<T: Copy, const N: usize> Queue<T, N> {
    pub const fn new() -> Self {
        assert!(N >= 2);
        Self {
            inner: Inner {
                slots: [const { UnsafeCell::new(None) }; N],
                head: AtomicUsize::new(0),
                tail: AtomicUsize::new(0),
            },
        }
    }

    // The queue cannot be inspected while its exclusive split borrow is active.
    pub fn len(&self) -> usize {
        let head = self.inner.head.load(Ordering::Relaxed);
        let tail = self.inner.tail.load(Ordering::Relaxed);
        if head >= tail {
            head - tail
        } else {
            N - tail + head
        }
    }

    // The exclusive queue borrow prevents a second active endpoint pair.
    pub fn split(&mut self) -> (Producer<'_, T, N>, Consumer<'_, T, N>) {
        (
            Producer { inner: &self.inner },
            Consumer { inner: &self.inner },
        )
    }
}

impl<T: Copy, const N: usize> Producer<'_, T, N> {
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        let head = self.inner.head.load(Ordering::Relaxed);
        let next = (head + 1) % N;
        if next == self.inner.tail.load(Ordering::Acquire) {
            return Err(value);
        }
        // Exclusive producer ownership and the free-slot check protect this slot.
        unsafe {
            *self.inner.slots[head].get() = Some(value);
        }
        self.inner.head.store(next, Ordering::Release);
        Ok(())
    }
}

impl<T: Copy, const N: usize> Consumer<'_, T, N> {
    pub fn dequeue(&mut self) -> Option<T> {
        let tail = self.inner.tail.load(Ordering::Relaxed);
        if tail == self.inner.head.load(Ordering::Acquire) {
            return None;
        }
        // The producer published a filled slot; it cannot reuse it before tail.
        let value = unsafe { (*self.inner.slots[tail].get()).take() };
        self.inner.tail.store((tail + 1) % N, Ordering::Release);
        value
    }
}

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fifo_capacity_wraparound_and_resplit() {
        let mut q = Queue::<u32, 4>::new();
        {
            let (mut p, mut c) = q.split();
            assert_eq!(c.dequeue(), None);
            for round in 0..1000 {
                for i in 0..3 {
                    assert_eq!(p.enqueue(round * 3 + i), Ok(()));
                }
                assert_eq!(p.enqueue(999999), Err(999999));
                for i in 0..3 {
                    assert_eq!(c.dequeue(), Some(round * 3 + i));
                }
                assert_eq!(c.dequeue(), None);
            }
            assert_eq!(p.enqueue(42), Ok(()));
        }
        let (_, mut c) = q.split();
        assert_eq!(c.dequeue(), Some(42));
        assert_eq!(c.dequeue(), None);
    }
    #[test]
    fn concurrent_ordered_transfer() {
        let mut q = Queue::<u32, 64>::new();
        let (mut p, mut c) = q.split();
        std::thread::scope(|scope| {
            scope.spawn(move || {
                for value in 0..100000 {
                    while p.enqueue(value).is_err() {
                        std::thread::yield_now();
                    }
                }
            });
            scope.spawn(move || {
                for expected in 0..100000 {
                    let actual = loop {
                        if let Some(value) = c.dequeue() {
                            break value;
                        }
                        std::thread::yield_now();
                    };
                    assert_eq!(actual, expected);
                }
                assert_eq!(c.dequeue(), None);
            });
        });
    }
    #[test]
    #[should_panic]
    fn invalid_capacity_is_rejected() {
        let _ = Queue::<u32, 1>::new();
    }
}
