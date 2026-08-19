use crate::arch::Architecture;

pub struct CpuInfo {
    pub arch: Architecture,
    pub id: u64,
    pub vendor: &'static str,
    pub frequency_hz: u64,
    pub cores: u8,
    pub physical_cores: u8,
    pub logical_cores: u8,
    pub threads_per_core: u8,
    pub model_name: [u8; 48],
    pub model_name_len: u8,
    pub l1_cache_kb: u16,
    pub l2_cache_kb: u16,
    pub l3_cache_kb: u16,
    pub has_ht: bool,
}

pub struct ComponentStatus {
    pub name: &'static str,
    pub available: bool,
    pub capacity: usize,
    pub commands: &'static [&'static str],
}

pub fn model_name_str(info: &CpuInfo) -> &str {
    let len = info.model_name_len as usize;
    if len == 0 {
        return "";
    }
    let slice = &info.model_name[..len];
    core::str::from_utf8(slice).unwrap_or("")
}

pub fn fill_cpu_component(cs: &mut ComponentStatus) -> bool {
    if let Some(info) = super::detect::detect_cpu_info() {
        cs.name = "cpu";
        cs.available = true;
        cs.capacity = info.frequency_hz as usize;
        cs.commands = &[];
        true
    } else {
        cs.name = "cpu";
        cs.available = false;
        cs.capacity = 0;
        cs.commands = &[];
        false
    }
}
