use crate::arch::shim;

pub(crate) unsafe fn raw_syscall(
    nr: i64,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> i64 {
    shim::raw_syscall(nr, a0, a1, a2, a3, a4, a5)
}
