use core::sync::atomic::{AtomicU32, Ordering};

static FEATURES: AtomicU32 = AtomicU32::new(0);

pub const FEATURE_SSE: u32 = 1 << 0;
pub const FEATURE_AVX: u32 = 1 << 1;
pub const FEATURE_NEON: u32 = 1 << 2;
pub const FEATURE_IOMMU: u32 = 1 << 3;
pub const FEATURE_DMA: u32 = 1 << 4;
pub const FEATURE_GPU: u32 = 1 << 5;
pub const FEATURE_TPU: u32 = 1 << 6;
pub const FEATURE_LPU: u32 = 1 << 7;
pub const FEATURE_ACPI: u32 = 1 << 8;
pub const FEATURE_UEFI: u32 = 1 << 9;
pub const FEATURE_SMBIOS: u32 = 1 << 10;
pub const FEATURE_DEVICETREE: u32 = 1 << 11;
pub const FEATURE_HPET: u32 = 1 << 12;
pub const FEATURE_PCIE_ECAM: u32 = 1 << 13;
pub const FEATURE_VTD: u32 = 1 << 14;
pub const FEATURE_GOP: u32 = 1 << 15;

pub fn enable(feature: u32) {
    FEATURES.fetch_or(feature, Ordering::AcqRel);
}

pub fn disable(feature: u32) {
    FEATURES.fetch_and(!feature, Ordering::AcqRel);
}

pub fn is_enabled(feature: u32) -> bool {
    FEATURES.load(Ordering::Acquire) & feature != 0
}

pub fn all_enabled() -> u32 {
    FEATURES.load(Ordering::Acquire)
}

pub fn set_all(mask: u32) {
    FEATURES.store(mask, Ordering::Release);
}
