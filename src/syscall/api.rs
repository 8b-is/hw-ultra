use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Copy, Clone)]
pub enum SyscallResult {
    Ok(usize),
    Denied,
    NotFound,
    Fault,
}

const MAX_SYSCALLS: usize = 256;

struct SyscallStorage {
    handlers: [AtomicUsize; MAX_SYSCALLS],
}

unsafe impl Sync for SyscallStorage {}

static SYSCALL_TABLE: SyscallStorage = SyscallStorage {
    handlers: [const { AtomicUsize::new(0) }; MAX_SYSCALLS],
};

pub fn register_syscall(nr: usize, handler: usize) -> bool {
    if nr >= MAX_SYSCALLS {
        return false;
    }
    SYSCALL_TABLE.handlers[nr].store(handler, Ordering::Release);
    true
}

pub fn handle_syscall(nr: usize, arg0: usize, arg1: usize, arg2: usize) -> SyscallResult {
    if nr >= MAX_SYSCALLS {
        return SyscallResult::NotFound;
    }
    let h = SYSCALL_TABLE.handlers[nr].load(Ordering::Acquire);
    if h == 0 {
        return SyscallResult::NotFound;
    }
    let func: fn(usize, usize, usize) -> usize = unsafe { core::mem::transmute(h) };
    SyscallResult::Ok(func(arg0, arg1, arg2))
}
