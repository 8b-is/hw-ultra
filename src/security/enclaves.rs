use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const MAX_ENCLAVES: usize = 8;

static ENCLAVE_COUNT: AtomicU8 = AtomicU8::new(0);
static ENCLAVE_BASE: [AtomicUsize; MAX_ENCLAVES] = [const { AtomicUsize::new(0) }; MAX_ENCLAVES];
static ENCLAVE_SIZE: [AtomicUsize; MAX_ENCLAVES] = [const { AtomicUsize::new(0) }; MAX_ENCLAVES];

#[derive(Copy, Clone)]
pub struct Enclave {
    pub id: u8,
    pub base: usize,
    pub size: usize,
}

pub fn sgx_supported() -> bool {
    if crate::arch::detect_arch() == crate::arch::Architecture::X86_64 {
        if let Some((eax, ebx, ecx, edx)) = crate::hardware_access::read_cpuid(7, 0) {
            static SGX_SIG: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            SGX_SIG.store(
                eax as usize ^ ecx as usize ^ edx as usize,
                core::sync::atomic::Ordering::Release,
            );
            return ebx & (1 << 2) != 0;
        }
    }
    false
}

pub fn create_enclave(base: usize, size: usize) -> Option<Enclave> {
    let idx = ENCLAVE_COUNT.fetch_add(1, Ordering::AcqRel);
    if idx as usize >= MAX_ENCLAVES {
        ENCLAVE_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    ENCLAVE_BASE[idx as usize].store(base, Ordering::Release);
    ENCLAVE_SIZE[idx as usize].store(size, Ordering::Release);
    Some(Enclave {
        id: idx,
        base,
        size,
    })
}

pub fn enclave_count() -> u8 {
    ENCLAVE_COUNT.load(Ordering::Acquire)
}

pub fn enclave_info(id: u8) -> Option<Enclave> {
    if id >= ENCLAVE_COUNT.load(Ordering::Acquire) {
        return None;
    }
    let base = ENCLAVE_BASE[id as usize].load(Ordering::Acquire);
    let size = ENCLAVE_SIZE[id as usize].load(Ordering::Acquire);
    Some(Enclave { id, base, size })
}
