use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const MAX_DOMAINS: usize = 16;

static DOMAIN_COUNT: AtomicU8 = AtomicU8::new(0);
static DOMAIN_BASE: [AtomicUsize; MAX_DOMAINS] = [const { AtomicUsize::new(0) }; MAX_DOMAINS];
static DOMAIN_SIZE: [AtomicUsize; MAX_DOMAINS] = [const { AtomicUsize::new(0) }; MAX_DOMAINS];
static DOMAIN_FLAGS: [AtomicU8; MAX_DOMAINS] = [const { AtomicU8::new(0) }; MAX_DOMAINS];

#[derive(Copy, Clone)]
pub struct Domain {
    pub id: u8,
    pub base: usize,
    pub size: usize,
    pub flags: u8,
}

pub fn create_domain(base: usize, size: usize, flags: u8) -> Option<Domain> {
    let id = DOMAIN_COUNT.fetch_add(1, Ordering::AcqRel);
    if id as usize >= MAX_DOMAINS {
        DOMAIN_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    DOMAIN_BASE[id as usize].store(base, Ordering::Release);
    DOMAIN_SIZE[id as usize].store(size, Ordering::Release);
    DOMAIN_FLAGS[id as usize].store(flags, Ordering::Release);
    Some(Domain {
        id,
        base,
        size,
        flags,
    })
}

pub fn domain_info(id: u8) -> Option<Domain> {
    if id >= DOMAIN_COUNT.load(Ordering::Acquire) {
        return None;
    }
    let base = DOMAIN_BASE[id as usize].load(Ordering::Acquire);
    let size = DOMAIN_SIZE[id as usize].load(Ordering::Acquire);
    let flags = DOMAIN_FLAGS[id as usize].load(Ordering::Acquire);
    Some(Domain {
        id,
        base,
        size,
        flags,
    })
}

pub fn domain_count() -> u8 {
    DOMAIN_COUNT.load(Ordering::Acquire)
}
