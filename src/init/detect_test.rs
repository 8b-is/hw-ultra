use core::sync::atomic::{AtomicUsize, Ordering};

static DETECT_SIGNATURE: AtomicUsize = AtomicUsize::new(0);

pub struct DetectedComponent {
    pub name: &'static str,
    pub found: bool,
    pub count: usize,
    pub base_or_info: usize,
}

pub struct DetectionReport {
    pub components: [DetectedComponent; 12],
    pub total_found: usize,
    pub signature: usize,
}

pub fn detect_all_components() -> DetectionReport {
    let mut report = DetectionReport {
        components: [
            DetectedComponent {
                name: "cpu",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "gpu",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "tpu",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "lpu",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "iommu",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "dma",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "interrupt",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "timer",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "pci_bus",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "topology",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "thermal",
                found: false,
                count: 0,
                base_or_info: 0,
            },
            DetectedComponent {
                name: "power",
                found: false,
                count: 0,
                base_or_info: 0,
            },
        ],
        total_found: 0,
        signature: 0,
    };

    detect_cpu(&mut report.components[0]);
    detect_gpu(&mut report.components[1]);
    detect_tpu(&mut report.components[2]);
    detect_lpu(&mut report.components[3]);
    detect_iommu(&mut report.components[4]);
    detect_dma(&mut report.components[5]);
    detect_interrupt(&mut report.components[6]);
    detect_timer(&mut report.components[7]);
    detect_pci_bus(&mut report.components[8]);
    detect_topology(&mut report.components[9]);
    detect_thermal(&mut report.components[10]);
    detect_power(&mut report.components[11]);

    let mut sig: usize = 0;
    let mut total: usize = 0;
    for c in &report.components {
        if c.found {
            total += 1;
            sig ^= c.base_or_info.wrapping_mul(c.count.wrapping_add(1));
        }
    }
    report.total_found = total;
    report.signature = sig;
    DETECT_SIGNATURE.store(sig, Ordering::Release);
    report
}

pub fn detection_signature() -> usize {
    DETECT_SIGNATURE.load(Ordering::Acquire)
}

fn detect_cpu(out: &mut DetectedComponent) {
    if let Some(info) = crate::cpu::api::detect_cpu_info() {
        out.found = true;
        out.count = info.cores as usize;
        out.base_or_info = info.frequency_hz as usize;
    }
}

fn detect_gpu(out: &mut DetectedComponent) {
    let mut devices = [crate::gpu::GpuDevice {
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
    let found = crate::gpu::detect_gpus(&mut devices);
    if found > 0 {
        out.found = true;
        out.count = found;
        out.base_or_info = devices[0].bar0 as usize;
    }
}

fn detect_tpu(out: &mut DetectedComponent) {
    if crate::tpu::drivers::generic::GenericTpu::probe().is_some() {
        out.found = true;
        out.count = 1;
        static TPU_SIG: AtomicUsize = AtomicUsize::new(0);
        TPU_SIG.store(1, Ordering::Release);
        out.base_or_info = TPU_SIG.load(Ordering::Acquire);
    }
}

fn detect_lpu(out: &mut DetectedComponent) {
    let base = crate::firmware::devicetree::find_device_by_compatible(b"arm,lpu")
        .map(|(b, _, _)| b)
        .unwrap_or(0);
    if base == 0 {
        return;
    }
    if let Some(accel) = crate::lpu::drivers::generic::Accelerator::probe(base) {
        out.found = true;
        out.count = 1;
        out.base_or_info = accel.base;
    }
}

fn detect_iommu(out: &mut DetectedComponent) {
    if let Some(ctrl) = crate::iommu::controller::get() {
        out.found = true;
        out.count = 1;
        out.base_or_info = ctrl.is_enabled() as usize;
    } else if let Some(probed) = crate::iommu::controller::IommuController::probe() {
        out.found = true;
        out.count = 1;
        out.base_or_info = probed.is_enabled() as usize;
    }
}

fn detect_dma(out: &mut DetectedComponent) {
    if let Some(engine) = crate::dma::engine::get() {
        out.found = true;
        out.count = 1;
        let mut descs = [crate::dma::engine::Descriptor {
            phys: 0,
            len: 0,
            flags: 0,
        }; 1];
        let pending = engine.drain(&mut descs);
        out.base_or_info = pending;
    }
}

fn detect_interrupt(out: &mut DetectedComponent) {
    let ok = crate::interrupt::register(0xFF, dummy_irq_handler);
    if ok {
        crate::interrupt::unregister(0xFF);
        out.found = true;
        out.count = 256;
        out.base_or_info = 256;
    }
}

fn dummy_irq_handler() {}

fn detect_timer(out: &mut DetectedComponent) {
    let freq = crate::timer::clocksource::frequency();
    let ticks = crate::timer::clocksource::read_ticks();
    if freq > 0 || ticks > 0 {
        out.found = true;
        out.count = 1;
        out.base_or_info = freq as usize;
    }
}

fn detect_pci_bus(out: &mut DetectedComponent) {
    let count = crate::bus::discovery::device_count();
    if count > 0 {
        out.found = true;
        out.count = count;
        out.base_or_info = count;
    }
}

fn detect_topology(out: &mut DetectedComponent) {
    let topo = crate::topology::system::detect();
    if topo.sockets > 0 || topo.total_cores > 0 {
        out.found = true;
        out.count = topo.total_cores;
        out.base_or_info = (topo.sockets as usize) << 16 | topo.total_threads;
    }
}

fn detect_thermal(out: &mut DetectedComponent) {
    let zones = crate::thermal::api::zone_count();
    if zones > 0 {
        out.found = true;
        out.count = zones;
        if let Some(temp) = crate::thermal::api::read_thermal_zone(0) {
            out.base_or_info = temp as usize;
        }
    }
}

fn detect_power(out: &mut DetectedComponent) {
    let gov = crate::power::governor::Governor::get_policy();
    out.found = true;
    out.count = 1;
    out.base_or_info = gov as usize;
}
