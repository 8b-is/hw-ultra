use crate::lpu::device::LpuDevice;
use crate::lpu::pipeline::{LpuPipeline, PipelineJob};
use crate::lpu::quantization::{QuantFormat, Quantization};

#[derive(Copy, Clone)]
pub struct InferenceRequest {
    pub token_count: usize,
    pub hidden_size: usize,
    pub format: QuantFormat,
    pub flags: u32,
    pub align: usize,
}

#[derive(Copy, Clone)]
pub struct InferenceResult {
    pub submitted_bytes: usize,
    pub tokens_processed: usize,
    pub quantized: bool,
}

/// An owned inference engine; mutation requires an exclusive borrow.
/// ```compile_fail
/// use hw_ultra::sys::lpu::inference::{Inference, InferenceRequest};
/// fn shared(engine: &Inference, request: InferenceRequest) { engine.submit(request); }
/// ```
pub struct Inference {
    pipeline: LpuPipeline,
}

impl Default for Inference {
    fn default() -> Self {
        Self::new()
    }
}

impl Inference {
    pub const fn new() -> Self {
        Inference {
            pipeline: LpuPipeline::new(),
        }
    }

    pub fn submit(&mut self, req: InferenceRequest) -> bool {
        if req.token_count == 0 || req.hidden_size == 0 || req.align == 0 {
            return false;
        }
        let elem = Quantization::element_size(req.format);
        let bytes = req
            .token_count
            .saturating_mul(req.hidden_size)
            .saturating_mul(elem);
        self.pipeline.push(PipelineJob {
            bytes,
            flags: req.flags,
            align: req.align,
        })
    }

    pub fn run_next(&mut self, device: &LpuDevice) -> Result<InferenceResult, &'static str> {
        let job = self.pipeline.pop().ok_or("pipeline empty")?;
        let mut left = job.bytes;
        let mut submitted = 0usize;
        let chunk = [0u8; 512];
        while left > 0 {
            let take = core::cmp::min(left, chunk.len());
            let sent = device.submit_task(&chunk[..take], job.flags, job.align)?;
            submitted = submitted.saturating_add(sent);
            left = left.saturating_sub(take);
        }
        Ok(InferenceResult {
            submitted_bytes: submitted,
            tokens_processed: submitted,
            quantized: true,
        })
    }

    pub fn pending(&self) -> usize {
        self.pipeline.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_requests_without_hardware() {
        let mut e = Inference::new();
        let mut r = InferenceRequest {
            token_count: 0,
            hidden_size: 1,
            format: QuantFormat::Q8,
            flags: 0,
            align: 1,
        };
        assert!(!e.submit(r));
        assert_eq!(e.pending(), 0);
        r.token_count = 1;
        for _ in 0..32 {
            assert!(e.submit(r));
        }
        assert_eq!(e.pending(), 32);
        assert!(!e.submit(r));
    }
}
