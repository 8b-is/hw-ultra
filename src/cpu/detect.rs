use super::info::CpuInfo;
use crate::arch::Architecture;

fn parse_cpu_online_count() -> u8 {
    let nr = crate::arch::shim::nr_sched_getaffinity();
    if nr == crate::common::error::ERR_NOT_IMPLEMENTED {
        return 1;
    }
    let mut mask = [0u64; 16];
    let ret = unsafe { crate::sys::raw_syscall(nr, 0, 128, mask.as_mut_ptr() as u64, 0, 0, 0) };
    if ret <= 0 {
        return 1;
    }
    let bytes_returned = ret as usize;
    let words = bytes_returned / 8;
    let mut count: u32 = 0;
    let mut i = 0;
    while i < words && i < 16 {
        count += mask[i].count_ones();
        i += 1;
    }
    if count == 0 {
        1
    } else if count > 255 {
        255
    } else {
        count as u8
    }
}

fn arm_part_to_name(part: u16, buf: &mut [u8; 48]) -> u8 {
    let name: &[u8] = match part {
        0xd03 => b"Cortex-A53",
        0xd04 => b"Cortex-A35",
        0xd05 => b"Cortex-A55",
        0xd06 => b"Cortex-A65",
        0xd07 => b"Cortex-A57",
        0xd08 => b"Cortex-A72",
        0xd09 => b"Cortex-A73",
        0xd0a => b"Cortex-A75",
        0xd0b => b"Cortex-A76",
        0xd0c => b"Neoverse-N1",
        0xd0d => b"Cortex-A77",
        0xd40 => b"Neoverse-V1",
        0xd41 => b"Cortex-A78",
        0xd44 => b"Cortex-X1",
        0xd46 => b"Cortex-A510",
        0xd47 => b"Cortex-A710",
        0xd48 => b"Cortex-X2",
        0xd4d => b"Cortex-A715",
        0xd4e => b"Cortex-X3",
        0xd80 => b"Cortex-A520",
        0xd81 => b"Cortex-A720",
        0xd82 => b"Cortex-X4",
        _ => b"ARM",
    };
    let len = name.len().min(48);
    let mut i = 0;
    while i < len {
        buf[i] = name[i];
        i += 1;
    }
    len as u8
}

pub fn detect_cpu_info() -> Option<CpuInfo> {
    match crate::arch::detect_arch() {
        Architecture::X86_64 => detect_cpu_info_x86(),
        Architecture::AArch64 => detect_cpu_info_aarch64(),
        _ => detect_cpu_info_x86().or_else(detect_cpu_info_aarch64_inner),
    }
}

fn detect_cpu_info_aarch64_inner() -> Option<CpuInfo> {
    detect_cpu_info_aarch64()
}

