use super::detection::{detect_gpus_impl, GpuDevice};
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::gpu::init_gpu(0, 0, 0) {
                static X86_GPU_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_GPU_SIG.store(
                    ctx.bus as usize
                        ^ ctx.device as usize
                        ^ ctx.function as usize
                        ^ ctx.vendor_id as usize
                        ^ ctx.device_id as usize
                        ^ ctx.bar0_base
                        ^ ctx.bar0_size
                        ^ ctx.msi_vector as usize
                        ^ ctx.vram_iova,
                    Ordering::Release,
                );
            }
            static X86_GPU_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_GPU_DIAG.store(
                crate::arch::x86_64::gpu::diagnostics(0, 0, 0),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,gpu")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 {
                return;
            }
            if let Some(ctx) = crate::arch::aarch64::gpu::init_gpu(dt_base, dt_size, dt_irq) {
                static ARM_GPU_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_GPU_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize
                        ^ ctx.vram_iova,
                    Ordering::Release,
                );
            }
            static ARM_GPU_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_GPU_DIAG.store(
                crate::arch::aarch64::gpu::diagnostics(dt_base),
                Ordering::Release,
            );
        }
        _ => {}
    }

    let mut devices = [GpuDevice {
        bus: 0,
        device: 0,
        function: 0,
        vendor_id: 0,
        device_id: 0,
        class: 0,
        subclass: 0,
        prog_if: 0,
        bar0: 0,
    }; 8];
    let found = detect_gpus_impl(&mut devices);
    if found == 0 {
        return;
    }
    for d in devices.iter().take(found.min(devices.len())) {
        if let Some(mut drv) = crate::gpu::drivers::generic::GpuDriver::probe(d) {
            if drv.init() {
                static GPU_SIG: AtomicUsize = AtomicUsize::new(0);
                let ic = crate::gpu::drivers::generic::GpuDriver::irq_count();
                let fb = drv.allocate_framebuffer(4096, 4096);
                let sq = drv.submit_command_queue(&[&[0u8; 4]]);
                let sc = drv.submit_command(&[0u8; 4]);
                let mc = drv.submit_mmio_command(0x1000, 4);
                let cc = drv.cmd_count();
                GPU_SIG.store(
                    ic ^ fb.unwrap_or(0) ^ sq.unwrap_or(0) ^ sc.unwrap_or(0) ^ cc ^ (mc as usize),
                    Ordering::Release,
                );
            }
        }
    }
}
