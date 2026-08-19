use super::raw::raw_syscall;
use crate::arch::shim;

pub fn sys_socket(domain: i32, type_: i32, protocol: i32) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_socket(),
            domain as u64,
            type_ as u64,
            protocol as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_bind(fd: i64, addr: *const u8, addrlen: u32) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_bind(),
            fd as u64,
            addr as u64,
            addrlen as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_listen(fd: i64, backlog: i32) -> i64 {
    unsafe { raw_syscall(shim::nr_listen(), fd as u64, backlog as u64, 0, 0, 0, 0) }
}

pub fn sys_accept(fd: i64) -> i64 {
    unsafe { raw_syscall(shim::nr_accept(), fd as u64, 0, 0, 0, 0, 0) }
}

pub fn sys_connect(fd: i64, addr: *const u8, addrlen: u32) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_connect(),
            fd as u64,
            addr as u64,
            addrlen as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_fcntl(fd: i64, cmd: i32, arg: i64) -> i64 {
    unsafe { raw_syscall(shim::nr_fcntl(), fd as u64, cmd as u64, arg as u64, 0, 0, 0) }
}
