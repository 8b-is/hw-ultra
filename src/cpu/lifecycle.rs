use crate::common::once::Once;
use crate::cpu::detect::detect_cpu_info;
use crate::cpu::info::CpuInfo;
use ::core::mem;
use ::core::sync::atomic::{AtomicUsize, Ordering};

static CPU_INFO: Once<CpuInfo> = Once::new();

pub fn init() {
    if let Some(info) = detect_cpu_info() {
        {
            static SET_OK: AtomicUsize = AtomicUsize::new(0);
            if CPU_INFO.set(info) {
                SET_OK.store(1, Ordering::Release);
            } else {
                SET_OK.store(2, Ordering::Release);
                static ERR_SZ0: AtomicUsize = AtomicUsize::new(0);
                ERR_SZ0.store(mem::size_of::<CpuInfo>(), Ordering::Release);
            }
        }
    }
}

pub fn get_info() -> Option<&'static CpuInfo> {
    CPU_INFO.get()
}
