pub fn handle() {
    crate::interrupt::controller::dispatch(0);
    crate::interrupt::Controller::eoi(0);
}

pub fn handle_irq(irq: usize) {
    crate::interrupt::controller::dispatch(irq);
    crate::interrupt::Controller::eoi(irq as u8);
}
