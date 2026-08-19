use super::raw::raw_syscall;
use crate::arch::shim;

pub fn fork() -> i64 {
    let current = crate::runtime::resources::cpu_threads();
    if !crate::arch::guardian::gate_cpu(current, 1) {
        return crate::common::error::ERR_NOT_IMPLEMENTED;
    }
    let pid = unsafe { raw_syscall(shim::nr_clone(), shim::os_sigchld() as u64, 0, 0, 0, 0, 0) };
    if pid > 0 {
        crate::runtime::resources::try_alloc_cpu(1);
    }
    pid
}

pub fn request_hw_privilege() -> bool {
    let nr = shim::nr_iopl();
    if nr == crate::common::error::ERR_NOT_IMPLEMENTED {
        return false;
    }
    let ret = unsafe { raw_syscall(nr, 3, 0, 0, 0, 0, 0) };
    if ret >= 0 {
        crate::arch::shim::privilege::set_hw_privilege(true);
        true
    } else {
        false
    }
}

pub fn has_hw_privilege() -> bool {
    crate::arch::shim::privilege::has_hw_privilege()
}

pub fn exit(code: i32) -> ! {
    shim::arch_exit(code)
}

pub fn waitpid(pid: i64) -> i64 {
    unsafe { raw_syscall(shim::nr_wait4(), pid as u64, 0, 0, 0, 0, 0) }
}

pub fn sys_kill(pid: i32, sig: i32) -> i64 {
    unsafe { raw_syscall(shim::nr_kill(), pid as u64, sig as u64, 0, 0, 0, 0) }
}

pub fn sys_execve(path: &[u8], argv: &[*const u8], envp: &[*const u8]) -> i64 {
    if path.is_empty() || path[path.len() - 1] != 0 {
        return crate::common::error::ERR_NOT_IMPLEMENTED;
    }
    unsafe {
        raw_syscall(
            shim::nr_execve(),
            path.as_ptr() as u64,
            argv.as_ptr() as u64,
            envp.as_ptr() as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_rt_sigaction(sig: i32, act: *const u8, oldact: *mut u8, sigsetsize: usize) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_rt_sigaction(),
            sig as u64,
            act as u64,
            oldact as u64,
            sigsetsize as u64,
            0,
            0,
        )
    }
}
