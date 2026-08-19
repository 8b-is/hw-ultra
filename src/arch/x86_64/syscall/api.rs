use crate::common::once::OnceCopy;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};
use crate::common::once::Once;

pub type SyscallHandler = fn(u64,u64,u64,u64) -> i64;

static SYSCALL_TABLE_INITED: AtomicU8 = AtomicU8::new(0);
static SYSCALL_TABLE_ONCE: Once<[OnceCopy<SyscallHandler>; 256]> = Once::new();

fn syscall_table() -> &'static [OnceCopy<SyscallHandler>; 256] {
    if SYSCALL_TABLE_ONCE.get().is_none() {
        if SYSCALL_TABLE_INITED.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let mut arr: MaybeUninit<[OnceCopy<SyscallHandler>; 256]> = MaybeUninit::uninit();
            let ptr = arr.as_mut_ptr() as *mut OnceCopy<SyscallHandler>;
            for i in 0..256usize {
                unsafe { core::ptr::write(ptr.add(i), OnceCopy::new()); }
            }
            let ok = SYSCALL_TABLE_ONCE.set(unsafe { arr.assume_init() });
            debug_assert!(ok);
            SYSCALL_TABLE_INITED.store(2, Ordering::Release);
        } else {
            while SYSCALL_TABLE_INITED.load(Ordering::Acquire) != 2 {}
        }
    }
    match SYSCALL_TABLE_ONCE.get() {
        Some(t) => t,
        None => {
            static EMPTY: [crate::common::once::OnceCopy<SyscallHandler>; 0] = [];
            &EMPTY
        }
    }
}

pub fn register_syscall(nr: u8, handler: SyscallHandler) -> bool {
    let table = syscall_table();
    if nr as usize >= table.len() { return false }
    table[nr as usize].set(handler)
}

pub fn handle_syscall(nr: u64, a: u64, b: u64, c: u64, d: u64) -> i64 {
    let idx = nr as usize;
    let table = syscall_table();
    if idx >= table.len() { return crate::common::error::ERR_NOT_FOUND }
    if let Some(f) = table[idx].get() { f(a,b,c,d) } else { crate::common::error::ERR_NOT_FOUND }
}
