pub fn hardware_start() -> ! {
    let sys = crate::runtime::system::System::init();

    sys.enforce_limits(true);
    sys.set_dma_limit(64 * 1024 * 1024);
    sys.set_memory_limit(256 * 1024 * 1024);
    sys.set_irq_limit(128);

    let status = sys.status();
    crate::runtime::system::set_system_signature(
        status.total_ram
            ^ status.dma_allocated
            ^ status.mem_allocated
            ^ (status.gpu_present as usize)
            ^ (status.tpu_present as usize)
            ^ (status.lpu_present as usize)
            ^ status.irq_registered,
    );

    let mut accel_sig = 0usize;
    sys.for_each_accel(|h| {
        accel_sig ^= h.mmio_base ^ h.mmio_size ^ h.vendor_id ^ h.device_id;
    });
    crate::runtime::system::set_system_signature(
        crate::runtime::system::system_signature() ^ accel_sig,
    );

    use crate::runtime::monitor::{Component, Precision};
    sys.set_temp_limit(Component::Cpu, 95_000);
    sys.set_temp_limit(Component::Gpu, 90_000);
    sys.set_temp_limit(Component::Tpu, 85_000);
    sys.set_temp_limit(Component::Lpu, 80_000);

    let cpu_max = if let Some(info) = crate::cpu::get_info() {
        if info.frequency_hz > 0 {
            info.frequency_hz as usize
        } else {
            5_000_000_000
        }
    } else {
        5_000_000_000
    };
    sys.set_freq_bounds(Component::Cpu, cpu_max / 6, cpu_max);
    sys.set_freq_bounds(Component::Gpu, 300_000_000, 2_500_000_000);
    sys.set_freq_bounds(Component::Tpu, 500_000_000, 1_500_000_000);
    sys.set_freq_bounds(Component::Lpu, 200_000_000, 1_000_000_000);

    sys.set_precision(Component::Gpu, Precision::Fp16);
    sys.set_precision(Component::Tpu, Precision::Bf16);
    sys.set_precision(Component::Lpu, Precision::Int8);

    let snaps = sys.all_snapshots();
    let mut monitor_sig = 0usize;
    let mut i = 0;
    while i < snaps.len() {
        monitor_sig ^=
            snaps[i].temp_millideg as usize ^ snaps[i].freq_hz ^ snaps[i].cycles ^ snaps[i].ops;
        i += 1;
    }
    crate::runtime::system::set_system_signature(
        crate::runtime::system::system_signature() ^ monitor_sig,
    );

    sys.spawn_workers();

    loop {
        core::hint::spin_loop();
    }
}
