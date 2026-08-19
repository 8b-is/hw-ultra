pub type Handler = fn();

pub struct Idt {
    handlers: core::cell::UnsafeCell<[Option<Handler>; 256]>,
}

unsafe impl Sync for Idt {}

impl Default for Idt {
    fn default() -> Self {
        Self::new()
    }
}

impl Idt {
    pub fn new() -> Self {
        Idt {
            handlers: core::cell::UnsafeCell::new([None; 256]),
        }
    }

    pub fn register(&self, vec: usize, h: Handler) -> bool {
        if vec >= 256 {
            return false;
        }
        unsafe {
            (*self.handlers.get())[vec] = Some(h);
        }
        true
    }

    pub fn unregister(&self, vec: usize) -> bool {
        if vec >= 256 {
            return false;
        }
        unsafe {
            (*self.handlers.get())[vec] = None;
        }
        true
    }

    pub fn handle(&self, vec: usize) {
        if vec >= 256 {
            return;
        }
        unsafe {
            if let Some(h) = (*self.handlers.get())[vec] {
                h();
            }
        }
    }

    pub fn get() -> Option<&'static Idt> {
        get()
    }
}

use crate::common::once::Once;

static IDT_ONCE: Once<Idt> = Once::new();

pub fn init() {
    let idt = Idt::new();
    if IDT_ONCE.set(idt) {
        if let Some(i) = IDT_ONCE.get() {
            for v in 0x20usize..0x30usize {
                let ok = i.register(v, default_handler);
                debug_assert!(ok);
                if let crate::arch::Architecture::X86_64 = crate::arch::detect_arch() {
                    let ok2 = crate::arch::x86_64::interrupt::idt::set_handler(v, default_handler);
                    debug_assert!(ok2);
                }
            }
        }
    }
}

fn default_handler() {}

pub fn get() -> Option<&'static Idt> {
    IDT_ONCE.get()
}
