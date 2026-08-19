use crate::gpu::GpuDevice;

pub struct GpuMmio {
    pub base: *mut u8,
    pub size: usize,
    pub fd: i64,
}

pub struct GpuHwInfo {
    pub vram_bytes: u64,
    pub shader_engines: u32,
    pub compute_units: u32,
    pub gpu_active: bool,
    pub status_reg: u32,
}

pub fn bar_size(bus: u8, dev: u8, func: u8, bar_index: usize) -> usize {
    let offset = 0x10 + (bar_index as u8) * 4;
    crate::bus::pci::config::probe_bar_size(bus, dev, func, offset).unwrap_or(0)
}

impl GpuMmio {
    pub fn open(gpu: &GpuDevice, resource: u8) -> Option<Self> {
        let size = bar_size(gpu.bus, gpu.device, gpu.function, resource as usize);
        if size == 0 {
            return None;
        }

        let bar_addr = if resource == 0 {
            gpu.bar0 as usize
        } else {
            let offset = 0x10 + resource * 4;
            let raw =
                crate::bus::pci::api::read_config_u32(gpu.bus, gpu.device, gpu.function, offset)
                    .unwrap_or(0);
            (raw & 0xFFFF_FFF0) as usize
        };
        if bar_addr == 0 {
            return None;
        }

        Some(GpuMmio {
            base: bar_addr as *mut u8,
            size,
            fd: -1,
        })
    }

    pub fn read32(&self, offset: usize) -> u32 {
        if offset + 4 > self.size {
            return 0;
        }
        unsafe { core::ptr::read_volatile(self.base.add(offset) as *const u32) }
    }

    pub fn write32(&self, offset: usize, val: u32) {
        if offset + 4 > self.size {
            return;
        }
        unsafe {
            core::ptr::write_volatile(self.base.add(offset) as *mut u32, val);
        }
    }

    pub fn close(&mut self) {
        self.base = core::ptr::null_mut();
        self.size = 0;
    }
}

pub fn detect_hw_info(gpu: &GpuDevice, mmio: &GpuMmio) -> GpuHwInfo {
    if gpu.vendor_id == 0x1002 {
        return crate::gpu::drivers::amd::detect_capabilities(mmio);
    }
    let status = mmio.read32(0);
    GpuHwInfo {
        vram_bytes: 0,
        shader_engines: 0,
        compute_units: 0,
        gpu_active: status != 0 && status != 0xFFFFFFFF,
        status_reg: status,
    }
}

pub fn stress_registers(gpu: &GpuDevice, mmio: &GpuMmio, iterations: usize) -> usize {
    if gpu.vendor_id == 0x1002 {
        return crate::gpu::drivers::amd::stress_scratch_registers(mmio, iterations);
    }
    0
}

pub fn stress_vram(gpu: &GpuDevice, iterations: usize) -> usize {
    let mmio = match GpuMmio::open(gpu, 2) {
        Some(m) => m,
        None => return 0,
    };
    let region = if mmio.size > 4096 { 4096 } else { mmio.size };
    let words = region / 4;
    let mut verified = 0usize;
    let mut iter = 0;
    while iter < iterations {
        let mut wi = 0;
        while wi < words {
            let pattern = (iter as u32)
                .wrapping_mul(0x6C078965)
                .wrapping_add(wi as u32);
            mmio.write32(wi * 4, pattern);
            wi += 1;
        }
        wi = 0;
        while wi < words {
            let pattern = (iter as u32)
                .wrapping_mul(0x6C078965)
                .wrapping_add(wi as u32);
            let readback = mmio.read32(wi * 4);
            if readback == pattern {
                verified += 1;
            }
            wi += 1;
        }
        iter += 1;
    }
    let mut mmio = mmio;
    mmio.close();
    verified
}
