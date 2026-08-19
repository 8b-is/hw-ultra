use super::raw::raw_syscall;
use crate::arch::shim;

pub fn sys_mmap_anon(len: usize) -> *mut u8 {
    let ret = unsafe {
        raw_syscall(
            shim::nr_mmap(),
            0,
            len as u64,
            shim::os_prot_read_write() as u64,
            shim::os_map_private_anon() as u64,
            (-1i64) as u64,
            0,
        )
    };
    if ret < 0 {
        core::ptr::null_mut()
    } else {
        ret as *mut u8
    }
}

pub fn sys_mmap_shared_anon(len: usize) -> *mut u8 {
    let ret = unsafe {
        raw_syscall(
            shim::nr_mmap(),
            0,
            len as u64,
            shim::os_prot_read_write() as u64,
            shim::os_map_shared_anon() as u64,
            (-1i64) as u64,
            0,
        )
    };
    if ret < 0 {
        core::ptr::null_mut()
    } else {
        ret as *mut u8
    }
}

pub fn sys_munmap(addr: *mut u8, len: usize) -> i64 {
    unsafe { raw_syscall(shim::nr_munmap(), addr as u64, len as u64, 0, 0, 0, 0) }
}

pub fn sys_mmap_device(fd: i64, len: usize, offset: u64) -> *mut u8 {
    let ret = unsafe {
        raw_syscall(
            shim::nr_mmap(),
            0,
            len as u64,
            shim::os_prot_read_write() as u64,
            shim::os_map_shared() as u64,
            fd as u64,
            offset,
        )
    };
    if ret < 0 {
        core::ptr::null_mut()
    } else {
        ret as *mut u8
    }
}
