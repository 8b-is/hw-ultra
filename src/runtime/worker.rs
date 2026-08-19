use super::monitor::{self, Component};
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicUsize, Ordering};

const MAX_WORKERS: usize = 5;
const POLL_INTERVAL_MS: u64 = 250;
const THROTTLE_COOLDOWN_MS: u64 = 1000;

static WORKERS_RUNNING: AtomicBool = AtomicBool::new(false);
static WORKER_PIDS: [AtomicI32; MAX_WORKERS] = [
    AtomicI32::new(0),
    AtomicI32::new(0),
    AtomicI32::new(0),
    AtomicI32::new(0),
    AtomicI32::new(0),
];
static WORKER_COUNT: AtomicUsize = AtomicUsize::new(0);

static POLL_COUNT: [AtomicUsize; MAX_WORKERS] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];
static THROTTLE_COUNT: [AtomicUsize; MAX_WORKERS] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];
static LAST_TEMP: [AtomicU32; MAX_WORKERS] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

fn component_index(comp: Component) -> usize {
    match comp {
        Component::Cpu => 0,
        Component::Ram => 1,
        Component::Gpu => 2,
        Component::Tpu => 3,
        Component::Lpu => 4,
    }
}

fn component_present(comp: Component) -> bool {
    match comp {
        Component::Cpu => true,
        Component::Ram => crate::memory::info::detect_memory_info().is_some(),
        Component::Gpu => super::handle::gpu_handle().is_some(),
        Component::Tpu => super::handle::tpu_handle().is_some(),
        Component::Lpu => super::handle::lpu_handle().is_some(),
    }
}

fn worker_loop(comp: Component) -> ! {
    let idx = component_index(comp);
    let mut throttled_since: u64 = 0;

    loop {
        crate::sys::sleep_ns(POLL_INTERVAL_MS * 1_000_000);

        POLL_COUNT[idx].fetch_add(1, Ordering::Relaxed);

        let temp = monitor::read_temperature(comp);
        LAST_TEMP[idx].store(temp, Ordering::Relaxed);

        if matches!(comp, Component::Cpu) {
            monitor::record_cycles(comp, monitor::read_cycles(comp));
        }

        if monitor::is_throttled(comp) {
            let now = crate::sys::monotonic_ns();
            if throttled_since == 0 {
                throttled_since = now;
            }
            THROTTLE_COUNT[idx].fetch_add(1, Ordering::Relaxed);

            let elapsed = (now - throttled_since) / 1_000_000;
            if elapsed > THROTTLE_COOLDOWN_MS {
                let cur_freq = monitor::frequency(comp);
                if cur_freq > 0 {
                    let reduced = cur_freq - cur_freq / 10;
                    let min = monitor::freq_min(comp);
                    let target = if reduced > min { reduced } else { min };
                    monitor::set_frequency(comp, target);
                }
            }
        } else if throttled_since > 0 {
            throttled_since = 0;
            let cur_freq = monitor::frequency(comp);
            let max_freq = monitor::freq_max(comp);
            if cur_freq < max_freq && max_freq > 0 {
                let restored = cur_freq + cur_freq / 20;
                let target = if restored < max_freq {
                    restored
                } else {
                    max_freq
                };
                monitor::set_frequency(comp, target);
            }
        }
    }
}

pub fn spawn_workers() -> usize {
    if WORKERS_RUNNING.swap(true, Ordering::AcqRel) {
        return WORKER_COUNT.load(Ordering::Acquire);
    }

    let components = [
        Component::Cpu,
        Component::Ram,
        Component::Gpu,
        Component::Tpu,
        Component::Lpu,
    ];

    let mut count = 0usize;
    let mut i = 0;
    while i < components.len() {
        let comp = components[i];
        if component_present(comp) {
            let pid = crate::sys::fork();
            if pid == 0 {
                worker_loop(comp);
            } else if pid > 0 {
                WORKER_PIDS[i].store(pid as i32, Ordering::Release);
                count += 1;
            }
        }
        i += 1;
    }

    WORKER_COUNT.store(count, Ordering::Release);
    count
}

pub fn stop_workers() {
    if !WORKERS_RUNNING.swap(false, Ordering::AcqRel) {
        return;
    }

    let mut i = 0;
    while i < MAX_WORKERS {
        let pid = WORKER_PIDS[i].swap(0, Ordering::AcqRel);
        if pid > 0 {
            crate::sys::sys_kill(pid, 9);
            crate::sys::waitpid(pid as i64);
            crate::runtime::resources::free_cpu(1);
        }
        i += 1;
    }

    WORKER_COUNT.store(0, Ordering::Release);
}

pub fn worker_count() -> usize {
    WORKER_COUNT.load(Ordering::Acquire)
}

pub fn is_running() -> bool {
    WORKERS_RUNNING.load(Ordering::Acquire)
}

pub fn poll_count(comp: Component) -> usize {
    POLL_COUNT[component_index(comp)].load(Ordering::Relaxed)
}

pub fn throttle_count(comp: Component) -> usize {
    THROTTLE_COUNT[component_index(comp)].load(Ordering::Relaxed)
}

pub fn last_temperature(comp: Component) -> u32 {
    LAST_TEMP[component_index(comp)].load(Ordering::Relaxed)
}

pub struct WorkerSnapshot {
    pub component: Component,
    pub pid: i32,
    pub polls: usize,
    pub throttles: usize,
    pub last_temp_millideg: u32,
    pub running: bool,
}

pub fn snapshot(comp: Component) -> WorkerSnapshot {
    let idx = component_index(comp);
    WorkerSnapshot {
        component: comp,
        pid: WORKER_PIDS[idx].load(Ordering::Acquire),
        polls: POLL_COUNT[idx].load(Ordering::Relaxed),
        throttles: THROTTLE_COUNT[idx].load(Ordering::Relaxed),
        last_temp_millideg: LAST_TEMP[idx].load(Ordering::Relaxed),
        running: WORKER_PIDS[idx].load(Ordering::Acquire) > 0
            && WORKERS_RUNNING.load(Ordering::Acquire),
    }
}

pub fn all_snapshots() -> [WorkerSnapshot; MAX_WORKERS] {
    [
        snapshot(Component::Cpu),
        snapshot(Component::Ram),
        snapshot(Component::Gpu),
        snapshot(Component::Tpu),
        snapshot(Component::Lpu),
    ]
}
