/// # Safety
/// Caller must ensure `nr` is a valid syscall number and arguments are correct.
pub unsafe fn raw_syscall(
    _nr: i64,
    _a0: u64,
    _a1: u64,
    _a2: u64,
    _a3: u64,
    _a4: u64,
    _a5: u64,
) -> i64 {
    crate::common::error::ERR_NOT_IMPLEMENTED
}