fn detect_cpu_info_x86() -> Option<CpuInfo> {
    if let Some((eax, ebx, ecx, edx)) = crate::arch::shim::cpuid_count(0, 0) {
        let max_leaf = eax;
        let mut v = [0u8; 12];
        v[0..4].copy_from_slice(&ebx.to_le_bytes());
        v[4..8].copy_from_slice(&edx.to_le_bytes());
        v[8..12].copy_from_slice(&ecx.to_le_bytes());
        let vendor = match &v {
            b"GenuineIntel" => "Intel",
            b"AuthenticAMD" => "AMD",
            _ => "x86_64-unknown",
        };

        let ext_max = crate::arch::shim::cpuid_count(0x80000000, 0)
            .map(|(a, _, _, _)| a)
            .unwrap_or(0);

        let mut model_name = [0u8; 48];
        let model_name_len = if ext_max >= 0x80000004 {
            super::frequency::read_brand_string(&mut model_name)
        } else {
            0
        };

        let (l2_cache_kb, l3_cache_kb) = if ext_max >= 0x80000006 {
            if let Some((_, _, c6, _)) = crate::arch::shim::cpuid_count(0x80000006, 0) {
                let l2 = (c6 >> 16) as u16;
                let l3_512k = if ext_max >= 0x80000006 {
                    if let Some((_, _, _, d6)) = crate::arch::shim::cpuid_count(0x80000006, 0) {
                        ((d6 >> 18) & 0x3FFF) as u16
                    } else {
                        0
                    }
                } else {
                    0
                };
                (l2, l3_512k * 512)
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };

        let l1_cache_kb = {
            let mut l1 = 0u16;
            // AMD: CPUID 0x80000005 has L1 data/instruction cache sizes
            if ext_max >= 0x80000005 {
                if let Some((_, _, c5, d5)) = crate::arch::shim::cpuid_count(0x80000005, 0) {
                    let data = (d5 >> 24) as u16;
                    let inst = (c5 >> 24) as u16;
                    l1 = data + inst;
                }
            }
            // Intel (and fallback): CPUID leaf 4 enumerates deterministic cache params
            if l1 == 0 {
                let mut sub = 0u32;
                while sub < 8 {
                    if let Some((eax4, ebx4, ecx4, _)) = crate::arch::shim::cpuid_count(4, sub) {
                        let cache_type = eax4 & 0x1F;
                        if cache_type == 0 {
                            break;
                        }
                        let level = (eax4 >> 5) & 0x7;
                        if level == 1 {
                            let ways = ((ebx4 >> 22) & 0x3FF) + 1;
                            let partitions = ((ebx4 >> 12) & 0x3FF) + 1;
                            let line_size = (ebx4 & 0xFFF) + 1;
                            let sets = ecx4 + 1;
                            let size = ways * partitions * line_size * sets;
                            l1 += (size / 1024) as u16;
                        }
                    } else {
                        break;
                    }
                    sub += 1;
                }
            }
            l1
        };

        let freq_hz = {
            let sysfs_freq = super::frequency::read_cpu_freq_sysfs(0);
            if sysfs_freq > 0 {
                sysfs_freq
            } else if max_leaf >= 0x16 {
                if let Some((fbase, _, _, _)) = crate::arch::shim::cpuid_count(0x16, 0) {
                    if fbase != 0 {
                        (fbase as u64) * 1_000_000u64
                    } else {
                        super::frequency::estimate_frequency()
                    }
                } else {
                    super::frequency::estimate_frequency()
                }
            } else {
                super::frequency::estimate_frequency()
            }
        };

        let mut physical_cores: u8 = 1;
        let mut logical_cores: u8 = 1;

        if vendor == "AMD" && ext_max >= 0x80000008 {
            if let Some((_, _, c8, _)) = crate::arch::shim::cpuid_count(0x80000008, 0) {
                let nc = (c8 & 0xFF) as u8;
                let total_logical = nc + 1;
                logical_cores = total_logical;

                if ext_max >= 0x8000001E {
                    if let Some((_, b1e, _, _)) = crate::arch::shim::cpuid_count(0x8000001E, 0) {
                        let threads_per_unit = ((b1e >> 8) & 0xFF) as u8 + 1;
                        physical_cores = total_logical / threads_per_unit;
                        if physical_cores == 0 {
                            physical_cores = 1;
                        }
                    } else {
                        physical_cores = total_logical;
                    }
                } else {
                    physical_cores = total_logical;
                }
            } else if let Some((_, ebx1, _, _)) = crate::arch::shim::cpuid_count(1, 0) {
                let lp = ((ebx1 >> 16) & 0xFF) as u8;
                if lp > 0 {
                    logical_cores = lp;
                }
                physical_cores = logical_cores;
            }
        } else if vendor == "Intel" {
            if max_leaf >= 0x0B {
                if let Some((_, ebx_smt, _, _)) = crate::arch::shim::cpuid_count(0x0B, 0) {
                    let smt_count = (ebx_smt & 0xFFFF) as u8;
                    if let Some((_, ebx_core, _, _)) = crate::arch::shim::cpuid_count(0x0B, 1) {
                        let total = (ebx_core & 0xFFFF) as u8;
                        if total > 0 && smt_count > 0 {
                            logical_cores = total;
                            physical_cores = total / smt_count;
                            if physical_cores == 0 {
                                physical_cores = 1;
                            }
                        }
                    }
                }
            } else if max_leaf >= 4 {
                if let Some((eax4, _, _, _)) = crate::arch::shim::cpuid_count(4, 0) {
                    physical_cores = ((eax4 >> 26) & 0x3F) as u8 + 1;
                }
                if let Some((_, ebx1, _, _)) = crate::arch::shim::cpuid_count(1, 0) {
                    logical_cores = ((ebx1 >> 16) & 0xFF) as u8;
                    if logical_cores == 0 {
                        logical_cores = physical_cores;
                    }
                }
            }
        } else if max_leaf >= 0x0B {
            if let Some((_, ebx_b1, _, _)) = crate::arch::shim::cpuid_count(0x0B, 1) {
                let total = (ebx_b1 & 0xFFFF) as u8;
                if total > 0 {
                    logical_cores = total;
                }
            }
            if let Some((_, ebx_b0, _, _)) = crate::arch::shim::cpuid_count(0x0B, 0) {
                let smt = (ebx_b0 & 0xFFFF) as u8;
                if smt > 0 && logical_cores > 0 {
                    physical_cores = logical_cores / smt;
                    if physical_cores == 0 {
                        physical_cores = 1;
                    }
                }
            }
        }

        let os_count = parse_cpu_online_count();
        if os_count > logical_cores {
            let ratio = if logical_cores > 0 && physical_cores > 0 {
                logical_cores / physical_cores
            } else {
                1
            };
            logical_cores = os_count;
            physical_cores = if ratio > 0 {
                os_count / ratio
            } else {
                os_count
            };
            if physical_cores == 0 {
                physical_cores = 1;
            }
        }

        let has_ht = if let Some((_, _, _, edx1)) = crate::arch::shim::cpuid_count(1, 0) {
            (edx1 & (1 << 28)) != 0
        } else {
            false
        };

        let threads_per_core = if physical_cores > 0 {
            logical_cores / physical_cores
        } else {
            1
        };

        return Some(CpuInfo {
            arch: Architecture::X86_64,
            id: if let Some((a1, b1, c1, _)) = crate::arch::shim::cpuid_count(1, 0) {
                ((a1 as u64) << 32) | ((b1 as u64) << 16) | (c1 as u64)
            } else {
                0
            },
            vendor,
            frequency_hz: freq_hz,
            cores: physical_cores,
            physical_cores,
            logical_cores,
            threads_per_core: if threads_per_core == 0 {
                1
            } else {
                threads_per_core
            },
            model_name,
            model_name_len,
            l1_cache_kb,
            l2_cache_kb,
            l3_cache_kb,
            has_ht,
        });
    }
    None
}

fn detect_cpu_info_aarch64() -> Option<CpuInfo> {
    if let Some(midr) = crate::arch::shim::read_aarch64_midr() {
        let implementer = ((midr >> 24) & 0xff) as u8;
        let vendor = match implementer {
            0x41 => "ARM",
            0x51 => "Qualcomm",
            0x43 => "Cavium",
            0x4E => "NVIDIA",
            0x50 => "APM",
            0x56 => "Marvell",
            _ => "aarch64-unknown",
        };
        let core_count = parse_cpu_online_count();
        let part = ((midr >> 4) & 0xfff) as u16;
        let mut model_name = [0u8; 48];
        let model_name_len = arm_part_to_name(part, &mut model_name);

        let (l1_cache_kb, l2_cache_kb, l3_cache_kb) = arm_part_cache_sizes(part);

        let freq_hz = {
            let f = super::frequency::read_cpu_freq_sysfs(0);
            if f > 0 {
                f
            } else {
                0
            }
        };

        let physical_cores = core_count;
        let threads_per_core: u8 = 1;

        return Some(CpuInfo {
            arch: Architecture::AArch64,
            id: midr,
            vendor,
            frequency_hz: freq_hz,
            cores: physical_cores,
            physical_cores,
            logical_cores: core_count,
            threads_per_core,
            model_name,
            model_name_len,
            l1_cache_kb,
            l2_cache_kb,
            l3_cache_kb,
            has_ht: false,
        });
    }

    None
}

/// Known cache sizes (L1 total KB, L2 KB, L3 KB) for common ARM cores.
/// Used as fallback when sysfs cache info is unavailable.
fn arm_part_cache_sizes(part: u16) -> (u16, u16, u16) {
    match part {
        0xd03 => (64, 512, 0), // Cortex-A53: 32K I + 32K D, up to 2MB L2 (typical 512K)
        0xd04 => (64, 256, 0), // Cortex-A35
        0xd05 => (64, 256, 0), // Cortex-A55: 32K I + 32K D, 256K L2
        0xd06 => (64, 256, 0), // Cortex-A65
        0xd07 => (96, 2048, 0), // Cortex-A57: 48K I + 32K D (sometimes listed 80), 2MB L2
        0xd08 => (96, 2048, 0), // Cortex-A72
        0xd09 => (96, 2048, 0), // Cortex-A73
        0xd0a => (128, 512, 0), // Cortex-A75: 64K I + 64K D
        0xd0b => (128, 512, 0), // Cortex-A76: 64K I + 64K D
        0xd0d => (128, 512, 0), // Cortex-A77
        0xd40 => (128, 512, 0), // Neoverse-V1
        0xd41 => (128, 1024, 0), // Cortex-A78
        0xd44 => (128, 512, 0), // Cortex-X1
        0xd46 => (128, 512, 0), // Cortex-A510
        0xd47 => (128, 512, 0), // Cortex-A710
        0xd48 => (128, 1024, 0), // Cortex-X2
        0xd4d => (128, 1024, 0), // Cortex-A715
        0xd4e => (128, 1024, 0), // Cortex-X3
        _ => (64, 256, 0),     // Conservative default
    }
}
