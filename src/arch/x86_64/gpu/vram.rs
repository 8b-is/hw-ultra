use core::sync::atomic::{AtomicUsize, Ordering};

const WC_MTRR_BASE_MSR: u32 = 0x200;
const WC_MTRR_MASK_MSR: u32 = 0x201;
const PAT_MSR: u32 = 0x277;
const MTRR_TYPE_WC: u64 = 0x01;

static VRAM_IOVA_BASE: AtomicUsize = AtomicUsize::new(0);
static VRAM_SIZE: AtomicUsize = AtomicUsize::new(0);
static VRAM_PAT_SLOT: AtomicUsize = AtomicUsize::new(0);

pub fn map_vram_region(phys_base: usize, size: usize) -> usize {
    if let Some(ctrl) = crate::iommu::controller::get() {
        if ctrl.is_enabled() {
            if let Some(iova) = ctrl.translate_iova(phys_base) {
                VRAM_IOVA_BASE.store(iova, Ordering::Release);
                VRAM_SIZE.store(size, Ordering::Release);
                return iova;
            }
        }
    }
    VRAM_IOVA_BASE.store(phys_base, Ordering::Release);
    VRAM_SIZE.store(size, Ordering::Release);
    phys_base
}

pub fn set_write_combining(phys_base: usize, size: usize) {
    if size == 0 || phys_base == 0 {
        return;
    }
    let pair_index = VRAM_PAT_SLOT.fetch_add(1, Ordering::AcqRel);
    if pair_index >= 4 {
        return;
    }
    let base_msr = WC_MTRR_BASE_MSR + (pair_index as u32) * 2;
    let mask_msr = WC_MTRR_MASK_MSR + (pair_index as u32) * 2;

    let base_val = (phys_base as u64 & 0xFFFF_FFFF_FFFF_F000) | MTRR_TYPE_WC;
    let mask_bits = !(size as u64 - 1) & 0xFFFF_FFFF_FFFF_F000;
    let mask_val = mask_bits | (1u64 << 11);

    unsafe {
        crate::arch::x86_64::msr::write_msr(base_msr, base_val);
        crate::arch::x86_64::msr::write_msr(mask_msr, mask_val);
    }
}

pub fn read_pat() -> u64 {
    crate::hardware_access::api::read_msr(PAT_MSR).unwrap_or(0)
}

pub fn vram_iova() -> usize {
    VRAM_IOVA_BASE.load(Ordering::Acquire)
}

pub fn vram_size() -> usize {
    VRAM_SIZE.load(Ordering::Acquire)
}

pub fn allocate_framebuffer(offset: usize, size: usize) -> Option<usize> {
    let base = VRAM_IOVA_BASE.load(Ordering::Acquire);
    let total = VRAM_SIZE.load(Ordering::Acquire);
    if base == 0 || offset + size > total {
        return None;
    }
    Some(base + offset)
}
