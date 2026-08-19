use core::sync::atomic::{AtomicUsize, Ordering};

static CPU_REG_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_cpu_reg_base(base: usize) {
    CPU_REG_BASE.store(base, Ordering::Release);
}

fn read_cpu_reg(offset: usize) -> u64 {
    let base = CPU_REG_BASE.load(Ordering::Acquire);
    if base != 0 {
        unsafe { core::ptr::read_volatile((base + offset) as *const u64) }
    } else {
        0
    }
}

pub fn read_mpidr_el1() -> u64 {
    read_cpu_reg(0)
}

pub fn cpu_affinity() -> (u8, u8, u8, u8) {
    let mpidr = read_mpidr_el1();
    let aff0 = (mpidr & 0xff) as u8;
    let aff1 = ((mpidr >> 8) & 0xff) as u8;
    let aff2 = ((mpidr >> 16) & 0xff) as u8;
    let aff3 = ((mpidr >> 32) & 0xff) as u8;
    (aff0, aff1, aff2, aff3)
}

pub fn read_revidr_el1() -> u64 {
    read_cpu_reg(8)
}
