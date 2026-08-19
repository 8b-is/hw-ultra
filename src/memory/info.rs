use crate::common::once::OnceCopy;

#[derive(Copy, Clone)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub available_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_free_bytes: u64,
}

pub type DetectMemoryFn = fn() -> Option<MemoryInfo>;

static DETECT_FN: OnceCopy<DetectMemoryFn> = OnceCopy::new();

pub fn set_detect_memory_fn(f: DetectMemoryFn) {
    DETECT_FN.set(f);
}

pub fn detect_memory_info() -> Option<MemoryInfo> {
    if let Some(f) = DETECT_FN.get() {
        return f();
    }
    detect_memory_native()
}

#[repr(C)]
struct Sysinfo {
    uptime: i64,
    loads: [u64; 3],
    totalram: u64,
    freeram: u64,
    sharedram: u64,
    bufferram: u64,
    totalswap: u64,
    freeswap: u64,
    procs: u16,
    _pad: u16,
    _pad2: u32,
    totalhigh: u64,
    freehigh: u64,
    mem_unit: u32,
}

fn detect_memory_native() -> Option<MemoryInfo> {
    let nr_sysinfo = crate::arch::shim::nr_sysinfo();
    if nr_sysinfo == crate::common::error::ERR_NOT_IMPLEMENTED {
        return None;
    }

    let mut info = Sysinfo {
        uptime: 0,
        loads: [0; 3],
        totalram: 0,
        freeram: 0,
        sharedram: 0,
        bufferram: 0,
        totalswap: 0,
        freeswap: 0,
        procs: 0,
        _pad: 0,
        _pad2: 0,
        totalhigh: 0,
        freehigh: 0,
        mem_unit: 0,
    };

    let ret = unsafe {
        crate::sys::arch::shim::raw_syscall(
            nr_sysinfo,
            &mut info as *mut Sysinfo as u64,
            0,
            0,
            0,
            0,
            0,
        )
    };
    if ret < 0 {
        return None;
    }

    let unit = if info.mem_unit == 0 {
        1u64
    } else {
        info.mem_unit as u64
    };
    Some(MemoryInfo {
        total_bytes: info.totalram * unit,
        free_bytes: info.freeram * unit,
        available_bytes: info.freeram * unit,
        swap_total_bytes: info.totalswap * unit,
        swap_free_bytes: info.freeswap * unit,
    })
}
