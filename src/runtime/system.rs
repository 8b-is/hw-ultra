use super::hal::{
    BootHal, BusHal, DiscoveryHal, DmaHal, FirmwareHal, HardwareAccessHal, InterruptHal, IommuHal,
    MemoryHal, SecurityHal,
};
use super::handle::{self, AccelHandle, AcceleratorKind};
use super::monitor;
use super::platform::{self, Platform};
use super::resources;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static SYSTEM_READY: AtomicBool = AtomicBool::new(false);

pub struct SystemStatus {
    pub arch: crate::arch::Architecture,
    pub gpu_present: bool,
    pub tpu_present: bool,
    pub lpu_present: bool,
    pub dma_allocated: usize,
    pub mem_allocated: usize,
    pub irq_registered: usize,
    pub limits_enforced: bool,
    pub total_ram: usize,
}

pub struct System {
    arch: crate::arch::Architecture,
    platform: Platform,
}

impl System {
    pub fn init() -> Self {
        crate::init::init_shims();
        crate::init::init();

        let arch = crate::arch::detect_arch();

        match arch {
            crate::arch::Architecture::X86_64 => {
                if crate::arch::x86_64::gpu::is_initialized() {
                    handle::register_gpu(
                        crate::arch::x86_64::gpu::gpu_mmio_base(),
                        crate::arch::x86_64::gpu::gpu_mmio_size(),
                        0,
                        0,
                    );
                }
                if crate::arch::x86_64::tpu::is_initialized() {
                    handle::register_tpu(0, 0, 0, 0);
                }
                if crate::arch::x86_64::lpu::is_initialized() {
                    handle::register_lpu(0, 0, 0, 0);
                }
            }
            crate::arch::Architecture::AArch64 => {
                if crate::arch::aarch64::gpu::is_initialized() {
                    let (gb, gs, _) =
                        crate::firmware::devicetree::find_device_by_compatible(b"arm,gpu")
                            .unwrap_or((0, 0, 0));
                    handle::register_gpu(gb, gs, 0, 0);
                }
                if crate::arch::aarch64::tpu::is_initialized() {
                    let (tb, ts, _) =
                        crate::firmware::devicetree::find_device_by_compatible(b"arm,tpu")
                            .unwrap_or((0, 0, 0));
                    handle::register_tpu(tb, ts, 0, 0);
                }
                if crate::arch::aarch64::lpu::is_initialized() {
                    let (lb, ls, _) =
                        crate::firmware::devicetree::find_device_by_compatible(b"arm,lpu")
                            .unwrap_or((0, 0, 0));
                    handle::register_lpu(lb, ls, 0, 0);
                }
            }
            _ => {}
        }

        if handle::gpu_handle().is_none() {
            let mut gpus = [crate::gpu::GpuDevice {
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
            let count = crate::gpu::detect_gpus(&mut gpus);
            if count > 0 {
                let g = &gpus[0];
                let bar_size =
                    crate::bus::pci::api::probe_bar_size(g.bus, g.device, g.function, 0x10)
                        .unwrap_or(0);
                handle::register_gpu(
                    g.bar0 as usize,
                    bar_size,
                    g.vendor_id as usize,
                    g.device_id as usize,
                );
            }
        }

        SYSTEM_READY.store(true, Ordering::Release);

        let platform = platform::detect();

        System { arch, platform }
    }

    pub fn is_ready(&self) -> bool {
        SYSTEM_READY.load(Ordering::Acquire)
    }

    pub fn arch(&self) -> crate::arch::Architecture {
        self.arch
    }

    pub fn gpu(&self) -> Option<AccelHandle> {
        handle::gpu_handle()
    }

    pub fn tpu(&self) -> Option<AccelHandle> {
        handle::tpu_handle()
    }

    pub fn lpu(&self) -> Option<AccelHandle> {
        handle::lpu_handle()
    }

    pub fn status(&self) -> SystemStatus {
        let snap = resources::snapshot();
        SystemStatus {
            arch: self.arch,
            gpu_present: handle::gpu_handle().is_some(),
            tpu_present: handle::tpu_handle().is_some(),
            lpu_present: handle::lpu_handle().is_some(),
            dma_allocated: snap.dma_allocated,
            mem_allocated: snap.mem_allocated,
            irq_registered: snap.irq_registered,
            limits_enforced: snap.limits_enforced,
            total_ram: crate::boot::total_usable_ram(),
        }
    }

    pub fn set_dma_limit(&self, bytes: usize) {
        resources::set_dma_limit(bytes);
    }

    pub fn set_memory_limit(&self, bytes: usize) {
        resources::set_memory_limit(bytes);
    }

    pub fn set_irq_limit(&self, max: usize) {
        resources::set_irq_limit(max);
    }

    pub fn enforce_limits(&self, enable: bool) {
        resources::enforce_limits(enable);
    }

    pub fn try_alloc_dma(&self, bytes: usize) -> bool {
        resources::try_alloc_dma(bytes)
    }

    pub fn free_dma(&self, bytes: usize) {
        resources::free_dma(bytes);
    }

    pub fn try_alloc_memory(&self, bytes: usize) -> bool {
        resources::try_alloc_memory(bytes)
    }

    pub fn free_memory(&self, bytes: usize) {
        resources::free_memory(bytes);
    }

    pub fn try_alloc_swap(&self, bytes: usize) -> bool {
        resources::try_alloc_swap(bytes)
    }

    pub fn free_swap(&self, bytes: usize) {
        resources::free_swap(bytes);
    }

    pub fn resources(&self) -> resources::ResourceSnapshot {
        resources::snapshot()
    }

    pub fn spawn_workers(&self) -> usize {
        super::worker::spawn_workers()
    }

    pub fn stop_workers(&self) {
        super::worker::stop_workers()
    }

    pub fn worker_count(&self) -> usize {
        super::worker::worker_count()
    }

    pub fn workers_running(&self) -> bool {
        super::worker::is_running()
    }

    pub fn worker_snapshot(&self, comp: monitor::Component) -> super::worker::WorkerSnapshot {
        super::worker::snapshot(comp)
    }

    pub fn all_worker_snapshots(&self) -> [super::worker::WorkerSnapshot; 5] {
        super::worker::all_snapshots()
    }

    pub fn shutdown(&self) {
        super::worker::stop_workers();
        resources::reset_counters();
        SYSTEM_READY.store(false, Ordering::Release);
    }

    pub fn accel_count(&self) -> usize {
        let mut n = 0;
        if handle::gpu_handle().is_some() {
            n += 1;
        }
        if handle::tpu_handle().is_some() {
            n += 1;
        }
        if handle::lpu_handle().is_some() {
            n += 1;
        }
        n
    }

    pub fn for_each_accel<F: FnMut(&AccelHandle)>(&self, mut f: F) {
        if let Some(ref g) = handle::gpu_handle() {
            f(g);
        }
        if let Some(ref t) = handle::tpu_handle() {
            f(t);
        }
        if let Some(ref l) = handle::lpu_handle() {
            f(l);
        }
    }

    pub fn accel_by_kind(&self, kind: AcceleratorKind) -> Option<AccelHandle> {
        match kind {
            AcceleratorKind::Gpu => handle::gpu_handle(),
            AcceleratorKind::Tpu => handle::tpu_handle(),
            AcceleratorKind::Lpu => handle::lpu_handle(),
        }
    }

    pub fn read_temperature(&self, comp: monitor::Component) -> u32 {
        monitor::read_temperature(comp)
    }

    pub fn set_temp_limit(&self, comp: monitor::Component, millideg: u32) {
        monitor::set_temp_limit(comp, millideg);
    }

    pub fn is_throttled(&self, comp: monitor::Component) -> bool {
        monitor::is_throttled(comp)
    }

    pub fn set_frequency(&self, comp: monitor::Component, hz: usize) {
        monitor::set_frequency(comp, hz);
    }

    pub fn frequency(&self, comp: monitor::Component) -> usize {
        monitor::frequency(comp)
    }

    pub fn set_freq_bounds(&self, comp: monitor::Component, min_hz: usize, max_hz: usize) {
        monitor::set_freq_bounds(comp, min_hz, max_hz);
    }

    pub fn read_cycles(&self, comp: monitor::Component) -> usize {
        monitor::read_cycles(comp)
    }

    pub fn record_ops(&self, comp: monitor::Component, count: usize) {
        monitor::record_ops(comp, count);
    }

    pub fn read_ops(&self, comp: monitor::Component) -> usize {
        monitor::read_ops(comp)
    }

    pub fn set_precision(&self, comp: monitor::Component, prec: monitor::Precision) {
        monitor::set_precision(comp, prec);
    }

    pub fn precision(&self, comp: monitor::Component) -> monitor::Precision {
        monitor::precision(comp)
    }

    pub fn component_snapshot(&self, comp: monitor::Component) -> monitor::ComponentSnapshot {
        monitor::snapshot(comp)
    }

    pub fn all_snapshots(&self) -> [monitor::ComponentSnapshot; 5] {
        monitor::snapshot_all()
    }

    pub fn platform(&self) -> Platform {
        self.platform
    }

    pub fn is_bare_metal(&self) -> bool {
        self.platform == Platform::BareMetal
    }

    pub fn is_hosted(&self) -> bool {
        self.platform == Platform::Hosted
    }

    pub fn memory(&self) -> MemoryHal {
        MemoryHal
    }

    pub fn interrupts(&self) -> InterruptHal {
        InterruptHal
    }

    pub fn bus(&self) -> BusHal {
        BusHal
    }

    pub fn dma(&self) -> DmaHal {
        DmaHal
    }

    pub fn iommu(&self) -> IommuHal {
        IommuHal
    }

    pub fn firmware(&self) -> FirmwareHal {
        FirmwareHal
    }

    pub fn discovery(&self) -> DiscoveryHal {
        DiscoveryHal
    }

    pub fn boot(&self) -> BootHal {
        BootHal
    }

    pub fn security(&self) -> SecurityHal {
        SecurityHal
    }

    pub fn hardware_access(&self) -> HardwareAccessHal {
        HardwareAccessHal
    }
}

static SYSTEM_SIG: AtomicUsize = AtomicUsize::new(0);

pub fn system_signature() -> usize {
    SYSTEM_SIG.load(Ordering::Acquire)
}

pub fn set_system_signature(sig: usize) {
    SYSTEM_SIG.store(sig, Ordering::Release);
}
