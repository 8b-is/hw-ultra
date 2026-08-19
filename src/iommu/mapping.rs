use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_IOMMU_MAPPINGS: usize = 64;

static MAPPING_COUNT: AtomicUsize = AtomicUsize::new(0);
static MAP_IOVA: [AtomicUsize; MAX_IOMMU_MAPPINGS] =
    [const { AtomicUsize::new(0) }; MAX_IOMMU_MAPPINGS];
static MAP_PHYS: [AtomicUsize; MAX_IOMMU_MAPPINGS] =
    [const { AtomicUsize::new(0) }; MAX_IOMMU_MAPPINGS];
static MAP_SIZE: [AtomicUsize; MAX_IOMMU_MAPPINGS] =
    [const { AtomicUsize::new(0) }; MAX_IOMMU_MAPPINGS];

pub fn map_iommu() {
    let count = MAPPING_COUNT.load(Ordering::Acquire);
    if count == 0 {
        add_mapping(0x0000_0000, 0x0000_0000, 0x1000);
    }
}

pub fn add_mapping(iova: usize, phys: usize, size: usize) -> bool {
    let idx = MAPPING_COUNT.fetch_add(1, Ordering::AcqRel);
    if idx >= MAX_IOMMU_MAPPINGS {
        MAPPING_COUNT.fetch_sub(1, Ordering::Release);
        return false;
    }
    MAP_IOVA[idx].store(iova, Ordering::Release);
    MAP_PHYS[idx].store(phys, Ordering::Release);
    MAP_SIZE[idx].store(size, Ordering::Release);
    true
}

pub fn translate(iova: usize) -> Option<usize> {
    let count = MAPPING_COUNT.load(Ordering::Acquire);
    let mut idx = 0;
    while idx < count && idx < MAX_IOMMU_MAPPINGS {
        let base = MAP_IOVA[idx].load(Ordering::Acquire);
        let size = MAP_SIZE[idx].load(Ordering::Acquire);
        if iova >= base && iova < base + size {
            let phys = MAP_PHYS[idx].load(Ordering::Acquire);
            return Some(phys + (iova - base));
        }
        idx += 1;
    }
    None
}

pub fn mapping_count() -> usize {
    MAPPING_COUNT.load(Ordering::Acquire)
}
