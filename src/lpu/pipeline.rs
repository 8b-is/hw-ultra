use crate::common::spsc;

#[derive(Copy, Clone)]
pub struct PipelineJob {
    pub bytes: usize,
    pub flags: u32,
    pub align: usize,
}

// One reserved slot preserves the existing 32-job usable capacity.
const PIPELINE_SLOTS: usize = 33;
pub type Producer<'a> = spsc::Producer<'a, PipelineJob, PIPELINE_SLOTS>;
pub type Consumer<'a> = spsc::Consumer<'a, PipelineJob, PIPELINE_SLOTS>;

/// An owned inference queue. Shared references cannot mutate it.
/// ```compile_fail
/// use hw_ultra::sys::lpu::pipeline::{LpuPipeline, PipelineJob};
/// fn shared(pipeline: &LpuPipeline) {
///     pipeline.push(PipelineJob { bytes: 1, flags: 0, align: 1 });
/// }
/// ```
pub struct LpuPipeline {
    queue: spsc::Queue<PipelineJob, PIPELINE_SLOTS>,
}
impl Default for LpuPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl LpuPipeline {
    pub const fn new() -> Self {
        Self {
            queue: spsc::Queue::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn split(&mut self) -> (Producer<'_>, Consumer<'_>) {
        self.queue.split()
    }
    pub fn push(&mut self, job: PipelineJob) -> bool {
        let (mut producer, _) = self.split();
        producer.enqueue(job).is_ok()
    }
    pub fn pop(&mut self) -> Option<PipelineJob> {
        let (_, mut consumer) = self.split();
        consumer.dequeue()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_capacity_fifo_and_wraparound() {
        let mut p = LpuPipeline::new();
        for round in 0..40 {
            assert!(p.is_empty());
            for i in 0..32 {
                assert!(p.push(PipelineJob {
                    bytes: round * 32 + i,
                    flags: 0,
                    align: 1
                }));
            }
            assert_eq!(p.len(), 32);
            assert!(!p.push(PipelineJob {
                bytes: 9999,
                flags: 0,
                align: 1
            }));
            for i in 0..32 {
                assert_eq!(p.pop().unwrap().bytes, round * 32 + i);
            }
            assert_eq!(p.len(), 0);
            assert!(p.pop().is_none());
        }
    }
    #[test]
    fn split_preserves_pending_jobs() {
        let mut p = LpuPipeline::new();
        {
            let (mut producer, mut consumer) = p.split();
            assert!(producer
                .enqueue(PipelineJob {
                    bytes: 42,
                    flags: 7,
                    align: 8
                })
                .is_ok());
            assert_eq!(consumer.dequeue().unwrap().flags, 7);
        }
        assert!(p.is_empty());
    }
}
