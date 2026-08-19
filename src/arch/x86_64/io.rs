use crate::common::once::OnceCopy;

fn has_hw_privilege() -> bool {
    crate::arch::shim::privilege::has_hw_privilege()
}

type InlFn = fn(u16) -> u32;
type OutlFn = fn(u16, u32);
type InbFn = fn(u16) -> u8;
type OutbFn = fn(u16, u8);

static INL_FN: OnceCopy<InlFn> = OnceCopy::new();
static OUTL_FN: OnceCopy<OutlFn> = OnceCopy::new();
static INB_FN: OnceCopy<InbFn> = OnceCopy::new();
static OUTB_FN: OnceCopy<OutbFn> = OnceCopy::new();

pub fn set_inl_fn(f: InlFn) {
    INL_FN.set(f);
}
pub fn set_outl_fn(f: OutlFn) {
    OUTL_FN.set(f);
}
pub fn set_inb_fn(f: InbFn) {
    INB_FN.set(f);
}
pub fn set_outb_fn(f: OutbFn) {
    OUTB_FN.set(f);
}

/// # Safety
/// `port` must be a valid I/O port and the caller must hold I/O privilege.
pub unsafe fn inl(port: u16) -> u32 {
    if !has_hw_privilege() {
        return 0xFFFF_FFFF;
    }
    if let Some(f) = INL_FN.get() {
        f(port)
    } else {
        0xFFFF_FFFF
    }
}

/// # Safety
/// `port` must be a valid I/O port and the caller must hold I/O privilege.
pub unsafe fn outl(port: u16, val: u32) {
    if !has_hw_privilege() {
        return;
    }
    if let Some(f) = OUTL_FN.get() {
        f(port, val);
    }
}

/// # Safety
/// `port` must be a valid I/O port and the caller must hold I/O privilege.
pub unsafe fn inb(port: u16) -> u8 {
    if !has_hw_privilege() {
        return 0xFF;
    }
    if let Some(f) = INB_FN.get() {
        f(port)
    } else {
        0xFF
    }
}

/// # Safety
/// `port` must be a valid I/O port and the caller must hold I/O privilege.
pub unsafe fn outb(port: u16, val: u8) {
    if !has_hw_privilege() {
        return;
    }
    if let Some(f) = OUTB_FN.get() {
        f(port, val);
    }
}
