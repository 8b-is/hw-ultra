use super::detection::{detect, SensorDevice, SensorKind};
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::sensor::init_sensor(0, 0, 0) {
                static X86_SENSOR_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_SENSOR_SIG.store(
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
            static X86_SENSOR_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_SENSOR_DIAG.store(
                crate::arch::x86_64::sensor::diagnostics(0, 0, 0),
                Ordering::Release,
            );
            static X86_SENSOR_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::x86_64::sensor::write_sensor_reg(0, 0);
            X86_SENSOR_API.store(
                crate::arch::x86_64::sensor::is_initialized() as usize
                    ^ crate::arch::x86_64::sensor::read_sensor_reg(0) as usize
                    ^ crate::arch::x86_64::sensor::sensor_mmio_base()
                    ^ crate::arch::x86_64::sensor::sensor_mmio_size(),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,sensor")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 {
                return;
            }
            if let Some(ctx) = crate::arch::aarch64::sensor::init_sensor(dt_base, dt_size, dt_irq) {
                static ARM_SENSOR_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_SENSOR_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize,
                    Ordering::Release,
                );
            }
            static ARM_SENSOR_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_SENSOR_DIAG.store(
                crate::arch::aarch64::sensor::diagnostics(dt_base),
                Ordering::Release,
            );
            static ARM_SENSOR_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::aarch64::sensor::write_sensor_reg(0, 0);
            ARM_SENSOR_API.store(
                crate::arch::aarch64::sensor::is_initialized() as usize
                    ^ crate::arch::aarch64::sensor::read_sensor_reg(0) as usize,
                Ordering::Release,
            );
        }
        _ => {}
    }

    let mut devices = [SensorDevice {
        kind: SensorKind::Unknown,
        reg_base: 0,
        irq: 0,
        compat: [0u8; 64],
        compat_len: 0,
    }; 16];
    let found = detect(&mut devices);
    static SENSOR_COUNT: AtomicUsize = AtomicUsize::new(0);
    SENSOR_COUNT.store(found, Ordering::Release);
    let mut i = 0;
    while i < found {
        static SENSOR_SIG: AtomicUsize = AtomicUsize::new(0);
        SENSOR_SIG.store(
            devices[i].reg_base as usize ^ devices[i].irq as usize ^ devices[i].compat_len,
            Ordering::Release,
        );
        i += 1;
    }
}
