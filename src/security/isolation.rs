use core::sync::atomic::{AtomicU8, Ordering};

const MAX_DOMAINS: usize = 16;

static DOMAIN_COUNT: AtomicU8 = AtomicU8::new(0);
static DOMAIN_FLAGS: [AtomicU8; MAX_DOMAINS] = [const { AtomicU8::new(0) }; MAX_DOMAINS];

pub struct IsolationDomain {
    pub id: u8,
    pub level: IsolationLevel,
}

#[derive(Copy, Clone, PartialEq)]
pub enum IsolationLevel {
    None,
    Process,
    Hardware,
    Enclave,
}

pub fn create_domain(level: IsolationLevel) -> Option<IsolationDomain> {
    let idx = DOMAIN_COUNT.fetch_add(1, Ordering::AcqRel);
    if idx as usize >= MAX_DOMAINS {
        DOMAIN_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    DOMAIN_FLAGS[idx as usize].store(level as u8, Ordering::Release);
    Some(IsolationDomain { id: idx, level })
}

pub fn domain_count() -> u8 {
    DOMAIN_COUNT.load(Ordering::Acquire)
}

pub fn domain_level(id: u8) -> IsolationLevel {
    if (id as usize) >= MAX_DOMAINS {
        return IsolationLevel::None;
    }
    match DOMAIN_FLAGS[id as usize].load(Ordering::Acquire) {
        1 => IsolationLevel::Process,
        2 => IsolationLevel::Hardware,
        3 => IsolationLevel::Enclave,
        _ => IsolationLevel::None,
    }
}

pub fn isolate() {
    match crate::arch::detect_arch() {
        crate::arch::Architecture::X86_64 => {
            let spec = crate::security::speculation::mitigations_active();
            if spec {
                create_domain(IsolationLevel::Hardware);
            }
        }
        crate::arch::Architecture::AArch64 => {
            create_domain(IsolationLevel::Process);
        }
        _ => {}
    }
}
