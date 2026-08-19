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

    pub fn submit(&self, cmd: crate::gpu::Command) -> bool {
        self.queue.enqueue(cmd)
    }

    pub fn poll(&self) -> Option<crate::gpu::Command> {
        self.queue.dequeue()
    }
}
