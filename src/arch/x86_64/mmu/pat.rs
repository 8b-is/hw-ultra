use core::sync::atomic::{AtomicUsize, Ordering};

static PAT_VALUE: AtomicUsize = AtomicUsize::new(0);

pub fn read_pat() -> u64 {
    let val = unsafe { crate::arch::x86_64::msr::read_msr(0x277) };
    PAT_VALUE.store(val as usize, Ordering::Release);
    val
}

pub fn pat_entry(index: u8) -> u8 {
    let pat = read_pat();
    ((pat >> ((index as u64) * 8)) & 0x7) as u8
}

pub fn is_write_combining(index: u8) -> bool {
    pat_entry(index) == 1
}

pub fn is_uncacheable(index: u8) -> bool {
    pat_entry(index) == 0
}

pub fn is_write_back(index: u8) -> bool {
    pat_entry(index) == 6
}
