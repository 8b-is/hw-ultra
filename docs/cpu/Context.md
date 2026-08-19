# CPU Context

Minimal register context for task switching.

## Structure

```rust
pub struct Context {
    regs: [usize; 16],
}
```

## Methods

```rust
pub fn new() -> Self                           // All registers zeroed
pub fn set_reg(&mut self, index: usize, val: usize)
pub fn get_reg(&self, index: usize) -> usize
pub fn save(&mut self)                         // Save current registers
pub fn restore(&self)                          // Restore saved registers
```

## Register layout

The 16 registers correspond to the general-purpose register file. On x86_64 this maps to RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8–R15. On aarch64 it maps to X0–X15.

## Usage

Used by the scheduler for cooperative task switching:

```rust
let mut ctx = Context::new();
ctx.save();          // capture current state
// ... switch to another task ...
ctx.restore();       // resume this task
```
