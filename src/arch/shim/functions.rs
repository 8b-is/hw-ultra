use crate::common::once::OnceCopy;
use core::sync::atomic::{AtomicU8, Ordering};

type CpuidFn = fn(u32, u32) -> Option<(u32, u32, u32, u32)>;
type ReadMsrFn = fn(u32) -> Option<u64>;
type MmioRead32Fn = fn(usize) -> Option<u32>;
type MmioWrite32Fn = fn(usize, u32) -> bool;
type ReadAarch64MidrFn = fn() -> Option<u64>;
type MkdirFn = fn(&[u8], u32) -> i64;
type ScanDirFn = fn(&[u8], &mut [crate::sys::DirEntry]) -> usize;

static CPUID_FN: OnceCopy<CpuidFn> = OnceCopy::new();
static READ_MSR_FN: OnceCopy<ReadMsrFn> = OnceCopy::new();
static MMIO_READ32_FN: OnceCopy<MmioRead32Fn> = OnceCopy::new();
static MMIO_WRITE32_FN: OnceCopy<MmioWrite32Fn> = OnceCopy::new();
static READ_AARCH64_MIDR_FN: OnceCopy<ReadAarch64MidrFn> = OnceCopy::new();
static MKDIR_FN: OnceCopy<MkdirFn> = OnceCopy::new();
static SCAN_DIR_FN: OnceCopy<ScanDirFn> = OnceCopy::new();

static SHIMS_INIT: AtomicU8 = AtomicU8::new(0);

pub fn init_shims() {
    if SHIMS_INIT
        .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return;
    }
    crate::arch::x86_64::init_shim();
    crate::arch::aarch64::init_shim();
    super::syscall::register_native_syscall();
}

fn default_cpuid(l: u32, s: u32) -> Option<(u32, u32, u32, u32)> {
    init_shims();
    if let Some(f) = CPUID_FN.get() {
        return f(l, s);
    }
    None
}

fn default_read_msr(msr: u32) -> Option<u64> {
    init_shims();
    if let Some(f) = READ_MSR_FN.get() {
        return f(msr);
    }
    None
}

fn default_mmio_read32(addr: usize) -> Option<u32> {
    init_shims();
    if let Some(f) = MMIO_READ32_FN.get() {
        return f(addr);
    }
    None
}

fn default_mmio_write32(addr: usize, val: u32) -> bool {
    init_shims();
    if let Some(f) = MMIO_WRITE32_FN.get() {
        return f(addr, val);
    }
    false
}

fn default_read_aarch64_midr() -> Option<u64> {
    init_shims();
    if let Some(f) = READ_AARCH64_MIDR_FN.get() {
        return f();
    }
    None
}

pub fn set_cpuid_fn(f: CpuidFn) {
    CPUID_FN.set(f);
}
pub fn set_read_msr_fn(f: ReadMsrFn) {
    READ_MSR_FN.set(f);
}
pub fn set_mmio_read32_fn(f: MmioRead32Fn) {
    MMIO_READ32_FN.set(f);
}
pub fn set_mmio_write32_fn(f: MmioWrite32Fn) {
    MMIO_WRITE32_FN.set(f);
}
pub fn set_read_aarch64_midr_fn(f: ReadAarch64MidrFn) {
    READ_AARCH64_MIDR_FN.set(f);
}

pub fn set_mkdir_fn(f: MkdirFn) {
    MKDIR_FN.set(f);
}
pub fn set_scan_dir_fn(f: ScanDirFn) {
    SCAN_DIR_FN.set(f);
}

pub fn cpuid_count(leaf: u32, subleaf: u32) -> Option<(u32, u32, u32, u32)> {
    if let Some(f) = CPUID_FN.get() {
        return f(leaf, subleaf);
    }
    default_cpuid(leaf, subleaf)
}

pub fn read_msr(msr: u32) -> Option<u64> {
    if let Some(f) = READ_MSR_FN.get() {
        f(msr)
    } else {
        default_read_msr(msr)
    }
}

pub fn mmio_read32(addr: usize) -> Option<u32> {
    if let Some(f) = MMIO_READ32_FN.get() {
        f(addr)
    } else {
        default_mmio_read32(addr)
    }
}

pub fn mmio_write32(addr: usize, val: u32) -> bool {
    if let Some(f) = MMIO_WRITE32_FN.get() {
        f(addr, val)
    } else {
        default_mmio_write32(addr, val)
    }
}

pub fn read_aarch64_midr() -> Option<u64> {
    if let Some(f) = READ_AARCH64_MIDR_FN.get() {
        f()
    } else {
        default_read_aarch64_midr()
    }
}

pub fn arch_exit(code: i32) -> ! {
    init_shims();
    unsafe {
        super::syscall::raw_syscall(super::syscall::nr_exit(), code as u64, 0, 0, 0, 0, 0);
    }
    loop {
        core::hint::spin_loop();
    }
}

pub fn mkdir(path: &[u8], mode: u32) -> i64 {
    if let Some(f) = MKDIR_FN.get() {
        return f(path, mode);
    }
    let nr = super::syscall::nr_mkdirat();
    if nr == crate::common::error::ERR_NOT_IMPLEMENTED {
        return crate::common::error::ERR_NOT_IMPLEMENTED;
    }
    unsafe {
        super::syscall::raw_syscall(
            nr,
            super::os::os_at_fdcwd() as u64,
            path.as_ptr() as u64,
            mode as u64,
            0,
            0,
            0,
        )
    }
}

pub fn scan_dir_dispatch(path: &[u8], out: &mut [crate::sys::DirEntry]) -> usize {
    if let Some(f) = SCAN_DIR_FN.get() {
        return f(path, out);
    }
    0
}
