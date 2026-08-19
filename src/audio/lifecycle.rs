use super::detection::{detect, AudioCodec, AudioController};
use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            if let Some(ctx) = crate::arch::x86_64::audio::init_audio(0, 0, 0) {
                static X86_AUDIO_SIG: AtomicUsize = AtomicUsize::new(0);
                X86_AUDIO_SIG.store(
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
            static X86_AUDIO_DIAG: AtomicUsize = AtomicUsize::new(0);
            X86_AUDIO_DIAG.store(
                crate::arch::x86_64::audio::diagnostics(0, 0, 0),
                Ordering::Release,
            );
            static X86_AUDIO_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::x86_64::audio::write_audio_reg(0, 0);
            X86_AUDIO_API.store(
                crate::arch::x86_64::audio::is_initialized() as usize
                    ^ crate::arch::x86_64::audio::read_audio_reg(0) as usize
                    ^ crate::arch::x86_64::audio::audio_mmio_base()
                    ^ crate::arch::x86_64::audio::audio_mmio_size(),
                Ordering::Release,
            );
        }
        crate::arch::Architecture::AArch64 => {
            let (dt_base, dt_size, dt_irq) =
                crate::firmware::devicetree::find_device_by_compatible(b"arm,audio")
                    .unwrap_or((0, 0, 0));
            if dt_base == 0 {
                return;
            }
            if let Some(ctx) = crate::arch::aarch64::audio::init_audio(dt_base, dt_size, dt_irq) {
                static ARM_AUDIO_SIG: AtomicUsize = AtomicUsize::new(0);
                ARM_AUDIO_SIG.store(
                    ctx.mmio_base
                        ^ ctx.mmio_size
                        ^ ctx.device_id as usize
                        ^ ctx.spi_id as usize
                        ^ ctx.smmu_stream_id as usize
                        ^ ctx.dma_region,
                    Ordering::Release,
                );
            }
            static ARM_AUDIO_DIAG: AtomicUsize = AtomicUsize::new(0);
            ARM_AUDIO_DIAG.store(
                crate::arch::aarch64::audio::diagnostics(dt_base),
                Ordering::Release,
            );
            static ARM_AUDIO_API: AtomicUsize = AtomicUsize::new(0);
            crate::arch::aarch64::audio::write_audio_reg(0, 0);
            ARM_AUDIO_API.store(
                crate::arch::aarch64::audio::is_initialized() as usize
                    ^ crate::arch::aarch64::audio::read_audio_reg(0) as usize,
                Ordering::Release,
            );
        }
        _ => {}
    }

    let mut devices = [AudioController {
        vendor_id: 0,
        device_id: 0,
        codec: AudioCodec::Unknown,
        hda_bar: 0,
        output_streams: 0,
        input_streams: 0,
        bus: 0,
        dev: 0,
        func: 0,
        pci: false,
        reg_base: 0,
    }; 8];
    let found = detect(&mut devices);
    static AUDIO_COUNT: AtomicUsize = AtomicUsize::new(0);
    AUDIO_COUNT.store(found, Ordering::Release);
    let mut i = 0;
    while i < found {
        static AUDIO_SIG: AtomicUsize = AtomicUsize::new(0);
        AUDIO_SIG.store(
            devices[i].vendor_id as usize
                ^ devices[i].device_id as usize
                ^ devices[i].hda_bar as usize
                ^ devices[i].reg_base as usize,
            Ordering::Release,
        );
        i += 1;
    }
}
