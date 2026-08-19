use crate::common::once::Once;
use crate::dma::buffer::DmaBuffer;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone)]
pub struct Descriptor {
    pub phys: usize,
    pub len: usize,
    pub flags: u32,
}

const RING_SIZE: usize = 128;

pub struct DmaEngine {
    ring: UnsafeCell<[Descriptor; RING_SIZE]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl Sync for DmaEngine {}

impl DmaEngine {
    pub fn init() -> Self {
        DmaEngine {
            ring: UnsafeCell::new(
                [Descriptor {
                    phys: 0,
                    len: 0,
                    flags: 0,
                }; RING_SIZE],
            ),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn prepare_descriptor(buf: &DmaBuffer, flags: u32) -> Descriptor {
        Descriptor {
            phys: buf.phys_addr(),
            len: buf.len(),
            flags,
        }
    }

    pub fn prepare_descriptor_with_phys(phys: usize, len: usize, flags: u32) -> Descriptor {
        Descriptor { phys, len, flags }
    }

    pub fn submit(&self, descs: &[Descriptor]) -> usize {
        let mut enq = 0usize;
        for d in descs {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = (head + 1) % RING_SIZE;
            if next == tail {
                break;
            }
            unsafe {
                (*self.ring.get())[head] = *d;
            }
            self.head.store(next, Ordering::Release);
            enq += 1;
        }
        enq
    }

    pub fn submit_buffer(
        &self,
        buf: &DmaBuffer,
        flags: u32,
        align: usize,
    ) -> Result<usize, &'static str> {
        let mapped = if let Some(ctrl) = crate::iommu::controller::get() {
            ctrl.map_dma_buffer(buf, align)
        } else {
            Some(buf.phys_addr())
        };

        let phys = match mapped {
            Some(p) => p,
            None => return Err("IOMMU mapping failed"),
        };

        let desc = Self::prepare_descriptor_with_phys(phys, buf.len(), flags);
        let enq = self.submit(core::slice::from_ref(&desc));
        if enq == 0 {
            Err("ring full")
        } else {
            Ok(enq)
        }
    }

    pub fn drain(&self, out: &mut [Descriptor]) -> usize {
        let mut cnt = 0usize;
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let head = self.head.load(Ordering::Acquire);
            if tail == head || cnt >= out.len() {
                break;
            }
            unsafe {
                out[cnt] = (*self.ring.get())[tail];
            }
            let next = (tail + 1) % RING_SIZE;
            self.tail.store(next, Ordering::Release);
            cnt += 1;
        }
        cnt
    }

    pub fn unmap_descriptor(desc: &Descriptor) -> bool {
        let iova = desc.phys;
        if let Some(ctrl) = crate::iommu::controller::get() {
            if ctrl.unmap_iova(iova) {
                return true;
            }
        }
        false
    }

    pub fn unmap_iova(iova: usize) -> bool {
        if let Some(ctrl) = crate::iommu::controller::get() {
            if ctrl.unmap_iova(iova) {
                return true;
            }
        }
        false
    }

    pub fn get() -> Option<&'static DmaEngine> {
        crate::dma::engine::get()
    }
}

static DMA_ENGINE: Once<DmaEngine> = Once::new();

pub fn init() {
    let eng = DmaEngine::init();
    if DMA_ENGINE.set(eng) {}
}

pub fn get() -> Option<&'static DmaEngine> {
    DMA_ENGINE.get()
}
