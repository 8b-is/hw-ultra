pub fn total_usable_ram() -> usize {
    if let Some(info) = crate::memory::info::detect_memory_info() {
        return info.total_bytes as usize;
    }
    0
}
