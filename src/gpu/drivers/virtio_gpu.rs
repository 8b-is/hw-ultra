use crate::dma::engine;
use crate::gpu::GpuDevice;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct VirtioGpu {
    pub device: GpuDevice,
}

impl VirtioGpu {
    pub fn probe(dev: &GpuDevice) -> Option<Self> {
        if dev.vendor_id == 0x1AF4 {
            Some(VirtioGpu { device: *dev })
        } else {
            None
        }
    }

    pub fn init(&mut self) -> bool {
        if let Some(d) = crate::bus::pci::api::read_config_u32(
            self.device.bus,
            self.device.device,
            self.device.function,
            0x3C,
        ) {
            let irq_line = (d & 0xff) as u8;
            if irq_line != 0 && irq_line != 0xff {
                let vec = 0x20u8.wrapping_add(irq_line);
                let ok = crate::interrupt::register(vec as usize, virtio_irq_shim);
                debug_assert!(ok);
                crate::interrupt::Controller::enable_irq(irq_line);
            }
        }
        true
    }

    pub fn submit_command(&self, cmd: &[u8]) -> Result<usize, &'static str> {
        let buf = crate::gpu::memory::allocator::GpuAllocator::alloc_framebuffer(cmd.len(), 4096)
            .ok_or("alloc failed")?;
        unsafe {
            core::ptr::copy_nonoverlapping(cmd.as_ptr(), buf.as_ptr(), cmd.len());
        }
        let engine = engine::get().ok_or("no dma engine")?;
        engine.submit_buffer(&buf, 0, 4096)
    }
}

static VIRTIO_IRQ_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn virtio_irq_shim() {
    VIRTIO_IRQ_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub fn virtio_irq_count() -> usize {
    VIRTIO_IRQ_COUNT.load(Ordering::Acquire)
}
