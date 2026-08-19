# IDT — Interrupt Descriptor Table

## Overview

The `idt` module implements an x86_64 Interrupt Descriptor Table with 256 handler slots.

## Structure

```
Idt {
    handlers: UnsafeCell<[Option<Handler>; 256]>
}
```

`Handler` is a type alias for `fn()`.

## Singleton

```rust
static IDT_ONCE: Once<Idt>
pub fn get() -> Option<&'static Idt>
```

## Methods

| Method | Description |
|--------|-------------|
| `new()` | Creates IDT with all 256 slots empty |
| `register(vector, handler)` | Sets handler for a vector |
| `unregister(vector)` | Clears handler for a vector |
| `handle(vector)` | Calls the registered handler if present |
| `get(vector)` | Returns the handler at a vector |
| `init()` | Registers default handlers for vectors `0x20`–`0x30` |

## Default vector assignments

| Vector | IRQ | Typical device |
|--------|-----|----------------|
| `0x20` | 0 | PIT timer |
| `0x21` | 1 | Keyboard |
| `0x22` | 2 | Cascade |
| `0x23` | 3 | COM2 |
| `0x24` | 4 | COM1 |
| `0x28` | 8 | RTC |
| `0x2E` | 14 | Primary ATA |
| `0x2F` | 15 | Secondary ATA |

Vectors 0–31 are reserved for CPU exceptions (divide error, page fault, GPF, etc.).

## Relation to Controller

The IDT is x86_64-specific. The `Controller` in `controller.rs` is the platform-agnostic layer. On x86_64, the Controller delegates to APIC for enable/disable/EOI, while the IDT manages the actual vector-to-handler mapping at the CPU level.
