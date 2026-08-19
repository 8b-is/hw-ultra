use crate::common::once::Once;
use crate::dma::buffer::DmaBuffer;
use crate::dma::engine;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct TpuDeviceInner {
    base: usize,
    initialized: AtomicBool,
    mode: AtomicUsize,
}

pub struct TpuDevice {
    inner: UnsafeCell<TpuDeviceInner>,
}

unsafe impl Sync for TpuDevice {}

impl TpuDevice {
    pub fn new_with_base(base: usize) -> Self {
        TpuDevice {
            inner: UnsafeCell::new(TpuDeviceInner {
                base,
                initialized: AtomicBool::new(false),
                mode: AtomicUsize::new(0),
            }),
        }
    }

    pub fn init(&self) -> bool {
        unsafe {
            (*self.inner.get())
                .initialized
                .store(true, Ordering::Release);
        }
        true
    }

    pub fn is_initialized(&self) -> bool {
        unsafe { (*self.inner.get()).initialized.load(Ordering::Acquire) }
    }

    pub fn base_addr(&self) -> usize {
        unsafe { (*self.inner.get()).base }
    }

    pub fn set_mode(&self, m: usize) {
        unsafe {
            (*self.inner.get()).mode.store(m, Ordering::Release);
        }
    }

    pub fn get_mode(&self) -> usize {
        unsafe { (*self.inner.get()).mode.load(Ordering::Acquire) }
    }

    pub fn transfer(&self, data: &[u8], flags: u32, align: usize) -> Result<usize, &'static str> {
        let buf = DmaBuffer::new(data.len(), align).ok_or("alloc failed")?;
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), buf.as_ptr(), data.len());
        }
        let engine = engine::get().ok_or("no dma engine")?;
        let enq = engine.submit_buffer(&buf, flags, align)?;
        if enq == 0 {
            return Err("submit failed");
        }
        let mut out = [crate::dma::engine::Descriptor {
            phys: 0,
            len: 0,
            flags: 0,
        }; 8];
        let mut waited = 0;
        loop {
            let drained = engine.drain(&mut out);
            for d in out.iter().take(drained) {
                if d.len == data.len() {
                    let ok = crate::dma::engine::DmaEngine::unmap_descriptor(d);
                    debug_assert!(ok);
                    return Ok(d.len);
                }
            }
            waited += 1;
            if waited > 1000 {
                break;
            }
        }
        Err("timeout")
    }
}

static TPU_DEVICE: Once<TpuDevice> = Once::new();

pub fn init_with_base(base: usize) {
    let dev = TpuDevice::new_with_base(base);
    if TPU_DEVICE.set(dev) {
        if let Some(d) = TPU_DEVICE.get() {
            d.init();
        }
    }
}

pub fn get() -> Option<&'static TpuDevice> {
    TPU_DEVICE.get()
}

static TPU_IRQ_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn tpu_irq_shim() {
    TPU_IRQ_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub fn tpu_irq_count() -> usize {
    TPU_IRQ_COUNT.load(Ordering::Acquire)
}

pub fn register_irq_vector(vec: usize) -> bool {
    crate::interrupt::register(vec, tpu_irq_shim)
}
