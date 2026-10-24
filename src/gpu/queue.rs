use crate::common::spsc;
use crate::gpu::Command;

const Q_SIZE: usize = 64;

/// The unique producer endpoint of a split GPU queue.
pub type Producer<'a> = spsc::Producer<'a, Command, Q_SIZE>;
/// The unique consumer endpoint of a split GPU queue.
pub type Consumer<'a> = spsc::Consumer<'a, Command, Q_SIZE>;

/// A 63-command ring with exclusive endpoint ownership.
///
/// Single-owner access requires a mutable queue. For concurrent access, split
/// once and move one endpoint to each thread; endpoints are not cloneable.
///
/// ```compile_fail
/// use hw_ultra::sys::gpu::{queue::Queue, Command};
/// let mut queue = Queue::new();
/// let (mut producer, _) = queue.split();
/// let (_, mut consumer) = queue.split();
/// producer.enqueue(Command::nop()).ok();
/// consumer.dequeue();
/// ```
///
/// ```compile_fail
/// use hw_ultra::sys::gpu::{queue::Producer, Command};
/// fn enqueue_shared(producer: &Producer<'_>) {
///     producer.enqueue(Command::nop()).ok();
/// }
/// ```
pub struct Queue {
    ring: spsc::Queue<Command, Q_SIZE>,
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

impl Queue {
    pub const fn new() -> Self {
        Self {
            ring: spsc::Queue::new(),
        }
    }

    /// Borrow the sole producer and consumer until both endpoints are dropped.
    pub fn split(&mut self) -> (Producer<'_>, Consumer<'_>) {
        self.ring.split()
    }

    /// Submit from one exclusive owner. Returns false only when full.
    pub fn enqueue(&mut self, cmd: Command) -> bool {
        let (mut producer, _) = self.split();
        producer.enqueue(cmd).is_ok()
    }

    /// Remove the oldest command from one exclusive owner.
    pub fn dequeue(&mut self) -> Option<Command> {
        let (_, mut consumer) = self.split();
        consumer.dequeue()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_queue_capacity_and_order() {
        let mut q = Queue::new();
        for opcode in 1..=63 {
            assert!(q.enqueue(Command::new(opcode, core::ptr::null(), 0)));
        }
        assert!(!q.enqueue(Command::nop()));
        for opcode in 1..=63 {
            assert_eq!(q.dequeue().unwrap().opcode, opcode);
        }
        assert!(q.dequeue().is_none());
    }
    #[test]
    fn scheduler_and_command_use_exclusive_ownership() {
        let mut scheduler = crate::gpu::scheduler::GpuScheduler::new();
        assert!(Command::nop().submit(&mut scheduler));
        assert!(scheduler.poll().unwrap().is_nop());
        let (mut producer, mut consumer) = scheduler.split();
        assert!(producer
            .enqueue(Command::new(42, core::ptr::null(), 0))
            .is_ok());
        assert_eq!(consumer.dequeue().unwrap().opcode, 42);
    }
}
