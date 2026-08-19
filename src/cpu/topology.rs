use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Topology {
    pub physical_cores: u8,
    pub logical_cores: u8,
    pub sockets: u8,
    pub threads_per_core: u8,
}

static TOPO_PHYS: AtomicUsize = AtomicUsize::new(0);
static TOPO_LOG: AtomicUsize = AtomicUsize::new(0);

pub fn detect() -> Topology {
    let topo = crate::topology::detect_topology();
    let phys = topo.cores_per_socket;
    let log = if let Some(info) = crate::cpu::get_info() {
        info.cores
    } else {
        phys
    };
    TOPO_PHYS.store(phys as usize, Ordering::Release);
    TOPO_LOG.store(log as usize, Ordering::Release);
    let tpc = if phys > 0 { log / phys } else { 1 };
    Topology {
        physical_cores: phys,
        logical_cores: log,
        sockets: topo.sockets,
        threads_per_core: if tpc == 0 { 1 } else { tpc },
    }
}
