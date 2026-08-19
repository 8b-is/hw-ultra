use super::detection::{detect, CameraDevice, CameraInterface};
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::camera::init_camera(0, 0, 0) {
                static X86_CAMERA_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_CAMERA_SIG.store(
                    ctx.bus as usize
                        ^ ctx.device as usize
                        ^ ctx.function as usize
                        ^ ctx.vendor_id as usize
                        ^ ctx.device_id as usize
                        ^ ctx.bar0_base
                        ^ ctx.bar0_size
                        ^ ctx.msi_vector as usize,
                    Ordering::Release,
                );
            }
            static X86_CAMERA_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_CAMERA_DIAG.store(
                crate::arch::x86_64::camera::diagnostics(0, 0, 0),
                Ordering::Release,
            );
            static X86_CAMERA_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::x86_64::camera::write_camera_reg(0, 0);
            X86_CAMERA_API.store(
                crate::arch::x86_64::camera::is_initialized() as usize
                    ^ crate::arch::x86_64::camera::read_camera_reg(0) as usize
                    ^ crate::arch::x86_64::camera::camera_mmio_base()
                    ^ crate::arch::x86_64::camera::camera_mmio_size(),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,camera")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 {
                return;
            }
            if let Some(ctx) = crate::arch::aarch64::camera::init_camera(dt_base, dt_size, dt_irq) {
                static ARM_CAMERA_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_CAMERA_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize
                        ^ ctx.dma_region,
                    Ordering::Release,
                );
            }
            static ARM_CAMERA_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_CAMERA_DIAG.store(
                crate::arch::aarch64::camera::diagnostics(dt_base),
                Ordering::Release,
            );
            static ARM_CAMERA_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::aarch64::camera::write_camera_reg(0, 0);
            ARM_CAMERA_API.store(
                crate::arch::aarch64::camera::is_initialized() as usize
                    ^ crate::arch::aarch64::camera::read_camera_reg(0) as usize,
                Ordering::Release,
            );
        }
        _ => {}
    }

    let mut devices = [CameraDevice {
        interface: CameraInterface::Platform,
        reg_base: 0,
        reg_size: 0,
        irq: 0,
        compat: [0u8; 64],
        compat_len: 0,
    }; 8];
    let found = detect(&mut devices);
    static CAMERA_COUNT: AtomicUsize = AtomicUsize::new(0);
    CAMERA_COUNT.store(found, Ordering::Release);
    let mut i = 0;
    while i < found {
        static CAMERA_SIG: AtomicUsize = AtomicUsize::new(0);
        CAMERA_SIG.store(
            devices[i].reg_base as usize ^ devices[i].irq as usize ^ devices[i].compat_len,
            Ordering::Release,
        );
        i += 1;
    }
}
