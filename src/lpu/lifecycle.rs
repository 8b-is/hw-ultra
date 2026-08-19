use super::inference::Inference;
use super::scheduler::LpuScheduler;
use crate::common::once::Once;
use core::sync::atomic::{AtomicUsize, Ordering};

static LPU_INFERENCE: Once<Inference> = Once::new();
static LPU_SCHEDULER: Once<LpuScheduler> = Once::new();

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::lpu::init_lpu(0, 0, 0) {
                static X86_LPU_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_LPU_SIG.store(
                    ctx.bus as usize
                        ^ ctx.device as usize
                        ^ ctx.function as usize
                        ^ ctx.vendor_id as usize
                        ^ ctx.device_id as usize
                        ^ ctx.bar0_base
                        ^ ctx.bar0_size
                        ^ ctx.msi_vector as usize
                        ^ ctx.coherent_dma_base,
                    Ordering::Release,
                );
            }
            static X86_LPU_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_LPU_DIAG.store(
                crate::arch::x86_64::lpu::diagnostics(0, 0, 0),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,lpu")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 { /* no LPU in device tree */
            } else if let Some(ctx) = crate::arch::aarch64::lpu::init_lpu(dt_base, dt_size, dt_irq)
            {
                static ARM_LPU_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_LPU_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize
                        ^ ctx.dma_region,
                    Ordering::Release,
                );
            }
            static ARM_LPU_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_LPU_DIAG.store(
                crate::arch::aarch64::lpu::diagnostics(dt_base),
                Ordering::Release,
            );
        }
        _ => {}
    }

    crate::lpu::device::init();
    if LPU_INFERENCE.get().is_none() {
        let ok_infer = LPU_INFERENCE.set(Inference::new());
        if !ok_infer {}
    }
    if LPU_SCHEDULER.get().is_none() {
        let ok_sched = LPU_SCHEDULER.set(LpuScheduler::new());
        if !ok_sched {}
    }

    let base = crate::firmware::devicetree::find_device_by_compatible(b"arm,lpu")
        .map(|(b, _, _)| b)
        .unwrap_or(0);
    if let Some(accel) = super::drivers::generic::Accelerator::probe(base) {
        accel.init();
        accel.submit_workload(0x5000, 16);
        let ai = accel.is_initialized();
        let ver = accel.read_version();
        static LPU_ACCEL_SIG: AtomicUsize = AtomicUsize::new(0);
        LPU_ACCEL_SIG.store(
            accel.workload_count() ^ (ai as usize) ^ (ver as usize),
            Ordering::Release,
        );
    }

    // Probe vendor-specific LPU drivers
    if let Some(apple) = super::drivers::apple::AppleLpu::probe(base) {
        apple.init();
        apple.submit_tokens(0x6000, 8);
        static APPLE_SIG: AtomicUsize = AtomicUsize::new(0);
        APPLE_SIG.store(
            apple.inference_count() ^ (apple.is_initialized() as usize),
            Ordering::Release,
        );
    }

    if let Some(edge) = super::drivers::edge::EdgeLpu::probe(base) {
        edge.init();
        edge.run_inference(0x7000, 0x8000, 0x9000);
        static EDGE_SIG: AtomicUsize = AtomicUsize::new(0);
        EDGE_SIG.store(
            edge.inference_count() ^ (edge.is_initialized() as usize),
            Ordering::Release,
        );
    }

    if let Some(qc) = super::drivers::qualcomm::Qualcomm::probe(base) {
        qc.init();
        qc.submit_workload(0xA000, 4);
        let ver = qc.read_version();
        static QC_SIG: AtomicUsize = AtomicUsize::new(0);
        QC_SIG.store(
            qc.workload_count() ^ (qc.is_initialized() as usize) ^ (ver as usize),
            Ordering::Release,
        );
    }
}

pub fn inference() -> Option<&'static Inference> {
    LPU_INFERENCE.get()
}

pub fn scheduler() -> Option<&'static LpuScheduler> {
    LPU_SCHEDULER.get()
}
