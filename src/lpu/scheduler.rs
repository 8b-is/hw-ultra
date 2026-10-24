use crate::common::spsc;
use crate::lpu::device::LpuDevice;
use crate::lpu::inference::{Inference, InferenceRequest, InferenceResult};

const SCHED_SLOTS: usize = 33;
pub type Producer<'a> = spsc::Producer<'a, InferenceRequest, SCHED_SLOTS>;
pub type Consumer<'a> = spsc::Consumer<'a, InferenceRequest, SCHED_SLOTS>;
pub struct LpuScheduler {
    queue: spsc::Queue<InferenceRequest, SCHED_SLOTS>,
}
impl Default for LpuScheduler {
    fn default() -> Self {
        Self::new()
    }
}
impl LpuScheduler {
    pub const fn new() -> Self {
        Self {
            queue: spsc::Queue::new(),
        }
    }
    pub fn split(&mut self) -> (Producer<'_>, Consumer<'_>) {
        self.queue.split()
    }
    pub fn enqueue(&mut self, req: InferenceRequest) -> bool {
        let (mut producer, _) = self.split();
        producer.enqueue(req).is_ok()
    }
    pub fn dequeue(&mut self) -> Option<InferenceRequest> {
        let (_, mut consumer) = self.split();
        consumer.dequeue()
    }
    pub fn pending(&self) -> usize {
        self.queue.len()
    }
    pub fn run_one(
        &mut self,
        engine: &mut Inference,
        device: &LpuDevice,
    ) -> Result<InferenceResult, &'static str> {
        let req = self.dequeue().ok_or("scheduler empty")?;
        if !engine.submit(req) {
            return Err("pipeline submit failed");
        }
        engine.run_next(device)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpu::quantization::QuantFormat;
    #[test]
    fn preserves_all_32_requests_and_split_ownership() {
        let mut s = LpuScheduler::new();
        let req = |flags| InferenceRequest {
            token_count: 1,
            hidden_size: 1,
            format: QuantFormat::Q8,
            flags,
            align: 1,
        };
        for flags in 0..32 {
            assert!(s.enqueue(req(flags)));
        }
        assert_eq!(s.pending(), 32);
        assert!(!s.enqueue(req(99)));
        for flags in 0..32 {
            assert_eq!(s.dequeue().unwrap().flags, flags);
        }
        {
            let (mut producer, mut consumer) = s.split();
            assert!(producer.enqueue(req(42)).is_ok());
            assert_eq!(consumer.dequeue().unwrap().flags, 42);
        }
        assert_eq!(s.pending(), 0);
    }
}
