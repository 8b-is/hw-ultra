use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Clone, Copy)]
pub enum AcceleratorKind {
    Gpu,
    Tpu,
    Lpu,
}

pub struct AccelHandle {
    pub kind: AcceleratorKind,
    pub arch: crate::arch::Architecture,
    pub mmio_base: usize,
    pub mmio_size: usize,
    pub initialized: bool,
    pub vendor_id: usize,
    pub device_id: usize,
}

impl AccelHandle {
    pub fn read_reg(&self, offset: usize) -> u32 {
        match (self.kind, self.arch) {
            (AcceleratorKind::Gpu, crate::arch::Architecture::X86_64) => {
                crate::arch::x86_64::gpu::read_gpu_reg(offset)
            }
            (AcceleratorKind::Gpu, crate::arch::Architecture::AArch64) => {
                crate::arch::aarch64::gpu::read_gpu_reg(offset)
            }
            (AcceleratorKind::Tpu, crate::arch::Architecture::X86_64) => {
                crate::arch::x86_64::tpu::read_tpu_reg(offset)
            }
            (AcceleratorKind::Tpu, crate::arch::Architecture::AArch64) => {
                crate::arch::aarch64::tpu::read_tpu_reg(offset)
            }
            (AcceleratorKind::Lpu, crate::arch::Architecture::X86_64) => {
                crate::arch::x86_64::lpu::read_lpu_reg(offset)
            }
            (AcceleratorKind::Lpu, crate::arch::Architecture::AArch64) => {
                crate::arch::aarch64::lpu::read_lpu_reg(offset)
            }
            _ => 0,
        }
    }

    pub fn write_reg(&self, offset: usize, val: u32) {
        match (self.kind, self.arch) {
            (AcceleratorKind::Gpu, crate::arch::Architecture::X86_64) => {
                crate::arch::x86_64::gpu::write_gpu_reg(offset, val)
            }
            (AcceleratorKind::Gpu, crate::arch::Architecture::AArch64) => {
                crate::arch::aarch64::gpu::write_gpu_reg(offset, val)
            }
            (AcceleratorKind::Tpu, crate::arch::Architecture::X86_64) => {
                crate::arch::x86_64::tpu::write_tpu_reg(offset, val)
            }
            (AcceleratorKind::Tpu, crate::arch::Architecture::AArch64) => {
                crate::arch::aarch64::tpu::write_tpu_reg(offset, val)
            }
            (AcceleratorKind::Lpu, crate::arch::Architecture::X86_64) => {
                crate::arch::x86_64::lpu::write_lpu_reg(offset, val)
            }
            (AcceleratorKind::Lpu, crate::arch::Architecture::AArch64) => {
                crate::arch::aarch64::lpu::write_lpu_reg(offset, val)
            }
            _ => {}
        }
    }

    pub fn is_ready(&self) -> bool {
        let status = self.read_reg(0x04);
        status & 0x01 != 0
    }

    pub fn is_error(&self) -> bool {
        let status = self.read_reg(0x04);
        status & 0x04 != 0
    }

    pub fn reset(&self) {
        self.write_reg(0x00, 1);
    }
}

static GPU_HANDLE: GpuSlot = GpuSlot::new();
static TPU_HANDLE: TpuSlot = TpuSlot::new();
static LPU_HANDLE: LpuSlot = LpuSlot::new();

struct GpuSlot {
    base: AtomicUsize,
    size: AtomicUsize,
    vendor: AtomicUsize,
    devid: AtomicUsize,
    ready: AtomicBool,
}

impl GpuSlot {
    const fn new() -> Self {
        GpuSlot {
            base: AtomicUsize::new(0),
            size: AtomicUsize::new(0),
            vendor: AtomicUsize::new(0),
            devid: AtomicUsize::new(0),
            ready: AtomicBool::new(false),
        }
    }
}

struct TpuSlot {
    base: AtomicUsize,
    size: AtomicUsize,
    vendor: AtomicUsize,
    devid: AtomicUsize,
    ready: AtomicBool,
}

