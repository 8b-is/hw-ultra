use super::command::Command;
use core::sync::atomic::{AtomicU16, Ordering};

const MAX_QUEUE_DEPTH: usize = 32;

pub struct CommandQueue {
    entries: [Option<Command>; MAX_QUEUE_DEPTH],
    head: usize,
    tail: usize,
    count: usize,
}

static INFLIGHT: AtomicU16 = AtomicU16::new(0);

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandQueue {
    pub const fn new() -> Self {
        const NONE: Option<Command> = None;
        CommandQueue {
            entries: [NONE; MAX_QUEUE_DEPTH],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn submit(&mut self, cmd: Command) -> bool {
        if self.count >= MAX_QUEUE_DEPTH {
            return false;
        }
        self.entries[self.tail] = Some(cmd);
        self.tail = (self.tail + 1) % MAX_QUEUE_DEPTH;
        self.count += 1;
        INFLIGHT.fetch_add(1, Ordering::AcqRel);
        true
    }

    pub fn dequeue(&mut self) -> Option<Command> {
        if self.count == 0 {
            return None;
        }
        let cmd = self.entries[self.head].take();
        self.head = (self.head + 1) % MAX_QUEUE_DEPTH;
        self.count -= 1;
        if cmd.is_some() {
            INFLIGHT.fetch_sub(1, Ordering::AcqRel);
        }
        cmd
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count >= MAX_QUEUE_DEPTH
    }

    pub fn depth(&self) -> usize {
        self.count
    }
}

pub fn inflight_count() -> u16 {
    INFLIGHT.load(Ordering::Acquire)
}
