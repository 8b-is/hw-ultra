use super::runtime::Runtime;
use crate::common::once::Once;
use core::sync::atomic::{AtomicUsize, Ordering};

static TPU_RUNTIME: Once<Runtime> = Once::new();

pub fn init(base: usize) -> Result<(), &'static str> {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::tpu::init_tpu(0, 0, 0) {
                static X86_TPU_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_TPU_SIG.store(
                    ctx.bus as usize
                        ^ ctx.device as usize
                        ^ ctx.function as usize
                        ^ ctx.vendor_id as usize
                        ^ ctx.device_id as usize
                        ^ ctx.bar0_base
                        ^ ctx.bar0_size
                        ^ ctx.msi_vector as usize
                        ^ ctx.dma_ring_iova,
                    Ordering::Release,
                );
            }
            static X86_TPU_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_TPU_DIAG.store(
                crate::arch::x86_64::tpu::diagnostics(0, 0, 0),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,tpu")
                    .unwrap_or((0, 0, 0));
            let aarch64_base = if dt_base != 0 { dt_base } else { base };
            let aarch64_size = if dt_base != 0 { dt_size } else { 0 };
            let aarch64_irq = if dt_base != 0 { dt_irq } else { 0 };
            if let Some(ctx) =
                crate::arch::aarch64::tpu::init_tpu(aarch64_base, aarch64_size, aarch64_irq)
            {
                static ARM_TPU_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_TPU_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize
                        ^ ctx.dma_region,
                    Ordering::Release,
                );
            }
            static ARM_TPU_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_TPU_DIAG.store(
                crate::arch::aarch64::tpu::diagnostics(aarch64_base),
                Ordering::Release,
            );
        }
        _ => {}
    }

    crate::tpu::device::init_with_base(base);
    let rt = crate::tpu::runtime::init();
    if TPU_RUNTIME.set(rt) {
        return Ok(());
    }
    Err("runtime already initialized")
}

pub fn runtime() -> Option<&'static Runtime> {
    TPU_RUNTIME.get()
}
