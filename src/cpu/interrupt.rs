pub fn raise_interrupt() {
    crate::interrupt::Controller::enable_irq(0);
}
