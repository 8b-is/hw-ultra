use crate::gpu::queue::Queue;

pub struct GpuScheduler {
    queue: Queue,
}

impl Default for GpuScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuScheduler {
    pub fn new() -> Self {
        GpuScheduler {
            queue: Queue::new(),
        }
    }

    /// Borrow one producer and one consumer for concurrent command handling.
    pub fn split(
        &mut self,
    ) -> (
        crate::gpu::queue::Producer<'_>,
        crate::gpu::queue::Consumer<'_>,
    ) {
        self.queue.split()
    }

    pub fn submit(&mut self, cmd: crate::gpu::Command) -> bool {
        self.queue.enqueue(cmd)
    }

    pub fn poll(&mut self) -> Option<crate::gpu::Command> {
        self.queue.dequeue()
    }
}
