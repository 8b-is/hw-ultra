use crate::common::once::Once;
use crate::interrupt::Handler;
use core::cell::UnsafeCell;

const IDT_ENTRIES: usize = 256;
struct IdtTable(UnsafeCell<[Option<Handler>; IDT_ENTRIES]>);

unsafe impl Sync for IdtTable {}

static IDT_ONCE: Once<IdtTable> = Once::new();

fn table() -> Option<&'static IdtTable> {
    if let Some(t) = IDT_ONCE.get() {
        return Some(t);
    }
    let arr = IdtTable(UnsafeCell::new([None; IDT_ENTRIES]));
    let _ = IDT_ONCE.set(arr);
    IDT_ONCE.get()
}

pub fn set_handler(vector: usize, h: Handler) -> bool {
    if vector >= IDT_ENTRIES {
        return false;
    }
    let cell = match table() {
        Some(t) => t,
        None => return false,
    };
    unsafe {
        (*cell.0.get())[vector] = Some(h);
    }
    true
}

pub fn invoke(vector: usize) {
    if vector >= IDT_ENTRIES {
        return;
    }
    crate::interrupt::dispatch(vector);
    crate::interrupt::Controller::eoi(vector as u8);
}
