use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_MAPPINGS: usize = 64;

struct DmaMapping {
    virt_addr: AtomicUsize,
    phys_addr: AtomicUsize,
    size: AtomicUsize,
}

struct MappingTable {
    entries: [DmaMapping; MAX_MAPPINGS],
    count: AtomicUsize,
}

unsafe impl Sync for MappingTable {}

static TABLE: MappingTable = MappingTable {
    entries: [const {
        DmaMapping {
            virt_addr: AtomicUsize::new(0),
            phys_addr: AtomicUsize::new(0),
            size: AtomicUsize::new(0),
        }
    }; MAX_MAPPINGS],
    count: AtomicUsize::new(0),
};

pub fn map(virt: usize, phys: usize, size: usize) -> bool {
    let idx = TABLE.count.load(Ordering::Acquire);
    if idx >= MAX_MAPPINGS {
        return false;
    }
    TABLE.entries[idx].virt_addr.store(virt, Ordering::Release);
    TABLE.entries[idx].phys_addr.store(phys, Ordering::Release);
    TABLE.entries[idx].size.store(size, Ordering::Release);
    TABLE.count.store(idx + 1, Ordering::Release);
    true
}

pub fn translate(virt: usize) -> Option<usize> {
    let count = TABLE.count.load(Ordering::Acquire);
    let mut i = 0;
    while i < count {
        let va = TABLE.entries[i].virt_addr.load(Ordering::Acquire);
        let sz = TABLE.entries[i].size.load(Ordering::Acquire);
        if virt >= va && virt < va + sz {
            let offset = virt - va;
            let pa = TABLE.entries[i].phys_addr.load(Ordering::Acquire);
            return Some(pa + offset);
        }
        i += 1;
    }
    None
}

pub fn mapping_count() -> usize {
    TABLE.count.load(Ordering::Acquire)
}
