use hardware::sys;

fn configure_drm_only() {
    sys::gpu::set_drm_constants(sys::gpu::DrmConstants {
        ioctl_version: 0xC040_6400,
        ioctl_gem_close: 0x4008_6409,
        ioctl_radeon_info: 0xC010_6467,
        ioctl_radeon_gem_info: 0xC018_645C,
        ioctl_radeon_gem_create: 0xC020_645D,
        ioctl_radeon_gem_mmap: 0xC020_645E,
        ioctl_radeon_gem_wait_idle: 0x4008_6464,
        ioctl_radeon_cs: 0xC020_6466,
        radeon_info_device_id: 0x00,
        radeon_info_num_gb_pipes: 0x01,
        radeon_info_vram_usage: 0x1E,
        radeon_info_active_cu_count: 0x20,
        radeon_info_current_gpu_sclk: 0x21,
        radeon_info_current_gpu_mclk: 0x22,
        radeon_info_current_gpu_temp: 0x09,
        radeon_info_max_se: 0x12,
        radeon_info_max_sh_per_se: 0x13,
        radeon_gem_domain_vram: 0x04,
        radeon_gem_domain_gtt: 0x02,
        radeon_chunk_id_relocs: 0x01,
        radeon_chunk_id_ib: 0x02,
        radeon_chunk_id_flags: 0x03,
        radeon_cs_ring_gfx: 0,
        radeon_cs_use_vm: 0x02,
    });
}

#[test]
fn drm_open_none_without_open_fn() {
    configure_drm_only();
    let dev = sys::gpu::drm::open();
    assert!(dev.is_none());
}

#[test]
fn drm_open_none_with_failing_open_fn() {
    configure_drm_only();
    sys::gpu::drm::set_open_drm_fn(|| -1);
    let dev = sys::gpu::drm::open();
    assert!(dev.is_none());
}

#[test]
fn drm_constants_are_retained() {
    configure_drm_only();
    let dev = sys::gpu::drm::open();
    assert!(dev.is_none());
}

#[test]
fn gpu_detect_returns_zero_without_pci_or_callback() {
    let mut buf = [sys::gpu::GpuDevice {
        bus: 0, device: 0, function: 0, vendor_id: 0, device_id: 0,
        class: 0, subclass: 0, prog_if: 0, bar0: 0,
    }; 4];
    let count = sys::gpu::detect_gpus(&mut buf);
    assert!(count <= buf.len());
}

#[test]
fn gpu_detect_with_callback_returns_value() {
    sys::gpu::set_detect_gpu_fn(|| {
        Some(sys::gpu::RawGpuId {
            vendor: 0x1002,
            raw_id: 0x6720_0000,
        })
    });
    let mut buf = [sys::gpu::GpuDevice {
        bus: 0, device: 0, function: 0, vendor_id: 0, device_id: 0,
        class: 0, subclass: 0, prog_if: 0, bar0: 0,
    }; 4];
    let count = sys::gpu::detect_gpus(&mut buf);
    assert!(count > 0);
    assert_eq!(buf[0].vendor_id, 0x1002);
}

#[test]
fn mali_parse_known_id() {
    let prod = sys::gpu::parse_mali_gpu_id(0x7212_0000);
    assert_ne!(prod, 0x0001);
}

#[test]
fn adreno_parse_known_id() {
    let prod = sys::gpu::parse_adreno_chip_id(0x0630_0000);
    assert_ne!(prod, 0x0001);
}
