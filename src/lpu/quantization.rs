#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuantFormat {
    Fp32,
    Q8,
}

pub struct Quantization;

impl Quantization {
    pub fn element_size(format: QuantFormat) -> usize {
        match format {
            QuantFormat::Fp32 => 4,
            QuantFormat::Q8 => 1,
        }
    }

    pub fn quantize_i8(input_q16: &[i32], scale_q16: u32, out: &mut [i8]) -> usize {
        if scale_q16 == 0 || out.is_empty() {
            return 0;
        }
        let count = core::cmp::min(input_q16.len(), out.len());
        let scale = scale_q16 as i64;
        for idx in 0..count {
            let v = input_q16[idx] as i64;
            let q = (v << 8) / scale;
            let clamped = if q < -128 {
                -128
            } else if q > 127 {
                127
            } else {
                q as i32
            };
            out[idx] = clamped as i8;
        }
        count
    }

    pub fn dequantize_i8(input: &[i8], scale_q16: u32, out_q16: &mut [i32]) -> usize {
        if out_q16.is_empty() {
            return 0;
        }
        let count = core::cmp::min(input.len(), out_q16.len());
        for idx in 0..count {
            out_q16[idx] = (input[idx] as i32) * (scale_q16 as i32);
        }
        count
    }
}
