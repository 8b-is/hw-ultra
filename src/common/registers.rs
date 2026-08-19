use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_REGS: usize = 32;

struct RegisterBank {
    values: [AtomicUsize; MAX_REGS],
    count: AtomicUsize,
}

unsafe impl Sync for RegisterBank {}

static BANK: RegisterBank = RegisterBank {
    values: [const { AtomicUsize::new(0) }; MAX_REGS],
    count: AtomicUsize::new(0),
};

pub fn write_register(index: usize, value: usize) -> bool {
    if index >= MAX_REGS {
        return false;
    }
    BANK.values[index].store(value, Ordering::Release);
    let cur = BANK.count.load(Ordering::Acquire);
    if index >= cur {
        BANK.count.store(index + 1, Ordering::Release);
    }
    true
}

pub fn read_register(index: usize) -> Option<usize> {
    if index >= MAX_REGS {
        return None;
    }
    if index >= BANK.count.load(Ordering::Acquire) {
        return None;
    }
    Some(BANK.values[index].load(Ordering::Acquire))
}

pub fn register_count() -> usize {
    BANK.count.load(Ordering::Acquire)
}
