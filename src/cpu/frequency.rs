pub(super) fn read_brand_string(out: &mut [u8; 48]) -> u8 {
    let mut pos = 0u8;
    for leaf in [0x80000002u32, 0x80000003, 0x80000004] {
        if let Some((a, b, c, d)) = crate::arch::shim::cpuid_count(leaf, 0) {
            for val in [a, b, c, d] {
                let bytes = val.to_le_bytes();
                for byte in bytes {
                    if (pos as usize) < 48 {
                        out[pos as usize] = byte;
                        pos += 1;
                    }
                }
            }
        }
    }
    while pos > 0 && (out[(pos - 1) as usize] == 0 || out[(pos - 1) as usize] == b' ') {
        pos -= 1;
    }
    pos
}

pub fn estimate_frequency() -> u64 {
    if let Some((denom, numer, crystal, _)) = crate::arch::shim::cpuid_count(0x15, 0) {
        if denom != 0 && numer != 0 {
            let crystal_hz = if crystal != 0 {
                crystal as u64
            } else {
                infer_crystal_hz()
            };
            let tsc_hz = crystal_hz * (numer as u64) / (denom as u64);
            if tsc_hz > 100_000_000 {
                return tsc_hz;
            }
        }
    }

    if let Some(freq) = parse_frequency_from_brand() {
        return freq;
    }

    calibrate_tsc()
}

/// Infer the crystal clock frequency from the CPU family/model when
/// CPUID leaf 0x15 ECX (crystal frequency) is zero.
/// Values from Intel SDM Vol. 3, Table 18-85.
fn infer_crystal_hz() -> u64 {
    if let Some((eax, _, _, _)) = crate::arch::shim::cpuid_count(0x01, 0) {
        let family = ((eax >> 8) & 0xF) as u8;
        let model = (((eax >> 16) & 0xF) << 4 | ((eax >> 4) & 0xF)) as u8;
        if family == 6 {
            match model {
                // Atom Goldmont / Denverton
                0x5C | 0x5F => return 19_200_000,
                // Skylake / Kaby Lake / Coffee Lake / Comet Lake etc.
                0x4E | 0x5E | 0x8E | 0x9E | 0xA5 | 0xA6 => return 24_000_000,
                // Ice Lake / Tiger Lake / Alder Lake / Raptor Lake
                0x7E | 0x7D | 0x8C | 0x8D | 0x97 | 0x9A | 0xB7 | 0xBA | 0xBF => return 24_000_000,
                _ => {}
            }
        }
    }
    // Last resort: use brand string frequency or TSC calibration
    // Return 0 so the caller skips the crystal path
    0
}

fn parse_frequency_from_brand() -> Option<u64> {
    let mut brand = [0u8; 48];
    let len = read_brand_string(&mut brand);
    if len == 0 {
        return None;
    }
    let s = &brand[..len as usize];

    let mut i = 0usize;
    while i < s.len() {
        if s[i] == b'@' {
            let mut j = i + 1;
            while j < s.len() && s[j] == b' ' {
                j += 1;
            }
            let start = j;
            let mut integer_part: u64 = 0;
            let mut frac_part: u64 = 0;
            let mut frac_digits: u32 = 0;
            let mut in_frac = false;
            while j < s.len() && (s[j].is_ascii_digit() || s[j] == b'.') {
                if s[j] == b'.' {
                    in_frac = true;
                } else if in_frac {
                    frac_part = frac_part * 10 + (s[j] - b'0') as u64;
                    frac_digits += 1;
                } else {
                    integer_part = integer_part * 10 + (s[j] - b'0') as u64;
                }
                j += 1;
            }
            if j == start {
                i += 1;
                continue;
            }
            let mut multiplier: u64 = 1_000_000;
            if j + 2 < s.len() && s[j] == b'G' && s[j + 1] == b'H' && s[j + 2] == b'z' {
                multiplier = 1_000_000_000;
            } else if j + 2 < s.len() && s[j] == b'M' && s[j + 1] == b'H' && s[j + 2] == b'z' {
                multiplier = 1_000_000;
            }
            let mut hz = integer_part * multiplier;
            if frac_digits > 0 {
                let mut div = 1u64;
                let mut d = 0u32;
                while d < frac_digits {
                    div *= 10;
                    d += 1;
                }
                hz += frac_part * multiplier / div;
            }
            if hz > 100_000_000 {
                return Some(hz);
            }
        }
        i += 1;
    }
    None
}

pub fn calibrate_tsc() -> u64 {
    // Use PIT channel 0 to measure TSC ticks over a known duration.
    // Set PIT to a known divisor, read TSC before and after PIT countdown.
    let divisor: u16 = 11932; // ~100Hz => ~10ms per tick (PIT_FREQ=1193182)
    let pit_hz: u64 = 1_193_182;
    unsafe {
        // Latch channel 0, mode 0 (interrupt on terminal count), binary
        crate::arch::x86_64::io::outb(0x43, 0x30);
        crate::arch::x86_64::io::outb(0x40, (divisor & 0xFF) as u8);
        crate::arch::x86_64::io::outb(0x40, ((divisor >> 8) & 0xFF) as u8);
    }
    let tsc_start = crate::arch::x86_64::cpu::tsc::read_tsc();
    // Wait for PIT countdown by polling the count
    let mut prev = divisor;
    let mut spins = 0u32;
    loop {
        let count = crate::timer::pit::Pit::read_count();
        if count > prev && spins > 100 {
            break; // Counter wrapped — countdown complete
        }
        prev = count;
        spins += 1;
        if spins > 1_000_000 {
            break; // Safety: don't spin forever
        }
    }
    let tsc_end = crate::arch::x86_64::cpu::tsc::read_tsc();
    let tsc_delta = tsc_end.saturating_sub(tsc_start);
    if tsc_delta == 0 {
        return 0;
    }
    // TSC frequency = tsc_delta * (pit_hz / divisor)
    // i.e. tsc_delta ticks happened in (divisor / pit_hz) seconds
    tsc_delta * pit_hz / (divisor as u64)
}

pub fn read_cpu_freq_sysfs(cpu_n: u8) -> u64 {
    let _ = cpu_n;
    0
}
