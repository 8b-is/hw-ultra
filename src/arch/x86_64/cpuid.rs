use core::sync::atomic::{AtomicUsize, Ordering};

static CPUID_MMIO: AtomicUsize = AtomicUsize::new(0);

pub fn set_cpuid_mmio(addr: usize) {
    CPUID_MMIO.store(addr, Ordering::Release);
}

pub fn cpuid_count(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    let base = CPUID_MMIO.load(Ordering::Acquire);
    if base != 0 {
        unsafe {
            core::ptr::write_volatile(base as *mut u32, leaf);
            core::ptr::write_volatile((base + 4) as *mut u32, subleaf);
            (
                core::ptr::read_volatile((base + 8) as *const u32),
                core::ptr::read_volatile((base + 12) as *const u32),
                core::ptr::read_volatile((base + 16) as *const u32),
                core::ptr::read_volatile((base + 20) as *const u32),
            )
        }
    } else {
        native_cpuid(leaf, subleaf)
    }
}

#[repr(C, align(4))]
struct AlignedBlob<const N: usize>([u8; N]);

#[used]
#[cfg_attr(target_os = "macos", link_section = "__TEXT,__text")]
#[cfg_attr(not(target_os = "macos"), link_section = ".text")]
static X86_64_CPUID_BLOB: AlignedBlob<27> = AlignedBlob([
    0x53, 0x89, 0xF8, 0x89, 0xF1, 0x49, 0x89, 0xD0, 0x0F, 0xA2, 0x41, 0x89, 0x00, 0x41, 0x89, 0x58,
    0x04, 0x41, 0x89, 0x48, 0x08, 0x41, 0x89, 0x50, 0x0C, 0x5B, 0xC3,
]);

fn native_cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    let mut out = [0u32; 4];
    unsafe {
        type CpuidBlobFn = unsafe extern "C" fn(u32, u32, *mut u32);
        let f: CpuidBlobFn = core::mem::transmute(X86_64_CPUID_BLOB.0.as_ptr());
        f(leaf, subleaf, out.as_mut_ptr());
    }
    (out[0], out[1], out[2], out[3])
}
