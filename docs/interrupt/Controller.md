# Interrupt Controller

## Controller

The `Controller` struct is the core interrupt management component. It holds an array of 256 handler function pointers and dispatches incoming interrupts to the correct handler.

### Singleton

```rust
static GLOBAL_CONTROLLER: Once<Controller>
```

Initialized once during `init_interrupts()`. Retrieved via the module-level functions.

### Initialization

`Controller::init()` detects the platform and configures the appropriate interrupt controller:
- **x86_64**: Programs the APIC (or legacy PIC)
- **AArch64**: Programs the GIC (Generic Interrupt Controller)

### Handler registration

Handlers are plain `fn()` pointers — no closures, no state. This ensures they are safe to call from interrupt context (no heap allocation, no locks, no panics).

### Dispatch path

1. Hardware raises IRQ → CPU vectors to interrupt entry
2. `handle_irq(irq)` is called with the vector number
3. `Controller::dispatch(irq)` checks `handlers[irq]`
4. If `Some(f)`, calls `f()`
5. `Controller::eoi(irq)` signals end-of-interrupt

### Platform-specific operations

| Operation | x86_64 | AArch64 |
|-----------|--------|---------|
| Enable IRQ | APIC LVT unmask | GIC ISENABLER |
| Disable IRQ | APIC LVT mask | GIC ICENABLER |
| EOI | Write to APIC EOI register | Write to GIC EOIR |

## IDT (x86_64 only)

The IDT is a second layer specifically for x86_64 that maps CPU exception vectors and hardware IRQs to handler functions. During init, vectors `0x20`–`0x30` are pre-registered for standard hardware IRQs (timer, keyboard, etc.).

The IDT singleton is separate from the Controller to allow both exception handling (vectors 0–31) and device IRQ handling (vectors 32+).
