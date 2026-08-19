use super::raw::raw_syscall;
use crate::arch::shim;

pub fn monotonic_ns() -> u64 {
    #[repr(C)]
    struct Timespec {
        tv_sec: i64,
        tv_nsec: i64,
    }
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let ret = unsafe {
        raw_syscall(
            shim::nr_clock_gettime(),
            shim::os_clock_monotonic() as u64,
            &mut ts as *mut _ as u64,
            0,
            0,
            0,
            0,
        )
    };
    if ret == 0 {
        ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
    } else {
        0
    }
}

pub fn sleep_ns(ns: u64) {
    #[repr(C)]
    struct Timespec {
        tv_sec: i64,
        tv_nsec: i64,
    }
    let req = Timespec {
        tv_sec: (ns / 1_000_000_000) as i64,
        tv_nsec: (ns % 1_000_000_000) as i64,
    };
    unsafe {
        raw_syscall(shim::nr_nanosleep(), &req as *const _ as u64, 0, 0, 0, 0, 0);
    }
}

pub fn sleep_secs(s: u64) {
    sleep_ns(s * 1_000_000_000);
}

pub fn sched_yield() {
    unsafe {
        raw_syscall(shim::nr_sched_yield(), 0, 0, 0, 0, 0, 0);
    }
}
