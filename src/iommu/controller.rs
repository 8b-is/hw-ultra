use crate::common::once::Once;
use crate::dma::buffer::DmaBuffer;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

const IOVA_BASE: usize = 0x1_0000_0000;
const IOVA_LIMIT: usize = 0x2_0000_0000;
const PAGE_SIZE: usize = 4096;

pub struct IommuController {
    base: usize,
    enabled: AtomicUsize,
    mappings: UnsafeCell<[(usize, usize); 64]>,
    map_count: AtomicUsize,
    iova_next: AtomicUsize,
}

unsafe impl Sync for IommuController {}

impl IommuController {
    pub fn new(base: usize) -> Self {
        IommuController {
            base,
            enabled: AtomicUsize::new(0),
            mappings: UnsafeCell::new([(0, 0); 64]),
            map_count: AtomicUsize::new(0),
            iova_next: AtomicUsize::new(IOVA_BASE),
        }
    }

    pub fn probe() -> Option<Self> {
        if let Some(addr) = crate::firmware::acpi::find_vtd_base() {
            return Some(IommuController::new(addr));
        }
        if let Some(addr) = crate::firmware::devicetree::find_smmu_base() {
            return Some(IommuController::new(addr));
        }
        None
    }

    pub fn init(&self) -> bool {
        let val = crate::hardware_access::mmio_read32(self.base).unwrap_or(0) as usize;
        if val == 0 || val == 0xffff_ffff {
            return false;
        }
        self.enabled.store(1, Ordering::Release);
        true
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire) != 0
    }

    fn align_up(sz: usize, align: usize) -> usize {
        let a = if align == 0 { PAGE_SIZE } else { align };
        (sz + a - 1) & !(a - 1)
    }

    fn alloc_iova(&self, size: usize, align: usize) -> Option<usize> {
        let sz = Self::align_up(size, align);
        let mut cur = self.iova_next.load(Ordering::Acquire);
        loop {
            let next = cur.checked_add(sz)?;
            if next > IOVA_LIMIT {
                return None;
            }
            if self
                .iova_next
                .compare_exchange(cur, next, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return Some(cur);
            }
            cur = self.iova_next.load(Ordering::Acquire);
        }
    }

    pub fn map_dma_buffer(&self, buf: &DmaBuffer, align: usize) -> Option<usize> {
        let len = buf.len();
        let iova = self.alloc_iova(len, align)?;
        let idx = self.map_count.fetch_add(1, Ordering::AcqRel);
        if idx >= 64 {
            return None;
        }
        unsafe { (*self.mappings.get())[idx] = (iova, buf.phys_addr()) }
        Some(iova)
    }

    pub fn unmap_iova(&self, iova: usize) -> bool {
        for i in 0..64 {
            unsafe {
                let (v, phys_addr) = (*self.mappings.get())[i];
                if phys_addr == 0 {
                    continue;
                }
                if v == iova {
                    (*self.mappings.get())[i] = (0, 0);
                    return true;
                }
            }
        }
        false
    }

    pub fn translate_iova(&self, iova: usize) -> Option<usize> {
        for i in 0..64 {
            unsafe {
                let (v, p) = (*self.mappings.get())[i];
                if v == 0 {
                    continue;
                }
                if iova >= v && iova < v + PAGE_SIZE {
                    let off = iova - v;
                    return Some(p + off);
                }
            }
        }
        None
    }
}

static IOMMU_ONCE: Once<IommuController> = Once::new();

pub fn init() {
    if let Some(ctrl) = IommuController::probe() {
        if IOMMU_ONCE.set(ctrl) {
            if let Some(c) = IOMMU_ONCE.get() {
                c.init();
            }
        }
    }
}

pub fn get() -> Option<&'static IommuController> {
    IOMMU_ONCE.get()
}
