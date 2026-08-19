use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Vector {
    pub width_bits: u32,
    pub support_level: u8,
}

static VEC_WIDTH: AtomicUsize = AtomicUsize::new(0);

pub fn detect() -> Vector {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            let feat = crate::arch::x86_64::cpu::features::detect();
            let (width, level) = if feat.avx2 {
                (256u32, 3u8)
            } else if feat.avx {
                (256, 2)
            } else if feat.sse2 {
                (128, 1)
            } else {
                (0, 0)
            };
            VEC_WIDTH.store(width as usize, Ordering::Release);
            Vector {
                width_bits: width,
                support_level: level,
            }
        }
        _ => {
            VEC_WIDTH.store(128, Ordering::Release);
            Vector {
                width_bits: 128,
                support_level: 1,
            }
        }
    }
}
