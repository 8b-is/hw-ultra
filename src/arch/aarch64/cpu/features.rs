use core::sync::atomic::{AtomicUsize, Ordering};

static FEATURE_REG_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_feature_reg_base(base: usize) {
    FEATURE_REG_BASE.store(base, Ordering::Release);
}

pub struct AArch64Features {
    pub neon: bool,
    pub sve: bool,
    pub sve2: bool,
    pub crypto: bool,
    pub crc32: bool,
    pub atomics: bool,
}

pub fn detect() -> AArch64Features {
    let base = FEATURE_REG_BASE.load(Ordering::Acquire);
    let (id_aa64isar0, id_aa64pfr0, id_aa64isar1) = if base != 0 {
        unsafe {
            (
                core::ptr::read_volatile(base as *const u64),
                core::ptr::read_volatile((base + 8) as *const u64),
                core::ptr::read_volatile((base + 16) as *const u64),
            )
        }
    } else {
        (0u64, 0u64, 0u64)
    };
    static ISAR_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    ISAR_SIG.store(id_aa64isar1 as usize, core::sync::atomic::Ordering::Release);
    AArch64Features {
        neon: ((id_aa64pfr0 >> 20) & 0xf) >= 1,
        sve: ((id_aa64pfr0 >> 32) & 0xf) >= 1,
        sve2: ((id_aa64pfr0 >> 32) & 0xf) >= 2,
        crypto: ((id_aa64isar0 >> 4) & 0xf) >= 1,
        crc32: ((id_aa64isar0 >> 16) & 0xf) >= 1,
        atomics: ((id_aa64isar0 >> 20) & 0xf) >= 2,
    }
}
