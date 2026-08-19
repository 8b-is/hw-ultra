use crate::dma::buffer::DmaBuffer;
use crate::dma::engine;
use crate::gpu::GpuDevice;
use core::sync::atomic::{AtomicUsize, Ordering};

static GPU_IRQ_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn gpu_irq_shim() {
    GPU_IRQ_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub struct GpuDriver {
    pub device: GpuDevice,
    mapped_bar: Option<usize>,
}

impl GpuDriver {
    pub fn probe(dev: &GpuDevice) -> Option<Self> {
        Some(GpuDriver {
            device: *dev,
            mapped_bar: None,
        })
    }

    pub fn init(&mut self) -> bool {
        if self.device.bar0 != 0 {
            let bar_off = 0x10u8;
            if let Some(sz) = crate::bus::pci::api::probe_bar_size(
                self.device.bus,
                self.device.device,
                self.device.function,
                bar_off,
            ) {
                let base = (self.device.bar0 as usize) & !0xfusize;
                let va = crate::bus::pci::api::map_mmio_region(base, sz);
                self.mapped_bar = Some(va);
            } else {
                self.mapped_bar = Some(self.device.bar0 as usize);
            }
        }
        if let Some(d) = crate::bus::pci::api::read_config_u32(
            self.device.bus,
            self.device.device,
            self.device.function,
            0x3C,
        ) {
            let irq_line = (d & 0xff) as u8;
            if irq_line != 0 && irq_line != 0xff {
                let vec = 0x20u8.wrapping_add(irq_line);
                let ok = crate::interrupt::register(vec as usize, gpu_irq_shim);
                debug_assert!(ok);
                crate::interrupt::Controller::enable_irq(irq_line);
            }
        }
        true
    }

    pub fn irq_count() -> usize {
        GPU_IRQ_COUNT.load(Ordering::Acquire)
    }

    pub fn allocate_framebuffer(&self, size: usize, align: usize) -> Option<usize> {
        let buf = crate::gpu::memory::allocator::GpuAllocator::alloc_framebuffer(size, align)?;
        crate::gpu::memory::allocator::GpuAllocator::map_for_device(&buf, align)
    }

    pub fn submit_command_queue(&self, cmds: &[&[u8]]) -> Result<usize, &'static str> {
        let total: usize = cmds.iter().map(|c| c.len()).sum();
        let buf = crate::gpu::memory::allocator::GpuAllocator::alloc_framebuffer(total, 4096)
            .ok_or("alloc failed")?;
        let mut off = 0usize;
        for c in cmds {
            unsafe {
                core::ptr::copy_nonoverlapping(c.as_ptr(), buf.as_ptr().add(off), c.len());
            }
            off += c.len();
        }
        let engine = engine::get().ok_or("no dma engine")?;
        engine.submit_buffer(&buf, 0, 4096)
    }

    pub fn submit_command(&self, cmd: &[u8]) -> Result<usize, &'static str> {
        let engine = engine::get().ok_or("no dma engine")?;
        let buf = DmaBuffer::new(cmd.len(), 4096).ok_or("alloc failed")?;
        unsafe {
            core::ptr::copy_nonoverlapping(cmd.as_ptr(), buf.as_ptr(), cmd.len());
        }
        engine.submit_buffer(&buf, 0, 4096)
    }

    pub fn submit_mmio_command(&self, cmd_addr: usize, cmd_len: u32) -> bool {
        if let Some(bar) = self.mapped_bar {
            crate::hardware_access::mmio_write32(bar + 0x100, cmd_addr as u32);
            crate::hardware_access::mmio_write32(bar + 0x104, cmd_len);
            crate::hardware_access::mmio_write32(bar + 0x108, 1);
            true
        } else {
            false
        }
    }

    pub fn cmd_count(&self) -> usize {
        GPU_IRQ_COUNT.load(Ordering::Acquire)
    }
}
