pub trait IrqController {
    fn init(&self) -> bool;
    fn enable_irq(&self, irq: u8);
    fn disable_irq(&self, irq: u8);
    fn eoi(&self, irq: u8);
}
