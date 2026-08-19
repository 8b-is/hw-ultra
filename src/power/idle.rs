pub fn enter_idle() {
    crate::sys::sleep_ns(1_000_000);
}

pub fn halt() {
    crate::sys::sched_yield();
    enter_idle();
}
