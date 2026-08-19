use core::sync::atomic::{AtomicUsize, Ordering};

static MICROCODE_REV: AtomicUsize = AtomicUsize::new(0);

pub fn read_revision() -> u32 {
    let rev = unsafe { crate::arch::x86_64::msr::read_msr(0x8B) };
    let high = (rev >> 32) as u32;
    MICROCODE_REV.store(high as usize, Ordering::Release);
    high
}

pub fn current_revision() -> u32 {
    let stored = MICROCODE_REV.load(Ordering::Acquire);
    if stored != 0 {
        stored as u32
    } else {
        read_revision()
    }
}

pub fn platform_id() -> u8 {
    let msr = unsafe { crate::arch::x86_64::msr::read_msr(0x17) };
    ((msr >> 50) & 0x7) as u8
}
