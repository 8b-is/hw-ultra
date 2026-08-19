use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const MAX_LINKS: usize = 16;

static LINK_COUNT: AtomicU8 = AtomicU8::new(0);
static LINK_SRC: [AtomicU8; MAX_LINKS] = [const { AtomicU8::new(0) }; MAX_LINKS];
static LINK_DST: [AtomicU8; MAX_LINKS] = [const { AtomicU8::new(0) }; MAX_LINKS];
static LINK_BW: [AtomicUsize; MAX_LINKS] = [const { AtomicUsize::new(0) }; MAX_LINKS];
static LINK_LATENCY: [AtomicUsize; MAX_LINKS] = [const { AtomicUsize::new(0) }; MAX_LINKS];

#[derive(Copy, Clone)]
pub struct Interconnect {
    pub id: u8,
    pub src_node: u8,
    pub dst_node: u8,
    pub bandwidth_mbps: usize,
    pub latency_ns: usize,
}

pub fn register_link(
    src: u8,
    dst: u8,
    bandwidth_mbps: usize,
    latency_ns: usize,
) -> Option<Interconnect> {
    let id = LINK_COUNT.fetch_add(1, Ordering::AcqRel);
    if id as usize >= MAX_LINKS {
        LINK_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    LINK_SRC[id as usize].store(src, Ordering::Release);
    LINK_DST[id as usize].store(dst, Ordering::Release);
    LINK_BW[id as usize].store(bandwidth_mbps, Ordering::Release);
    LINK_LATENCY[id as usize].store(latency_ns, Ordering::Release);
    Some(Interconnect {
        id,
        src_node: src,
        dst_node: dst,
        bandwidth_mbps,
        latency_ns,
    })
}

pub fn link_info(id: u8) -> Option<Interconnect> {
    if id >= LINK_COUNT.load(Ordering::Acquire) {
        return None;
    }
    Some(Interconnect {
        id,
        src_node: LINK_SRC[id as usize].load(Ordering::Acquire),
        dst_node: LINK_DST[id as usize].load(Ordering::Acquire),
        bandwidth_mbps: LINK_BW[id as usize].load(Ordering::Acquire),
        latency_ns: LINK_LATENCY[id as usize].load(Ordering::Acquire),
    })
}

pub fn link_count() -> u8 {
    LINK_COUNT.load(Ordering::Acquire)
}
