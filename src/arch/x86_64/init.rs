use crate::arch::shim::set_cpuid_fn;

pub fn init_shim() {
    set_cpuid_fn(|eax, ecx| Some(crate::arch::x86_64::cpuid::cpuid_count(eax, ecx)));
    crate::arch::x86_64::cpu::tsc::set_tsc_mmio(0);
    crate::arch::x86_64::cpuid::set_cpuid_mmio(0);
    crate::arch::x86_64::msr::set_msr_mmio_base(0);
}
