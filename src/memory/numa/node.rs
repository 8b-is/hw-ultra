use core::sync::atomic::{AtomicUsize, Ordering};

pub struct NumaNode {
    pub id: u8,
    pub memory_base: usize,
    pub memory_size: usize,
    pub cpu_mask: usize,
}

const MAX_NODES: usize = 8;

struct NodeTable {
    bases: [AtomicUsize; MAX_NODES],
    sizes: [AtomicUsize; MAX_NODES],
    masks: [AtomicUsize; MAX_NODES],
    count: AtomicUsize,
}

unsafe impl Sync for NodeTable {}

static NODES: NodeTable = NodeTable {
    bases: [const { AtomicUsize::new(0) }; MAX_NODES],
    sizes: [const { AtomicUsize::new(0) }; MAX_NODES],
    masks: [const { AtomicUsize::new(0) }; MAX_NODES],
    count: AtomicUsize::new(0),
};

pub fn register_node(node: &NumaNode) -> bool {
    let idx = node.id as usize;
    if idx >= MAX_NODES {
        return false;
    }
    NODES.bases[idx].store(node.memory_base, Ordering::Release);
    NODES.sizes[idx].store(node.memory_size, Ordering::Release);
    NODES.masks[idx].store(node.cpu_mask, Ordering::Release);
    let cur = NODES.count.load(Ordering::Acquire);
    if idx >= cur {
        NODES.count.store(idx + 1, Ordering::Release);
    }
    true
}

pub fn detect_nodes() -> usize {
    let total = crate::boot::total_usable_ram();
    let node = NumaNode {
        id: 0,
        memory_base: 0,
        memory_size: total,
        cpu_mask: 1,
    };
    let ok = register_node(&node);
    static REG_SIG: AtomicUsize = AtomicUsize::new(0);
    REG_SIG.store(ok as usize, Ordering::Release);
    1
}

pub fn node_count() -> usize {
    NODES.count.load(Ordering::Acquire)
}
