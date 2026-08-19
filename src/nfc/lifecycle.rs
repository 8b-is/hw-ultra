use super::detection::{detect, NfcChip, NfcController};
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::nfc::init_nfc(0, 0, 0) {
                static X86_NFC_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_NFC_SIG.store(
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
            static X86_NFC_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_NFC_DIAG.store(
                crate::arch::x86_64::nfc::diagnostics(0, 0, 0),
                Ordering::Release,
            );
            static X86_NFC_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::x86_64::nfc::write_nfc_reg(0, 0);
            X86_NFC_API.store(
                crate::arch::x86_64::nfc::is_initialized() as usize
                    ^ crate::arch::x86_64::nfc::read_nfc_reg(0) as usize
                    ^ crate::arch::x86_64::nfc::nfc_mmio_base()
                    ^ crate::arch::x86_64::nfc::nfc_mmio_size(),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,nfc")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 {
                return;
            }
            if let Some(ctx) = crate::arch::aarch64::nfc::init_nfc(dt_base, dt_size, dt_irq) {
                static ARM_NFC_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_NFC_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize,
                    Ordering::Release,
                );
            }
            static ARM_NFC_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_NFC_DIAG.store(
                crate::arch::aarch64::nfc::diagnostics(dt_base),
                Ordering::Release,
            );
            static ARM_NFC_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::aarch64::nfc::write_nfc_reg(0, 0);
            ARM_NFC_API.store(
                crate::arch::aarch64::nfc::is_initialized() as usize
                    ^ crate::arch::aarch64::nfc::read_nfc_reg(0) as usize,
                Ordering::Release,
            );
        }
        _ => {}
    }

    let mut devices = [NfcController {
        chip: NfcChip::Unknown,
        reg_base: 0,
        irq: 0,
        compat: [0u8; 64],
        compat_len: 0,
    }; 4];
    let found = detect(&mut devices);
    static NFC_COUNT: AtomicUsize = AtomicUsize::new(0);
    NFC_COUNT.store(found, Ordering::Release);
    let mut i = 0;
    while i < found {
        static NFC_SIG: AtomicUsize = AtomicUsize::new(0);
        NFC_SIG.store(
            devices[i].reg_base as usize ^ devices[i].irq as usize ^ devices[i].compat_len,
            Ordering::Release,
        );
        i += 1;
    }
}
