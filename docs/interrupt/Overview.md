# Interrupt Module

The `interrupt` module manages interrupt registration, dispatch, and platform-specific interrupt controllers (APIC on x86_64, GIC on AArch64).

## Submodules

| File | Description |
|------|-------------|
| `controller.rs` | Global interrupt controller with 256 handler slots |
| `handler.rs` | Top-level interrupt dispatch and EOI |
| `idt.rs` | Interrupt Descriptor Table (x86_64) |
| `irq.rs` | `IrqController` trait for platform abstraction |
| `vector.rs` | `Vector` struct (vector number + priority) |
| `apic.rs` | APIC (Advanced Programmable Interrupt Controller) |

## Architecture

```
IRQ fires
  → handler::handle_irq(irq)
    → Controller::dispatch(irq)
      → calls registered fn() handler
    → Controller::eoi(irq)
      → platform-specific EOI (APIC / GIC)
```

## Controller

```
Controller {
    handlers: UnsafeCell<[Option<fn()>; 256]>
}
```

Singleton via `Once<Controller>`. Supports 256 interrupt vectors.

| Method | Description |
|--------|-------------|
| `register(irq, handler)` | Registers a handler for vector `irq` |
| `unregister(irq)` | Removes handler for vector `irq` |
| `dispatch(irq)` | Calls the handler if registered |
| `enable_irq(irq)` | Enables the IRQ on the platform controller |
| `disable_irq(irq)` | Disables the IRQ on the platform controller |
| `eoi(irq)` | Sends end-of-interrupt to the platform controller |
| `init()` | Initializes based on detected architecture |

## Module-level API

| Function | Description |
|----------|-------------|
| `register(irq, handler)` | Registers a handler (delegates to Controller) |
| `unregister(irq)` | Removes a handler |
| `dispatch(irq)` | Dispatches an interrupt |
| `handle()` | Generic interrupt entry point |

## IrqController trait

```rust
pub trait IrqController {
    fn init(&self) -> bool;
    fn enable_irq(&self, irq: u8);
    fn disable_irq(&self, irq: u8);
    fn eoi(&self, irq: u8);
}
```

Platform-specific controllers implement this trait.

## IDT (x86_64)

```
Idt {
    handlers: UnsafeCell<[Option<Handler>; 256]>
}
```

Singleton via `Once<Idt>`. `init()` registers default handlers for vectors `0x20`–`0x30` (IRQ 0–16).

## Vector

```
Vector {
    vector_num: u8
    priority: u8
}
```

## Safety considerations

- Handler array uses `UnsafeCell` — only safe because registration is single-threaded during init
- EOI must be sent for every handled interrupt, or the controller will mask further interrupts
- See [Warnings.md](../Warnings.md) warning 9 for interrupt handler safety
