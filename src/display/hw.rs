use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

static FB_BASE: AtomicUsize = AtomicUsize::new(0);
static FB_SIZE: AtomicUsize = AtomicUsize::new(0);
static FB_STRIDE: AtomicU32 = AtomicU32::new(0);

pub fn set_framebuffer(base: usize, size: usize, stride: u32) {
    FB_BASE.store(base, Ordering::Release);
    FB_SIZE.store(size, Ordering::Release);
    FB_STRIDE.store(stride, Ordering::Release);
}

pub fn framebuffer_base() -> usize {
    FB_BASE.load(Ordering::Acquire)
}

pub fn framebuffer_size() -> usize {
    FB_SIZE.load(Ordering::Acquire)
}

pub fn stride() -> u32 {
    FB_STRIDE.load(Ordering::Acquire)
}

pub fn read_reg(base: usize, offset: usize) -> u32 {
    let ptr = (base + offset) as *const u32;
    unsafe { core::ptr::read_volatile(ptr) }
}

pub fn write_reg(base: usize, offset: usize, val: u32) {
    let ptr = (base + offset) as *mut u32;
    unsafe { core::ptr::write_volatile(ptr, val) }
}

pub fn enable_display(base: usize) {
    let ctrl = read_reg(base, 0x00);
    write_reg(base, 0x00, ctrl | 1);
}

pub fn disable_display(base: usize) {
    let ctrl = read_reg(base, 0x00);
    write_reg(base, 0x00, ctrl & !1);
}

pub fn is_enabled(base: usize) -> bool {
    (read_reg(base, 0x00) & 1) != 0
}
