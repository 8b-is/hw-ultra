/// # Safety
/// `addr` must be a valid, aligned MMIO address mapped into the process.
pub unsafe fn mmio_read32(addr: usize) -> u32 {
    let ptr = addr as *const u32;
    core::ptr::read_volatile(ptr)
}

/// # Safety
/// `addr` must be a valid, aligned MMIO address mapped into the process.
pub unsafe fn mmio_write32(addr: usize, val: u32) {
    let ptr = addr as *mut u32;
    core::ptr::write_volatile(ptr, val);
}
