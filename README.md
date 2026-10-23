<div align="center">
  <img src="./assets/polar_galaxy.jpg" alt="Polar Galaxy research artwork" width="800"/>
  <h1>hw-ultra 🚀</h1>
  <p><strong>An experimental no_std hardware abstraction crate.</strong></p>
</div>

## Current scope

This fork accompanies `MLX-QUANT` research. Its exported APIs include hardware
abstractions and memory allocators; source availability does not establish that
all backends work on a particular device or operating system.

The bump allocator in `src/memory/heap/bump.rs` reserves aligned ranges from a
single static **64 KiB** byte array shared by every allocator instance. Its
atomic cursor reserves disjoint ranges for concurrent allocations. The guardian
rejects requests exceeding **80%** of the heap, including alignment padding.
This allocator does not map GPU memory or bypass an operating system driver.

## Usage

Add `hw_ultra` as a dependency, using the checkout or revision you have validated.
The current allocator is exported through `hw_ultra::sys`:

```rust
use hw_ultra::sys::BumpAllocator;

let allocator = BumpAllocator::new();
let raw_tensor_ptr = allocator.alloc(1024, 16); // 1 KiB, aligned to 16 bytes.
assert!(!raw_tensor_ptr.is_null(), "shared bump heap exhausted");

// This allocation is writable CPU memory, not a GPU buffer.
unsafe {
    raw_tensor_ptr.write(42);
    assert_eq!(raw_tensor_ptr.read(), 42);
}
```

`alloc(size, align)` returns a null pointer for invalid alignment, overflow or
insufficient guarded capacity. Alignment must be a nonzero power of two.
Allocation does not free individual ranges. `reset()` rewinds the shared heap:
call it only after all users have stopped allocating and stopped using every
previously returned pointer. Constructing a new allocator does not create a new
heap. The example needs no hardware initialization or device access.

## Validation and research claims

The focused `bump_allocator` integration test exercises concurrent reservations,
alignment and capacity boundaries using ordinary process memory. Run it with
`cargo test --test bump_allocator`; it is separate from hardware detection and
stress tests. Passing this test does not validate GPU dispatch or hardware access.

Earlier README figures (0.042 ms dispatch, 99.8% VRAM usage, ~80x allocation
speedup and 130 ns cache hits) lack a reproducible benchmark linked from this
README. Treat them as unverified claims, not measured guarantees or comparisons.
Device performance needs a recorded workload, hardware/OS configuration,
revision and measurement method before it can be compared with MLX or PyTorch.

The polar-galaxy, biological shielding, quantum entanglement, antigravity and
dark-energy descriptions are research imagery, not established physical effects
or implemented API guarantees. Direct Apple AGX/AMD MI300X dispatch and remote
VRAM synchronization remain unverified here. Do not use illustrative MMIO
addresses as instructions for hardware access.

---

## 🙏 Acknowledgements & Support
This astrophysical memory architecture exists alongside `MLX-QUANT` and the relentless contributions of the global AI community.

- **[QWEN](https://qwenlm.github.io/) (Alibaba Cloud)** – For their phenomenal model architecture.
- **[DeepSeek](https://v2.deepseek.com/)** – For pioneering highly efficient MoE architectures.

### 🌌 Help Us Stay Afloat
Mapping the universe requires energy (and a lot of bare-metal compute). If this research is useful to you, consider supporting its development:
- **[GitHub Sponsors](https://github.com/sponsors/peterlodri-sec)**
- **[Support on Ko-fi](https://ko-fi.com/peterlodri)**
- **Star this repository** and share it with a fellow astronaut.


---
