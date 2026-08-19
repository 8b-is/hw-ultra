# Contributing Guide

## Code style

- **No `std`** — the crate is `#![no_std]`. No `println!`, `Vec`, `String`, `Box`, or any `std` type.
- **No `#[cfg]`** — no conditional compilation. Platform differences are handled by runtime dispatch through shims.
- **`extern "C"` for blob calling convention only** — used on machine code blobs (syscall, CPUID) to guarantee register placement. No C library linkage.
- **No `#[no_mangle]`** — no symbol export.
- **No `static mut`** — use `AtomicUsize`, `AtomicBool`, `AtomicI64`, `Once<T>`, or `OnceCopy<T>`.
- **No comments** — except `/// # Safety` doc comments required by clippy for `unsafe` functions.
- **Fixed-size arrays** — no heap allocation. All collections are bounded arrays.

## Module visibility

- Only `pub mod sys` is exported from `lib.rs`
- All other modules are `mod` (private)
- All access goes through `hardware::sys::*`

## Adding code

1. Read [Warnings.md](../Warnings.md) completely before modifying any module
2. Read the overview doc for the module you're changing
3. Follow the existing patterns (see [DesignPatterns.md](../DesignPatterns.md))
4. Run `cargo test` — all 12 tests must pass
5. Run `cargo clippy` — must be clean (zero warnings)

## Common patterns

- **Singleton**: use `Once<T>` with `init()` + `get() -> Option<&'static T>`
- **State**: use `AtomicUsize` or `AtomicBool`
- **Hardware access**: always go through `arch::shim` function pointers
- **Error handling**: return `Option<T>` (no `Result` with string errors in core paths)
- **Rate limiting**: use `Guard` atomics for hardware access functions

## Tests

Tests are in `src/init/detect_test.rs`. The test suite includes:
- **11 `detect_*` tests** — verify each subsystem initializes correctly
- **1 `stress_sequential` test** — 7-phase hardware stress test

Never skip tests with `#[ignore]` — all tests must pass on the CI target.

## Review checklist

Before submitting changes, verify:
- [ ] `cargo build` succeeds
- [ ] `cargo test` — 12/12 passing
- [ ] `cargo clippy` — 0 warnings
- [ ] No `std` imports
- [ ] No `#[cfg]`, `#[no_mangle]`, or `static mut` (`extern "C"` allowed only for blob calling convention)
- [ ] No unbounded allocations
- [ ] All hardware access goes through shims
- [ ] Singletons use `Once<T>` pattern
