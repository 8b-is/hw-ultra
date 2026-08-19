use crate::common::guard::Guard;
use core::sync::atomic::{AtomicU8, Ordering};

static SHIM_INITED: AtomicU8 = AtomicU8::new(0);

fn ensure_shims() {
    if SHIM_INITED.load(Ordering::Acquire) == 0 {
        crate::arch::init_shims();
        SHIM_INITED.store(1, Ordering::Release);
    }
}

static CPU_GUARD: Guard = Guard::new(1024);
static MMIO_GUARD: Guard = Guard::new(10000);

pub fn read_cpuid(leaf: u32, subleaf: u32) -> Option<(u32, u32, u32, u32)> {
    if !CPU_GUARD.allow() {
        return None;
    }
    ensure_shims();
    crate::arch::cpuid_count(leaf, subleaf)
}

pub fn read_msr(msr: u32) -> Option<u64> {
    if !CPU_GUARD.allow() {
        return None;
    }
    ensure_shims();
    crate::arch::read_msr(msr)
}

pub fn mmio_read32(addr: usize) -> Option<u32> {
    if !MMIO_GUARD.allow() {
        return None;
    }
    ensure_shims();
    crate::arch::mmio_read32(addr)
}

pub fn mmio_write32(addr: usize, val: u32) -> bool {
    if !MMIO_GUARD.allow() {
        return false;
    }
    ensure_shims();
    crate::arch::mmio_write32(addr, val)
}

pub fn read_aarch64_midr() -> Option<u64> {
    if !CPU_GUARD.allow() {
        return None;
    }
    ensure_shims();
    crate::arch::read_aarch64_midr()
}
