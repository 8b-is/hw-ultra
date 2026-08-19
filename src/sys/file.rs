use super::consts::O_RDONLY;
use super::raw::raw_syscall;
use crate::arch::shim;

pub fn sys_open(path: &[u8], flags: i32, mode: u32) -> i64 {
    if path.is_empty() || path[path.len() - 1] != 0 {
        return crate::common::error::ERR_NOT_IMPLEMENTED;
    }
    unsafe {
        raw_syscall(
            shim::nr_openat(),
            shim::os_at_fdcwd() as u64,
            path.as_ptr() as u64,
            flags as u64,
            mode as u64,
            0,
            0,
        )
    }
}

pub fn sys_write_fd(fd: i64, buf: &[u8]) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_write(),
            fd as u64,
            buf.as_ptr() as u64,
            buf.len() as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_read_fd(fd: i64, buf: &mut [u8]) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_read(),
            fd as u64,
            buf.as_mut_ptr() as u64,
            buf.len() as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_close(fd: i64) -> i64 {
    unsafe { raw_syscall(shim::nr_close(), fd as u64, 0, 0, 0, 0, 0) }
}

pub fn sys_getdents64(fd: i64, buf: &mut [u8]) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_getdents64(),
            fd as u64,
            buf.as_mut_ptr() as u64,
            buf.len() as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_fsync(fd: i64) -> i64 {
    unsafe { raw_syscall(shim::nr_fsync(), fd as u64, 0, 0, 0, 0, 0) }
}

pub fn sys_unlink(path: &[u8]) -> i64 {
    if path.is_empty() || path[path.len() - 1] != 0 {
        return crate::common::error::ERR_NOT_IMPLEMENTED;
    }
    unsafe {
        raw_syscall(
            shim::nr_unlinkat(),
            shim::os_at_fdcwd() as u64,
            path.as_ptr() as u64,
            0,
            0,
            0,
            0,
        )
    }
}

pub fn sys_mkdir(path: &[u8], mode: u32) -> i64 {
    if path.is_empty() || path[path.len() - 1] != 0 {
        return crate::common::error::ERR_NOT_IMPLEMENTED;
    }
    shim::mkdir(path, mode)
}

pub fn sys_stat(path: &[u8], statbuf: &mut [u8; 144]) -> i64 {
    if path.is_empty() || path[path.len() - 1] != 0 {
        return crate::common::error::ERR_NOT_IMPLEMENTED;
    }
    unsafe {
        raw_syscall(
            shim::nr_stat(),
            shim::os_at_fdcwd() as u64,
            path.as_ptr() as u64,
            statbuf.as_mut_ptr() as u64,
            0,
            0,
            0,
        )
    }
}

pub fn sys_getcwd(buf: &mut [u8]) -> i64 {
    unsafe {
        raw_syscall(
            shim::nr_getcwd(),
            buf.as_mut_ptr() as u64,
            buf.len() as u64,
            0,
            0,
            0,
            0,
        )
    }
}

pub fn sys_ioctl(fd: i64, request: u64, arg: u64) -> i64 {
    unsafe { raw_syscall(shim::nr_ioctl(), fd as u64, request, arg, 0, 0, 0) }
}

#[derive(Copy, Clone)]
pub struct DirEntry {
    pub name: [u8; 128],
    pub name_len: u8,
    pub d_type: u8,
}

pub fn scan_dir(dir_path: &[u8], out: &mut [DirEntry]) -> usize {
    shim::scan_dir_dispatch(dir_path, out)
}

pub fn read_file_bytes(path: &[u8], buf: &mut [u8]) -> usize {
    let fd = sys_open(path, O_RDONLY, 0);
    if fd < 0 {
        return 0;
    }
    let n = sys_read_fd(fd, buf);
    sys_close(fd);
    if n < 0 {
        0
    } else {
        n as usize
    }
}

pub fn trim_trailing(buf: &[u8], len: usize) -> usize {
    let mut l = len;
    while l > 0 && (buf[l - 1] == b'\n' || buf[l - 1] == b'\r' || buf[l - 1] == b' ') {
        l -= 1;
    }
    l
}
