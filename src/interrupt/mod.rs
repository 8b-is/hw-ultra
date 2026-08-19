mod controller;
mod handler;
pub mod idt;
mod irq;
mod vector;

pub use controller::Controller;
pub use idt::Handler;
pub use idt::Idt;
pub use irq::IrqController;
pub use vector::Vector;

pub fn register(irq: usize, handler: fn()) -> bool {
    controller::register(irq, handler)
}

pub fn unregister(irq: usize) -> bool {
    controller::unregister(irq)
}

pub fn dispatch(irq: usize) {
    controller::dispatch(irq)
}

pub fn handle() {
    handler::handle()
}
