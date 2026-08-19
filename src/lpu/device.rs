use crate::common::once::Once;
use crate::dma::buffer::DmaBuffer;
use crate::dma::engine;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct LpuDeviceInner {
    base: usize,
    initialized: AtomicBool,
    mode: AtomicUsize,
}

pub struct LpuDevice {
    inner: UnsafeCell<LpuDeviceInner>,
}

unsafe impl Sync for LpuDevice {}

impl LpuDevice {
    pub fn new_with_base(base: usize) -> Self {
        LpuDevice {
            inner: UnsafeCell::new(LpuDeviceInner {
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

    pub fn submit_task(
        &self,
        payload: &[u8],
        flags: u32,
        align: usize,
    ) -> Result<usize, &'static str> {
        let buf = DmaBuffer::new(payload.len(), align).ok_or("alloc failed")?;
        unsafe {
            core::ptr::copy_nonoverlapping(payload.as_ptr(), buf.as_ptr(), payload.len());
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
                if d.len == payload.len() {
                    let ok = crate::dma::engine::DmaEngine::unmap_iova(d.phys);
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

static LPU_DEVICE: Once<LpuDevice> = Once::new();

pub fn init_with_base(base: usize) {
    let dev = LpuDevice::new_with_base(base);
    if LPU_DEVICE.set(dev) {
        if let Some(d) = LPU_DEVICE.get() {
            d.init();
        }
    }
}

pub fn init() {
    init_with_base(0);
}

pub fn get() -> Option<&'static LpuDevice> {
    LPU_DEVICE.get()
}

static LPU_IRQ_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn lpu_irq_shim() {
    LPU_IRQ_COUNT.fetch_add(1, Ordering::AcqRel);
}

pub fn lpu_irq_count() -> usize {
    LPU_IRQ_COUNT.load(Ordering::Acquire)
}

pub fn register_irq_vector(vec: usize) -> bool {
    crate::interrupt::register(vec, lpu_irq_shim)
}
