use core::sync::atomic::{AtomicBool, Ordering};

static HW_PRIVILEGE: AtomicBool = AtomicBool::new(false);

pub fn set_hw_privilege(enabled: bool) {
    HW_PRIVILEGE.store(enabled, Ordering::Release);
}

pub fn has_hw_privilege() -> bool {
    HW_PRIVILEGE.load(Ordering::Acquire)
}
