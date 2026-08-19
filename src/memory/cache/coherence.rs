pub fn ensure_coherence() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}