impl TpuSlot {
    const fn new() -> Self {
        TpuSlot {
            base: AtomicUsize::new(0),
            size: AtomicUsize::new(0),
            vendor: AtomicUsize::new(0),
            devid: AtomicUsize::new(0),
            ready: AtomicBool::new(false),
        }
    }
}

struct LpuSlot {
    base: AtomicUsize,
    size: AtomicUsize,
    vendor: AtomicUsize,
    devid: AtomicUsize,
    ready: AtomicBool,
}

impl LpuSlot {
    const fn new() -> Self {
        LpuSlot {
            base: AtomicUsize::new(0),
            size: AtomicUsize::new(0),
            vendor: AtomicUsize::new(0),
            devid: AtomicUsize::new(0),
            ready: AtomicBool::new(false),
        }
    }
}

pub fn register_gpu(base: usize, size: usize, vendor: usize, devid: usize) {
    GPU_HANDLE.base.store(base, Ordering::Release);
    GPU_HANDLE.size.store(size, Ordering::Release);
    GPU_HANDLE.vendor.store(vendor, Ordering::Release);
    GPU_HANDLE.devid.store(devid, Ordering::Release);
    GPU_HANDLE.ready.store(true, Ordering::Release);
}

pub fn register_tpu(base: usize, size: usize, vendor: usize, devid: usize) {
    TPU_HANDLE.base.store(base, Ordering::Release);
    TPU_HANDLE.size.store(size, Ordering::Release);
    TPU_HANDLE.vendor.store(vendor, Ordering::Release);
    TPU_HANDLE.devid.store(devid, Ordering::Release);
    TPU_HANDLE.ready.store(true, Ordering::Release);
}

pub fn register_lpu(base: usize, size: usize, vendor: usize, devid: usize) {
    LPU_HANDLE.base.store(base, Ordering::Release);
    LPU_HANDLE.size.store(size, Ordering::Release);
    LPU_HANDLE.vendor.store(vendor, Ordering::Release);
    LPU_HANDLE.devid.store(devid, Ordering::Release);
    LPU_HANDLE.ready.store(true, Ordering::Release);
}

pub fn gpu_handle() -> Option<AccelHandle> {
    if !GPU_HANDLE.ready.load(Ordering::Acquire) {
        return None;
    }
    Some(AccelHandle {
        kind: AcceleratorKind::Gpu,
        arch: crate::arch::detect_arch(),
        mmio_base: GPU_HANDLE.base.load(Ordering::Acquire),
        mmio_size: GPU_HANDLE.size.load(Ordering::Acquire),
        initialized: true,
        vendor_id: GPU_HANDLE.vendor.load(Ordering::Acquire),
        device_id: GPU_HANDLE.devid.load(Ordering::Acquire),
    })
}

pub fn tpu_handle() -> Option<AccelHandle> {
    if !TPU_HANDLE.ready.load(Ordering::Acquire) {
        return None;
    }
    Some(AccelHandle {
        kind: AcceleratorKind::Tpu,
        arch: crate::arch::detect_arch(),
        mmio_base: TPU_HANDLE.base.load(Ordering::Acquire),
        mmio_size: TPU_HANDLE.size.load(Ordering::Acquire),
        initialized: true,
        vendor_id: TPU_HANDLE.vendor.load(Ordering::Acquire),
        device_id: TPU_HANDLE.devid.load(Ordering::Acquire),
    })
}

pub fn lpu_handle() -> Option<AccelHandle> {
    if !LPU_HANDLE.ready.load(Ordering::Acquire) {
        return None;
    }
    Some(AccelHandle {
        kind: AcceleratorKind::Lpu,
        arch: crate::arch::detect_arch(),
        mmio_base: LPU_HANDLE.base.load(Ordering::Acquire),
        mmio_size: LPU_HANDLE.size.load(Ordering::Acquire),
        initialized: true,
        vendor_id: LPU_HANDLE.vendor.load(Ordering::Acquire),
        device_id: LPU_HANDLE.devid.load(Ordering::Acquire),
    })
}
