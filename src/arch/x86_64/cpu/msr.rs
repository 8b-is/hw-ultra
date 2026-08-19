pub fn read_apic_base() -> u64 {
    unsafe { crate::arch::x86_64::msr::read_msr(0x1B) }
}

pub fn read_tsc_aux() -> u32 {
    unsafe { crate::arch::x86_64::msr::read_msr(0xC0000103) as u32 }
}

pub fn read_efer() -> u64 {
    unsafe { crate::arch::x86_64::msr::read_msr(0xC0000080) }
}

pub fn write_efer(val: u64) {
    unsafe { crate::arch::x86_64::msr::write_msr(0xC0000080, val) }
}

pub fn read_star() -> u64 {
    unsafe { crate::arch::x86_64::msr::read_msr(0xC0000081) }
}

pub fn read_lstar() -> u64 {
    unsafe { crate::arch::x86_64::msr::read_msr(0xC0000082) }
}

pub fn write_lstar(val: u64) {
    unsafe { crate::arch::x86_64::msr::write_msr(0xC0000082, val) }
}

pub fn read_pat() -> u64 {
    unsafe { crate::arch::x86_64::msr::read_msr(0x277) }
}
