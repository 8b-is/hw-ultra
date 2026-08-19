use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static SOCKET_COUNT: AtomicU8 = AtomicU8::new(0);
static TOTAL_CORES: AtomicUsize = AtomicUsize::new(0);
static TOTAL_THREADS: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct SystemTopology {
    pub sockets: u8,
    pub total_cores: usize,
    pub total_threads: usize,
}

pub fn detect() -> SystemTopology {
    let topo = crate::topology::detect_topology();
    let cores = (topo.sockets as usize) * (topo.cores_per_socket as usize);
    SOCKET_COUNT.store(topo.sockets, Ordering::Release);
    TOTAL_CORES.store(cores, Ordering::Release);

    let threads = if let Some(info) = crate::cpu::api::detect_cpu_info() {
        info.logical_cores as usize
    } else {
        cores
    };

    TOTAL_THREADS.store(threads, Ordering::Release);
    SystemTopology {
        sockets: topo.sockets,
        total_cores: cores,
        total_threads: threads,
    }
}

pub fn cached_topology() -> SystemTopology {
    SystemTopology {
        sockets: SOCKET_COUNT.load(Ordering::Acquire),
        total_cores: TOTAL_CORES.load(Ordering::Acquire),
        total_threads: TOTAL_THREADS.load(Ordering::Acquire),
    }
}
