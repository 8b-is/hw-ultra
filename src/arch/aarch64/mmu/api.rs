use crate::common::once::OnceCopy;
use crate::memory::phys::frame::Frame;
use crate::memory::virt::address::VirtAddr;
use core::sync::atomic::Ordering;

pub type PhysToVirtFn = fn(usize) -> Option<usize>;

static PHYS_TO_VIRT_CB: OnceCopy<PhysToVirtFn> = OnceCopy::new();

pub fn set_phys_to_virt_cb(f: PhysToVirtFn) {
    PHYS_TO_VIRT_CB.set(f);
}

fn phys_to_virt_addr(paddr: usize) -> Option<usize> {
    if let Some(cb) = PHYS_TO_VIRT_CB.get() {
        cb(paddr)
    } else {
        None
    }
}

pub struct PageTable {
    root: crate::memory::phys::frame::Frame,
}
pub struct Entry {
    pub vaddr: core::sync::atomic::AtomicUsize,
    pub paddr: core::sync::atomic::AtomicUsize,
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
}

impl Entry {
    pub const fn new() -> Self {
        Entry {
            vaddr: core::sync::atomic::AtomicUsize::new(0),
            paddr: core::sync::atomic::AtomicUsize::new(0),
        }
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

impl PageTable {
    pub fn new() -> Self {
        if let Some(f) = crate::memory::phys::allocator::PhysAllocator::alloc_frame() {
            PageTable {
                root: crate::memory::phys::frame::Frame::new(f.as_usize()),
            }
        } else {
            PageTable {
                root: crate::memory::phys::frame::Frame::new(0),
            }
        }
    }

    pub fn root_frame(&self) -> crate::memory::phys::frame::Frame {
        self.root
    }
}

const MAX_MAPPINGS: usize = 256;
use crate::common::once::Once;
use core::cell::UnsafeCell;

struct MappingStorage(UnsafeCell<[core::sync::atomic::AtomicUsize; MAX_MAPPINGS]>);

unsafe impl Sync for MappingStorage {}

static MAPPINGS_INIT: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
static MAPPINGS_V_ONCE: Once<MappingStorage> = Once::new();
static MAPPINGS_P_ONCE: Once<MappingStorage> = Once::new();

fn ensure_mappings_init() {
    if MAPPINGS_INIT.load(Ordering::Acquire) == 0
        && MAPPINGS_INIT
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    {
        let v_arr: [core::sync::atomic::AtomicUsize; MAX_MAPPINGS] =
            [const { core::sync::atomic::AtomicUsize::new(0) }; MAX_MAPPINGS];
        let p_arr: [core::sync::atomic::AtomicUsize; MAX_MAPPINGS] =
            [const { core::sync::atomic::AtomicUsize::new(0) }; MAX_MAPPINGS];
        let ok_v = MAPPINGS_V_ONCE.set(MappingStorage(UnsafeCell::new(v_arr)));
        debug_assert!(ok_v);
        let ok_p = MAPPINGS_P_ONCE.set(MappingStorage(UnsafeCell::new(p_arr)));
        debug_assert!(ok_p);
    }
}

pub fn map_page(pt: &mut PageTable, v: VirtAddr, f: Frame) -> bool {
    let v_us = v.as_usize();
    let p_us = f.as_usize();
    ensure_mappings_init();
    let v_arr = match MAPPINGS_V_ONCE.get() {
        Some(s) => unsafe { &*s.0.get() },
        None => return false,
    };
    let p_arr = match MAPPINGS_P_ONCE.get() {
        Some(s) => unsafe { &*s.0.get() },
        None => return false,
    };
    for i in 0..MAX_MAPPINGS {
        let cur = v_arr[i].load(Ordering::Acquire);
        if cur == v_us {
            p_arr[i].store(p_us, Ordering::Release);
            return true;
        }
    }
    for i in 0..MAX_MAPPINGS {
        let cur = v_arr[i].load(Ordering::Acquire);
        if cur == 0 {
            v_arr[i].store(v_us, Ordering::Release);
            p_arr[i].store(p_us, Ordering::Release);
            let root = pt.root.as_usize();
            if root != 0 {
                if let Some(virt) = phys_to_virt_addr(root) {
                    unsafe {
                        let table_ptr = virt as *mut u64;
                        let entries = core::slice::from_raw_parts_mut(table_ptr, 4096 / 8);
                        for entry in entries.iter_mut() {
                            if *entry == 0 {
                                *entry = (p_us as u64) | 0x3u64;
                                break;
                            }
                        }
                    }
                }
            }
            return true;
        }
    }
    false
}

pub fn unmap_page(pt: &mut PageTable, v: VirtAddr) {
    let v_us = v.as_usize();
    ensure_mappings_init();
    let v_arr = match MAPPINGS_V_ONCE.get() {
        Some(s) => unsafe { &*s.0.get() },
        None => return,
    };
    let p_arr = match MAPPINGS_P_ONCE.get() {
        Some(s) => unsafe { &*s.0.get() },
        None => return,
    };
    for i in 0..MAX_MAPPINGS {
        let cur = v_arr[i].load(Ordering::Acquire);
        if cur == v_us {
            v_arr[i].store(0, Ordering::Release);
            p_arr[i].store(0, Ordering::Release);
            let root = pt.root.as_usize();
            if root != 0 {
                if let Some(virt) = phys_to_virt_addr(root) {
                    unsafe {
                        let table_ptr = virt as *mut u64;
                        let entries = core::slice::from_raw_parts_mut(table_ptr, 4096 / 8);
                        for entry in entries.iter_mut() {
                            let phys_in_entry = *entry & 0xFFFF_FFFF_F000;
                            if phys_in_entry != 0 && *entry & 0x3 != 0 {
                                *entry = 0;
                                break;
                            }
                        }
                    }
                }
            }
            unsafe {
                core::ptr::write_volatile(
                    v_us as *mut u8,
                    core::ptr::read_volatile(v_us as *const u8),
                );
                core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
            }
            return;
        }
    }
}
