#[derive(Copy, Clone)]
pub struct Topology {
    pub sockets: u8,
    pub cores_per_socket: u8,
}

pub fn detect_topology() -> Topology {
    let sockets = detect_sockets();

    // On AArch64, use cpu detect info directly (CPUID doesn't exist)
    if crate::arch::detect_arch() == crate::arch::Architecture::AArch64 {
        if let Some(info) = crate::cpu::api::detect_cpu_info() {
            let cps = if sockets > 0 {
                info.physical_cores / sockets
            } else {
                info.physical_cores
            };
            return Topology {
                sockets: if sockets == 0 { 1 } else { sockets },
                cores_per_socket: if cps == 0 { 1 } else { cps },
            };
        }
        return Topology {
            sockets: if sockets == 0 { 1 } else { sockets },
            cores_per_socket: 1,
        };
    }

    if let Some((max, ebx, _, _)) = crate::hardware_access::read_cpuid(0, 0) {
        let mut v = [0u8; 12];
        v[0..4].copy_from_slice(&ebx.to_le_bytes());
        if let Some((_, _, ecx0, edx0)) = crate::hardware_access::read_cpuid(0, 0) {
            v[4..8].copy_from_slice(&edx0.to_le_bytes());
            v[8..12].copy_from_slice(&ecx0.to_le_bytes());
        }
        let is_amd = &v == b"AuthenticAMD";

        if is_amd {
            let ext_max = crate::hardware_access::read_cpuid(0x80000000, 0)
                .map(|(a, _, _, _)| a)
                .unwrap_or(0);
            if ext_max >= 0x80000008 {
                if let Some((_, _, c8, _)) = crate::hardware_access::read_cpuid(0x80000008, 0) {
                    let total = (c8 & 0xFF) as u8 + 1;
                    let threads_per_core = if ext_max >= 0x8000001E {
                        crate::hardware_access::read_cpuid(0x8000001E, 0)
                            .map(|(_, b1e, _, _)| ((b1e >> 8) & 0xFF) as u8 + 1)
                            .unwrap_or(1)
                    } else {
                        1
                    };
                    let phys = total / threads_per_core;
                    return Topology {
                        sockets: if sockets == 0 { 1 } else { sockets },
                        cores_per_socket: if phys == 0 { 1 } else { phys },
                    };
                }
            }
        }

        if max >= 0x0B {
            if let Some((_, ebx_core, _, _)) = crate::hardware_access::read_cpuid(0x0B, 1) {
                let total = (ebx_core & 0xFFFF) as u8;
                if let Some((_, ebx_smt, _, _)) = crate::hardware_access::read_cpuid(0x0B, 0) {
                    let smt = (ebx_smt & 0xFFFF) as u8;
                    if smt > 0 && total > 0 {
                        let phys = total / smt;
                        return Topology {
                            sockets: if sockets == 0 { 1 } else { sockets },
                            cores_per_socket: if phys == 0 { 1 } else { phys },
                        };
                    }
                }
                if total > 0 {
                    return Topology {
                        sockets: if sockets == 0 { 1 } else { sockets },
                        cores_per_socket: total,
                    };
                }
            }
        }

        if max >= 4 {
            if let Some((eax4, _, _, _)) = crate::hardware_access::read_cpuid(4, 0) {
                let cores = ((eax4 >> 26) & 0x3f) as u8 + 1;
                return Topology {
                    sockets: if sockets == 0 { 1 } else { sockets },
                    cores_per_socket: cores,
                };
            }
        }
    }
    Topology {
        sockets: if sockets == 0 { 1 } else { sockets },
        cores_per_socket: 1,
    }
}

fn detect_sockets() -> u8 {
    1
}
