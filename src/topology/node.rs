use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const MAX_NODES: usize = 8;

static NODE_COUNT: AtomicU8 = AtomicU8::new(0);
static NODE_CORES: [AtomicU8; MAX_NODES] = [const { AtomicU8::new(0) }; MAX_NODES];
static NODE_MEM_BASE: [AtomicUsize; MAX_NODES] = [const { AtomicUsize::new(0) }; MAX_NODES];
static NODE_MEM_SIZE: [AtomicUsize; MAX_NODES] = [const { AtomicUsize::new(0) }; MAX_NODES];

#[derive(Copy, Clone)]
pub struct Node {
    pub id: u8,
    pub core_count: u8,
    pub mem_base: usize,
    pub mem_size: usize,
}

pub fn register_node(core_count: u8, mem_base: usize, mem_size: usize) -> Option<Node> {
    let id = NODE_COUNT.fetch_add(1, Ordering::AcqRel);
    if id as usize >= MAX_NODES {
        NODE_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    NODE_CORES[id as usize].store(core_count, Ordering::Release);
    NODE_MEM_BASE[id as usize].store(mem_base, Ordering::Release);
    NODE_MEM_SIZE[id as usize].store(mem_size, Ordering::Release);
    Some(Node {
        id,
        core_count,
        mem_base,
        mem_size,
    })
}

pub fn node_info(id: u8) -> Option<Node> {
    if id >= NODE_COUNT.load(Ordering::Acquire) {
        return None;
    }
    let core_count = NODE_CORES[id as usize].load(Ordering::Acquire);
    let mem_base = NODE_MEM_BASE[id as usize].load(Ordering::Acquire);
    let mem_size = NODE_MEM_SIZE[id as usize].load(Ordering::Acquire);
    Some(Node {
        id,
        core_count,
        mem_base,
        mem_size,
    })
}

pub fn node_count() -> u8 {
    NODE_COUNT.load(Ordering::Acquire)
}
