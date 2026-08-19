use crate::arch::shim::{
    set_mmio_read32_fn, set_mmio_write32_fn, set_read_aarch64_midr_fn, set_read_msr_fn,
};

pub fn init_shim() {
    set_read_msr_fn(|msr| {
        static READ_MSR_SHIM_SIG: core::sync::atomic::AtomicUsize =
            core::sync::atomic::AtomicUsize::new(0);
        READ_MSR_SHIM_SIG.store(msr as usize, core::sync::atomic::Ordering::Release);
        None
    });

    set_mmio_read32_fn(|addr| unsafe { Some(crate::arch::aarch64::mmio::mmio_read32(addr)) });
    set_mmio_write32_fn(|addr, val| unsafe {
        crate::arch::aarch64::mmio::mmio_write32(addr, val);
        true
    });

    set_read_aarch64_midr_fn(|| {
        let raw = unsafe { crate::arch::aarch64::sysreg::read_midr_el1() };
        if raw != 0 {
            return Some(raw);
        }
        None
    });

    crate::arch::aarch64::cpu::registers::set_cpu_reg_base(0);
    crate::arch::aarch64::cpu::features::set_feature_reg_base(0);
    crate::arch::aarch64::cpu::system_regs::set_sysreg_base(0);
    crate::arch::aarch64::interrupt::gic::set_gic_cpu_base(0);
    crate::arch::aarch64::simd::detect::set_sve_vl_mmio(0);
    crate::arch::aarch64::virtualization::hyp::set_hyp_reg_base(0);
    crate::arch::aarch64::sysreg::set_midr_mmio(0);
}
