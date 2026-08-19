use crate::memory::phys::frame::Frame;
use crate::memory::virt::address::VirtAddr;

pub fn map_page(v: VirtAddr, f: Frame) -> bool {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            let mut pt = crate::arch::x86_64::mmu::paging::PageTable::new();
            crate::arch::x86_64::mmu::paging::map_page(&mut pt, v, f)
        }
        crate::arch::Architecture::AArch64 => {
            let mut pt = crate::arch::aarch64::mmu::PageTable::new();
            crate::arch::aarch64::mmu::map_page(&mut pt, v, f)
        }
        other => {
            static MAP_ARCH_SIG: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            MAP_ARCH_SIG.store(other as usize, core::sync::atomic::Ordering::Release);
            false
        }
    }
}

pub fn unmap_page(v: VirtAddr) {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            let mut pt = crate::arch::x86_64::mmu::paging::PageTable::new();
            crate::arch::x86_64::mmu::paging::unmap_page(&mut pt, v);
        }
        crate::arch::Architecture::AArch64 => {
            let mut pt = crate::arch::aarch64::mmu::PageTable::new();
            crate::arch::aarch64::mmu::unmap_page(&mut pt, v);
        }
        other => {
            static UNMAP_ARCH_SIG: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            UNMAP_ARCH_SIG.store(other as usize, core::sync::atomic::Ordering::Release);
        }
    }
}
