use super::detection::{detect, ModemDevice, ModemGeneration, ModemInterface};
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::modem::init_modem(0, 0, 0) {
                static X86_MODEM_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_MODEM_SIG.store(
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
            static X86_MODEM_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_MODEM_DIAG.store(
                crate::arch::x86_64::modem::diagnostics(0, 0, 0),
                Ordering::Release,
            );
            static X86_MODEM_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::x86_64::modem::write_modem_reg(0, 0);
            X86_MODEM_API.store(
                crate::arch::x86_64::modem::is_initialized() as usize
                    ^ crate::arch::x86_64::modem::read_modem_reg(0) as usize
                    ^ crate::arch::x86_64::modem::modem_mmio_base()
                    ^ crate::arch::x86_64::modem::modem_mmio_size(),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,modem")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 {
                return;
            }
            if let Some(ctx) = crate::arch::aarch64::modem::init_modem(dt_base, dt_size, dt_irq) {
                static ARM_MODEM_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_MODEM_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize
                        ^ ctx.shared_mem,
                    Ordering::Release,
                );
            }
            static ARM_MODEM_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_MODEM_DIAG.store(
                crate::arch::aarch64::modem::diagnostics(dt_base),
                Ordering::Release,
            );
            static ARM_MODEM_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::aarch64::modem::write_modem_reg(0, 0);
            ARM_MODEM_API.store(
                crate::arch::aarch64::modem::is_initialized() as usize
                    ^ crate::arch::aarch64::modem::read_modem_reg(0) as usize,
                Ordering::Release,
            );
        }
        _ => {}
    }

    let mut devices = [ModemDevice {
        generation: ModemGeneration::Unknown,
        interface: ModemInterface::Platform,
        vendor_id: 0,
        device_id: 0,
        reg_base: 0,
        compat: [0u8; 64],
        compat_len: 0,
    }; 4];
    let found = detect(&mut devices);
    static MODEM_COUNT: AtomicUsize = AtomicUsize::new(0);
    MODEM_COUNT.store(found, Ordering::Release);
    let mut i = 0;
    while i < found {
        static MODEM_SIG: AtomicUsize = AtomicUsize::new(0);
        MODEM_SIG.store(
            devices[i].vendor_id as usize
                ^ devices[i].device_id as usize
                ^ devices[i].reg_base as usize
                ^ devices[i].compat_len,
            Ordering::Release,
        );
        i += 1;
    }
}
