use core::sync::atomic::{AtomicUsize, Ordering};

static VRAM_IOVA_BASE: AtomicUsize = AtomicUsize::new(0);
static VRAM_SIZE: AtomicUsize = AtomicUsize::new(0);

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

pub fn clean_cache_range(va_start: usize, size: usize) {
    let line_size = 64usize;
    let mut addr = va_start & !(line_size - 1);
    let end = va_start + size;
    if crate::arch::detect_arch() == crate::arch::architecture::Architecture::AArch64 {
        while addr < end {
            unsafe { super::super::sysreg::dc_cvau(addr) }
            addr += line_size;
        }
        unsafe { super::super::sysreg::dsb_ish() }
    } else {
        while addr < end {
            unsafe {
                core::ptr::write_volatile(
                    addr as *mut u8,
                    core::ptr::read_volatile(addr as *const u8),
                );
            }
            addr += line_size;
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    }
}

pub fn invalidate_cache_range(va_start: usize, size: usize) {
    let line_size = 64usize;
    let mut addr = va_start & !(line_size - 1);
    let end = va_start + size;
    if crate::arch::detect_arch() == crate::arch::architecture::Architecture::AArch64 {
        while addr < end {
            unsafe { super::super::sysreg::dc_civac(addr) }
            addr += line_size;
        }
        unsafe { super::super::sysreg::dsb_ish() }
    } else {
        while addr < end {
            unsafe {
                core::ptr::write_volatile(
                    addr as *mut u8,
                    core::ptr::read_volatile(addr as *const u8),
                );
            }
            addr += line_size;
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    }
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
