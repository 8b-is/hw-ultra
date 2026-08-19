use crate::arch::Architecture;
use core::cell::UnsafeCell;

type HandlerTable = UnsafeCell<[Option<fn()>; 256]>;

pub struct Controller {
    handlers: HandlerTable,
}

unsafe impl Sync for Controller {}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            handlers: UnsafeCell::new([None; 256]),
        }
    }

    pub fn register(&self, irq: usize, h: fn()) -> bool {
        if irq >= 256 {
            return false;
        }
        unsafe {
            (*self.handlers.get())[irq] = Some(h);
        }
        true
    }

    pub fn unregister(&self, irq: usize) -> bool {
        if irq >= 256 {
            return false;
        }
        unsafe {
            (*self.handlers.get())[irq] = None;
        }
        true
    }

    pub fn dispatch(&self, irq: usize) {
        if irq >= 256 {
            return;
        }
        unsafe {
            if let Some(h) = (*self.handlers.get())[irq] {
                h();
            }
        }
    }
}

static GLOBAL_CONTROLLER: crate::common::once::Once<Controller> = crate::common::once::Once::new();

pub fn register(irq: usize, h: fn()) -> bool {
    if let Some(c) = GLOBAL_CONTROLLER.get() {
        c.register(irq, h)
    } else {
        false
    }
}

pub fn unregister(irq: usize) -> bool {
    if let Some(c) = GLOBAL_CONTROLLER.get() {
        c.unregister(irq)
    } else {
        false
    }
}

pub fn dispatch(irq: usize) {
    if let Some(c) = GLOBAL_CONTROLLER.get() {
        c.dispatch(irq)
    }
    dispatch_irq_wrapper(irq);
}

impl Controller {
    pub fn init() {
        let ok = GLOBAL_CONTROLLER.set(Controller::new());
        debug_assert!(ok);

        match crate::arch::detect_arch() {
            Architecture::X86_64 => crate::arch::x86_64::interrupt::controller::init(),
            Architecture::AArch64 => crate::arch::aarch64::interrupt::gic::init(),
            _ => crate::arch::x86_64::interrupt::controller::init(),
        }

        register_default_irq_handlers();
    }

    pub fn enable_irq(irq: u8) {
        match crate::arch::detect_arch() {
            Architecture::X86_64 => crate::arch::x86_64::interrupt::controller::enable_irq(irq),
            Architecture::AArch64 => crate::arch::aarch64::interrupt::gic::enable_irq(irq),
            _ => crate::arch::x86_64::interrupt::controller::enable_irq(irq),
        }
    }

    pub fn disable_irq(irq: u8) {
        match crate::arch::detect_arch() {
            Architecture::X86_64 => crate::arch::x86_64::interrupt::controller::disable_irq(irq),
            Architecture::AArch64 => crate::arch::aarch64::interrupt::gic::disable_irq(irq),
            _ => crate::arch::x86_64::interrupt::controller::disable_irq(irq),
        }
    }

    pub fn eoi(irq: u8) {
        match crate::arch::detect_arch() {
            Architecture::X86_64 => crate::arch::x86_64::interrupt::controller::eoi(irq),
            Architecture::AArch64 => crate::arch::aarch64::interrupt::gic::eoi(irq),
            _ => crate::arch::x86_64::interrupt::controller::eoi(irq),
        }
    }
}

fn default_irq_wrapper() {
    crate::interrupt::handler::handle();
}

fn dispatch_irq_wrapper(irq: usize) {
    crate::interrupt::handler::handle_irq(irq);
}

fn register_default_irq_handlers() {
    if let Some(c) = GLOBAL_CONTROLLER.get() {
        for irq in 0usize..256usize {
            let ok = c.register(irq, default_irq_wrapper);
            debug_assert!(ok);
        }
    }
}
