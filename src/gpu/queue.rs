use crate::gpu::Command;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

const Q_SIZE: usize = 64;

pub struct Queue {
    ring: UnsafeCell<[Option<Command>; Q_SIZE]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl Sync for Queue {}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            ring: UnsafeCell::new([None; Q_SIZE]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn enqueue(&self, cmd: Command) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        let next = (head + 1) % Q_SIZE;
        if next == tail {
            return false;
        }
        unsafe {
            (*self.ring.get())[head] = Some(cmd);
        }
        self.head.store(next, Ordering::Release);
        true
    }

    pub fn dequeue(&self) -> Option<Command> {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        if tail == head {
            return None;
        }
        let cmd = unsafe { (*self.ring.get())[tail].take() };
        let next = (tail + 1) % Q_SIZE;
        self.tail.store(next, Ordering::Release);
        cmd
    }
}
