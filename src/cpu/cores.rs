#[derive(Copy, Clone)]
pub struct CoreInfo {
    pub core_id: u32,
    pub frequency_hz: u64,
    pub raw_temp: Option<u64>,
}

pub fn detect_cores(out: &mut [CoreInfo]) -> usize {
    if let Some(info) = super::detect::detect_cpu_info() {
        let total = info.logical_cores as usize;
        let base_freq = info.frequency_hz;
        let mut count = 0usize;
        while count < out.len() && count < total {
            let per_core_freq = super::frequency::read_cpu_freq_sysfs(count as u8);
            out[count] = CoreInfo {
                core_id: count as u32,
                frequency_hz: if per_core_freq > 0 {
                    per_core_freq
                } else {
                    base_freq
                },
                raw_temp: crate::arch::shim::read_msr(0x19C),
            };
            count += 1;
        }
        return count;
    }

    if crate::arch::shim::read_aarch64_midr().is_some() {
        if let Some(info) = super::detect::detect_cpu_info() {
            let total = info.logical_cores as usize;
            let mut count = 0usize;
            while count < out.len() && count < total {
                let per_core_freq = super::frequency::read_cpu_freq_sysfs(count as u8);
                out[count] = CoreInfo {
                    core_id: count as u32,
                    frequency_hz: if per_core_freq > 0 {
                        per_core_freq
                    } else {
                        info.frequency_hz
                    },
                    raw_temp: None,
                };
                count += 1;
            }
            return count;
        }
        return 0;
    }

    0
}
