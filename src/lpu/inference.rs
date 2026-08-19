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

    pub fn submit(&self, req: InferenceRequest) -> bool {
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

    pub fn run_next(&self, device: &LpuDevice) -> Result<InferenceResult, &'static str> {
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
