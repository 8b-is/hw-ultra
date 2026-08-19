pub fn compiler_fence() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

pub fn memory_fence() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}

pub fn load_fence() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
}

pub fn store_fence() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
}
