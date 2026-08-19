use core::sync::atomic::{AtomicU8, Ordering};

use crate::arch::x86_64::interrupt::apic::Apic;
use crate::arch::x86_64::interrupt::ioapic::IoApic;
use crate::arch::x86_64::interrupt::pic::Pic8259;
use crate::interrupt::IrqController;

pub struct X86Controller;

static SELECTED_KIND: AtomicU8 = AtomicU8::new(0);
static PIC_IMPL: Pic8259 = Pic8259::new();
static APIC_ONCE: crate::common::once::Once<Apic> = crate::common::once::Once::new();
static IOAPIC_ONCE: crate::common::once::Once<IoApic> = crate::common::once::Once::new();
static APIC_ADAPTER: ApicAdapter = ApicAdapter;

struct ApicAdapter;

impl ApicAdapter {
    unsafe fn apic() -> Option<&'static Apic> {
        APIC_ONCE.get()
    }
}

impl IrqController for ApicAdapter {
    fn init(&self) -> bool {
        match unsafe { Self::apic() } {
            Some(a) => a.init(),
            None => false,
        }
    }
    fn enable_irq(&self, irq: u8) {
        static IRQ_X86_LAST: core::sync::atomic::AtomicUsize =
            core::sync::atomic::AtomicUsize::new(0);
        IRQ_X86_LAST.store(irq as usize, core::sync::atomic::Ordering::Release);
    }
    fn disable_irq(&self, irq: u8) {
        static IRQ_X86_LAST2: core::sync::atomic::AtomicUsize =
            core::sync::atomic::AtomicUsize::new(0);
        IRQ_X86_LAST2.store(irq as usize, core::sync::atomic::Ordering::Release);
    }
    fn eoi(&self, irq: u8) {
        static IRQ_X86_LAST3: core::sync::atomic::AtomicUsize =
            core::sync::atomic::AtomicUsize::new(0);
        IRQ_X86_LAST3.store(irq as usize, core::sync::atomic::Ordering::Release);
        unsafe {
            if let Some(a) = Self::apic() {
                a.eoi();
            }
        }
    }
}

struct IoapicAdapter;
static IOAPIC_ADAPTER: IoapicAdapter = IoapicAdapter;

impl IoapicAdapter {
    unsafe fn ioapic() -> Option<&'static IoApic> {
        IOAPIC_ONCE.get()
    }
    unsafe fn apic() -> Option<&'static Apic> {
        APIC_ONCE.get()
    }
}

impl IrqController for IoapicAdapter {
    fn init(&self) -> bool {
        unsafe {
            match Self::ioapic() {
                Some(io) => io.version() != 0,
                None => false,
            }
        }
    }

    fn enable_irq(&self, irq: u8) {
        unsafe {
            if let (Some(apic), Some(ioapic)) = (Self::apic(), Self::ioapic()) {
                let apic_id = apic.id() as u8;
                let vec = 0x20u8.wrapping_add(irq);
                ioapic.route_irq(irq, apic_id, vec);
            }
        }
    }

    fn disable_irq(&self, irq: u8) {
        unsafe {
            if let Some(ioapic) = Self::ioapic() {
                let vec = 0u8;
                let apic_id = 0u8;
                let low = (vec as u32) | (1u32 << 16);
                let high = (apic_id as u32) << 24;
                ioapic.write_redtbl(irq, low, high);
            }
        }
    }

    fn eoi(&self, irq: u8) {
        static IRQ_IOAPIC_LAST: core::sync::atomic::AtomicUsize =
            core::sync::atomic::AtomicUsize::new(0);
        IRQ_IOAPIC_LAST.store(irq as usize, core::sync::atomic::Ordering::Release);
        unsafe {
            if let Some(apic) = Self::apic() {
                apic.eoi();
            }
        }
    }
}

pub fn init() {
    let apic = Apic::probe();
    debug_assert!(APIC_ONCE.get().is_none());
    APIC_ONCE.set(apic);

    if let Some(base) = crate::firmware::acpi::find_ioapic_base() {
        let ioapic = IoApic::new(base);
        debug_assert!(IOAPIC_ONCE.get().is_none());
        IOAPIC_ONCE.set(ioapic);
        if IOAPIC_ADAPTER.init() {
            SELECTED_KIND.store(2, Ordering::Release);
            return;
        }
    }

    if APIC_ADAPTER.init() {
        SELECTED_KIND.store(1, Ordering::Release);
        return;
    }

    PIC_IMPL.init();
    SELECTED_KIND.store(0, Ordering::Release);
}

fn get_selected() -> &'static dyn IrqController {
    match SELECTED_KIND.load(Ordering::Acquire) {
        1 => &APIC_ADAPTER as &dyn IrqController,
        2 => &IOAPIC_ADAPTER as &dyn IrqController,
        other => {
            static SEL_X86_SIG: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            SEL_X86_SIG.store(other as usize, core::sync::atomic::Ordering::Release);
            &PIC_IMPL as &dyn IrqController
        }
    }
}

pub fn enable_irq(irq: u8) {
    get_selected().enable_irq(irq);
}
pub fn disable_irq(irq: u8) {
    get_selected().disable_irq(irq);
}
pub fn eoi(irq: u8) {
    get_selected().eoi(irq);
}
