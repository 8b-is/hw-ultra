use crate::memory::phys::frame::Frame;
use crate::memory::virt::address::VirtAddr;
use core::sync::atomic::{AtomicUsize, Ordering};

static PAGES_MAPPED: AtomicUsize = AtomicUsize::new(0);
static PAGES_UNMAPPED: AtomicUsize = AtomicUsize::new(0);

pub fn map(virt: VirtAddr, frame: Frame) -> bool {
    let result = crate::memory::virt::mapping::map_page(virt, frame);
    if result {
        PAGES_MAPPED.fetch_add(1, Ordering::AcqRel);
    }
    result
}

pub fn unmap(virt: VirtAddr) {
    crate::memory::virt::mapping::unmap_page(virt);
    PAGES_UNMAPPED.fetch_add(1, Ordering::AcqRel);
}

pub fn mapped_count() -> usize {
    PAGES_MAPPED.load(Ordering::Acquire)
}

pub fn unmapped_count() -> usize {
    PAGES_UNMAPPED.load(Ordering::Acquire)
}
