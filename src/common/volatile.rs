/// # Safety
/// `src` must be a valid, aligned pointer for reads of type `T`.
pub unsafe fn read_volatile<T>(src: *const T) -> T {
    core::ptr::read_volatile(src)
}
/// # Safety
/// `dst` must be a valid, aligned pointer for writes of type `T`.
pub unsafe fn write_volatile<T>(dst: *mut T, val: T) {
    core::ptr::write_volatile(dst, val)
}
