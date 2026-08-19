pub struct GenericTpu;

impl GenericTpu {
    pub fn probe() -> Option<Self> {
        Some(GenericTpu)
    }

    pub fn init(&mut self) -> bool {
        let ok = crate::tpu::device::register_irq_vector(0x20usize);
        debug_assert!(ok);
        crate::interrupt::Controller::enable_irq(0);
        true
    }
}
