use super::detection::{detect, DisplayController};
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::display::init_display(0, 0, 0) {
                static X86_DISPLAY_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_DISPLAY_SIG.store(
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
            static X86_DISPLAY_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_DISPLAY_DIAG.store(
                crate::arch::x86_64::display::diagnostics(0, 0, 0),
                Ordering::Release,
            );
            static X86_DISPLAY_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::x86_64::display::write_display_reg(0, 0);
            X86_DISPLAY_API.store(
                crate::arch::x86_64::display::is_initialized() as usize
                    ^ crate::arch::x86_64::display::read_display_reg(0) as usize
                    ^ crate::arch::x86_64::display::display_mmio_base()
                    ^ crate::arch::x86_64::display::display_mmio_size(),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,display")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 {
                return;
            }
            if let Some(ctx) = crate::arch::aarch64::display::init_display(dt_base, dt_size, dt_irq)
            {
                static ARM_DISPLAY_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_DISPLAY_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize
                        ^ ctx.fb_iova,
                    Ordering::Release,
                );
            }
            static ARM_DISPLAY_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_DISPLAY_DIAG.store(
                crate::arch::aarch64::display::diagnostics(dt_base),
                Ordering::Release,
            );
            static ARM_DISPLAY_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::aarch64::display::write_display_reg(0, 0);
            ARM_DISPLAY_API.store(
                crate::arch::aarch64::display::is_initialized() as usize
                    ^ crate::arch::aarch64::display::read_display_reg(0) as usize,
                Ordering::Release,
            );
        }
        _ => {}
    }

    let mut devices = [DisplayController {
        vendor_id: 0,
        device_id: 0,
        display_type: super::detection::DisplayType::Unknown,
        framebuffer_bar: 0,
        bus: 0,
        dev: 0,
        func: 0,
        pci: false,
        reg_base: 0,
        reg_size: 0,
    }; 8];
    let found = detect(&mut devices);
    static DISPLAY_COUNT: AtomicUsize = AtomicUsize::new(0);
    DISPLAY_COUNT.store(found, Ordering::Release);
    let mut i = 0;
    while i < found {
        static DISPLAY_SIG: AtomicUsize = AtomicUsize::new(0);
        DISPLAY_SIG.store(
            devices[i].vendor_id as usize
                ^ devices[i].device_id as usize
                ^ devices[i].framebuffer_bar as usize
                ^ devices[i].reg_base as usize,
            Ordering::Release,
        );
        i += 1;
    }
}
