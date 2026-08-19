#[derive(Copy, Clone)]
pub struct RamInfo {
    pub total_bytes: usize,
    pub modules: u8,
    pub ecc_enabled: bool,
}

pub fn detect_ram(info: &mut RamInfo) -> bool {
    let total = crate::boot::total_usable_ram();
    if total > 0 {
        info.total_bytes = total;
        info.modules = 1;
        info.ecc_enabled = false;
        return true;
    }
    false
}
