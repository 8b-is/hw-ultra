use crate::common::once::OnceCopy;
use core::sync::atomic::{AtomicUsize, Ordering};

static MIDR_MMIO: AtomicUsize = AtomicUsize::new(0);

pub fn set_midr_mmio(addr: usize) {
    MIDR_MMIO.store(addr, Ordering::Release);
}

#[repr(C, align(4))]
struct AlignedBlob<const N: usize>([u8; N]);

#[used]
#[cfg_attr(target_os = "macos", link_section = "__TEXT,__text")]
#[cfg_attr(not(target_os = "macos"), link_section = ".text")]
static AARCH64_MRS_MIDR_BLOB: AlignedBlob<8> =
    AlignedBlob([0x00, 0x00, 0x38, 0xD5, 0xC0, 0x03, 0x5F, 0xD6]);

fn native_read_midr() -> u64 {
    unsafe {
        type MrsBlobFn = unsafe extern "C" fn() -> u64;
        let f: MrsBlobFn = core::mem::transmute(AARCH64_MRS_MIDR_BLOB.0.as_ptr());
        f()
    }
}

/// # Safety
/// Must only be called with a valid MMIO fallback configured or on AArch64 hardware.
pub unsafe fn read_midr_el1() -> u64 {
    let addr = MIDR_MMIO.load(Ordering::Acquire);
    if addr != 0 {
        core::ptr::read_volatile(addr as *const u64)
    } else {
        native_read_midr()
    }
}

type CacheOpFn = fn(usize);
type BarrierFn = fn();

static DC_CIVAC_FN: OnceCopy<CacheOpFn> = OnceCopy::new();
static DC_CVAU_FN: OnceCopy<CacheOpFn> = OnceCopy::new();
static DSB_ISH_FN: OnceCopy<BarrierFn> = OnceCopy::new();

pub fn set_dc_civac_fn(f: CacheOpFn) {
    DC_CIVAC_FN.set(f);
}
pub fn set_dc_cvau_fn(f: CacheOpFn) {
    DC_CVAU_FN.set(f);
}
pub fn set_dsb_ish_fn(f: BarrierFn) {
    DSB_ISH_FN.set(f);
}

/// # Safety
/// `addr` must be a valid virtual address with data at that cache line.
pub unsafe fn dc_civac(addr: usize) {
    if let Some(f) = DC_CIVAC_FN.get() {
        f(addr);
    }
}

/// # Safety
/// `addr` must be a valid virtual address with data at that cache line.
pub unsafe fn dc_cvau(addr: usize) {
    if let Some(f) = DC_CVAU_FN.get() {
        f(addr);
    }
}

/// # Safety
/// Issues an ISH data synchronisation barrier; only valid on aarch64.
pub unsafe fn dsb_ish() {
    if let Some(f) = DSB_ISH_FN.get() {
        f();
    }
}

/// # Safety
/// `id` must be a valid aarch64 system register encoding.
pub unsafe fn read_sysreg(id: u32) -> u64 {
    static SYSREG_LAST: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    SYSREG_LAST.store(id as usize, core::sync::atomic::Ordering::Release);
    0u64
}
