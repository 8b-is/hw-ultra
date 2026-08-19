use crate::common::once::OnceCopy;
use crate::memory::phys::frame::Frame;
use crate::memory::virt::address::VirtAddr;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type PhysToVirtFn = unsafe fn(usize) -> *const u8;
static PHYS_TO_VIRT: OnceCopy<PhysToVirtFn> = OnceCopy::new();
pub fn set_phys_to_virt_cb(f: PhysToVirtFn) {
    PHYS_TO_VIRT.set(f);
}

#[inline]
fn phys_to_virt(paddr: usize) -> *mut u8 {
    if let Some(func) = PHYS_TO_VIRT.get() {
        unsafe { func(paddr) as *mut u8 }
    } else {
        default_phys_to_virt(paddr)
    }
}

fn default_phys_to_virt(paddr: usize) -> *mut u8 {
    const FALLBACK_HIGHER_HALF: usize = 0xffff_8000_0000_0000usize;
    let base = PHYS_DIRECT_MAP_BASE.load(Ordering::Acquire);
    let map_base = if base == 0 {
        FALLBACK_HIGHER_HALF
    } else {
        base
    };
    (map_base + paddr) as *mut u8
}

static PHYS_DIRECT_MAP_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_phys_direct_map_base(base: usize) {
    PHYS_DIRECT_MAP_BASE.store(base, Ordering::Release);
}

pub fn phys_to_virt_addr(paddr: usize) -> usize {
    phys_to_virt(paddr) as usize
}

pub struct PageTable {
    root: Frame,
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

impl PageTable {
    pub fn new() -> Self {
        if let Some(f) = crate::memory::phys::allocator::PhysAllocator::alloc_frame() {
            PageTable { root: f }
        } else {
            PageTable {
                root: Frame::new(0),
            }
        }
    }
    pub fn root_frame(&self) -> Frame {
        self.root
    }
}

pub const ENTRIES: usize = 512;
pub const ENTRY_SIZE: usize = 8;

pub const P_PRESENT: u64 = 1 << 0;
pub const P_WRITABLE: u64 = 1 << 1;
pub const P_USER: u64 = 1 << 2;
pub const P_PWT: u64 = 1 << 3;
pub const P_PCD: u64 = 1 << 4;
pub const P_ACCESSED: u64 = 1 << 5;
pub const P_DIRTY: u64 = 1 << 6;
pub const P_HUGE: u64 = 1 << 7;

fn idx_from_va(v: usize) -> [usize; 4] {
    [
        (v >> 39) & 0x1ff,
        (v >> 30) & 0x1ff,
        (v >> 21) & 0x1ff,
        (v >> 12) & 0x1ff,
    ]
}

unsafe fn read_entry(table_frame: usize, index: usize) -> u64 {
    let base = phys_to_virt(table_frame) as *mut u64;
    if base.is_null() {
        return 0;
    }
    core::ptr::read_volatile(base.add(index))
}

unsafe fn write_entry(table_frame: usize, index: usize, val: u64) {
    let base = phys_to_virt(table_frame) as *mut u64;
    if base.is_null() {
        return;
    }
    core::ptr::write_volatile(base.add(index), val);
}

fn allocate_table() -> Option<Frame> {
    crate::memory::phys::allocator::PhysAllocator::alloc_frame()
}

pub fn map_page(pt: &mut PageTable, v: VirtAddr, f: Frame) -> bool {
    let va = v.as_usize();
    let pa = f.as_usize();
    if pt.root.as_usize() == 0 {
        return false;
    }

    let idxs = idx_from_va(va);
    let mut cur_frame = pt.root.as_usize();

    for (level, &idx) in idxs.iter().enumerate() {
        unsafe {
            let entry = read_entry(cur_frame, idx);
            if level == 3 {
                let pte = ((pa as u64) & !0xfffu64) | P_PRESENT | P_WRITABLE;
                write_entry(cur_frame, idx, pte);
                if crate::arch::detect_arch() == crate::arch::Architecture::X86_64 {
                    core::ptr::write_volatile(
                        va as *mut u8,
                        core::ptr::read_volatile(va as *const u8),
                    );
                    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
                }
                return true;
            }
            if (entry & P_PRESENT) == 0 {
                if let Some(newf) = allocate_table() {
                    let new_pa = newf.as_usize() as u64;
                    let val = (new_pa & !0xfffu64) | P_PRESENT | P_WRITABLE;
                    write_entry(cur_frame, idx, val);
                    cur_frame = newf.as_usize();
                } else {
                    return false;
                }
            } else {
                let next_pa = (entry & !0xfffu64) as usize;
                cur_frame = next_pa;
            }
        }
    }
    false
}

pub fn unmap_page(pt: &mut PageTable, v: VirtAddr) {
    let va = v.as_usize();
    if pt.root.as_usize() == 0 {
        return;
    }
    let idxs = idx_from_va(va);
    let mut frames: [usize; 5] = [0; 5];
    frames[0] = pt.root.as_usize();

    for level in 0..4 {
        unsafe {
            let e = read_entry(frames[level], idxs[level]);
            if (e & P_PRESENT) == 0 {
                return;
            }
            if level < 3 {
                frames[level + 1] = (e & !0xfffu64) as usize;
            }
        }
    }

    unsafe {
        write_entry(frames[3], idxs[3], 0);
    }
    if crate::arch::detect_arch() == crate::arch::Architecture::X86_64 {
        unsafe {
            core::ptr::write_volatile(va as *mut u8, core::ptr::read_volatile(va as *const u8));
            core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        }
    }

    for level in (0..3).rev() {
        let frame = frames[level + 1];
        unsafe {
            let base = phys_to_virt(frame) as *mut u64;
            if base.is_null() {
                break;
            }
            let mut all_zero = true;
            for i in 0..ENTRIES {
                if core::ptr::read_volatile(base.add(i)) != 0 {
                    all_zero = false;
                    break;
                }
            }
            if all_zero {
                write_entry(frames[level], idxs[level], 0);
                crate::memory::phys::allocator::PhysAllocator::free_frame(Frame::new(frame));
            } else {
                break;
            }
        }
    }
}
