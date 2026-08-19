use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_KERNELS: usize = 16;

static KERNEL_COUNT: AtomicUsize = AtomicUsize::new(0);
static KERNEL_ENTRY: [AtomicUsize; MAX_KERNELS] = [const { AtomicUsize::new(0) }; MAX_KERNELS];
static KERNEL_WORKGROUP: [AtomicUsize; MAX_KERNELS] = [const { AtomicUsize::new(0) }; MAX_KERNELS];

#[derive(Copy, Clone)]
pub struct Kernel {
    pub id: usize,
    pub entry_point: usize,
    pub workgroup_size: usize,
}

pub fn register_kernel(entry_point: usize, workgroup_size: usize) -> Option<Kernel> {
    let id = KERNEL_COUNT.fetch_add(1, Ordering::AcqRel);
    if id >= MAX_KERNELS {
        KERNEL_COUNT.fetch_sub(1, Ordering::Release);
        return None;
    }
    KERNEL_ENTRY[id].store(entry_point, Ordering::Release);
    KERNEL_WORKGROUP[id].store(workgroup_size, Ordering::Release);
    Some(Kernel {
        id,
        entry_point,
        workgroup_size,
    })
}

pub fn kernel_info(id: usize) -> Option<Kernel> {
    if id >= KERNEL_COUNT.load(Ordering::Acquire) {
        return None;
    }
    let entry_point = KERNEL_ENTRY[id].load(Ordering::Acquire);
    let workgroup_size = KERNEL_WORKGROUP[id].load(Ordering::Acquire);
    Some(Kernel {
        id,
        entry_point,
        workgroup_size,
    })
}

pub fn kernel_count() -> usize {
    KERNEL_COUNT.load(Ordering::Acquire)
}

pub fn dispatch(k: &Kernel, num_groups: usize) {
    crate::gpu::compute::dispatch::dispatch_kernel();
    let total_invocations = num_groups * k.workgroup_size;
    static TOTAL_DISPATCHED: AtomicUsize = AtomicUsize::new(0);
    TOTAL_DISPATCHED.fetch_add(total_invocations, Ordering::Relaxed);
}
