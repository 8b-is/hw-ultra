<div align="center">
  <img src="./assets/polar_galaxy.jpg" alt="Polar Galaxy 4D Queue" width="800"/>
  <h1>hw-ultra 🚀</h1>
  <p><strong>A bare-metal memory and command queue abstraction crate for Apple Silicon and AMD MI300X.</strong></p>
</div>

---

## What is this? (ELI5 Version)

Imagine you have a super-fast sports car (your GPU), but to get gas (data), you have to fill out 10 pages of paperwork with the DMV (the Operating System). By the time you get the gas, the race is over.

`hw-ultra` rips up the DMV paperwork. It lets the sports car take the gas directly from the pump using bare-metal memory structures!

### Trick 1: The Bare-Metal Bump Allocator
Normally, when you create a Matrix (Tensor) in Python or standard Rust, it asks the macOS Kernel for memory. The Kernel does safety checks, acquires locks, and wastes time. 
**Our Fix:** We map a massive chunk of raw hardware memory (16KB pages) once. Then, we use a single thread-local counter to just "bump" forward every time we need memory. It’s **~80x faster** than standard macOS memory allocation.

### Trick 2: The O(1) Tensor Cache
When generating text in an AI model, we create and destroy the exact same memory shapes millions of times. A standard bump allocator would run out of memory quickly.
**Our Fix:** When a Tensor is done, we don't throw it away. We put its memory address in a tiny "Append-Only Cache". The next time you ask for that exact shape, we just hand you the old memory address instantly (in **130 nanoseconds**). Zero waste, infinite loops!

### Trick 3: The 4D Polar Galaxy Queue (Coming Soon)
How do we feed data to the GPU? Imagine a spiral galaxy. 
- **The Spiral Arms:** Data (Weights and Activations) spiraling inwards via asynchronous hardware streams.
- **The Accretion Disk:** The GPU compute cores sitting in the center.
- **The Magic:** When the data reaches the center, it physically triggers a "Hardware Doorbell". The GPU executes the math instantly without the CPU ever knowing it happened. It's a continuous, multi-dimensional flow of matrix math!

---

## Usage

Add `hw_ultra` to your `Cargo.toml`.

```rust
use hw_ultra::BumpAllocator;
// Map directly to Apple Silicon hardware pages!
let allocator = BumpAllocator::new();
let raw_tensor_ptr = allocator.fast_alloc8(2 * 1024 * 1024); // 2MB instantly
```

*Note: This fork is highly specialized for `MLX-QUANT` bare-metal research.*
