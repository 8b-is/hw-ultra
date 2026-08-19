# Testing

## Test suites

The crate includes two test categories:

### `detect_all` tests (11 tests)

These tests run the full hardware detection pipeline and verify that all subsystems initialize correctly. Output is **buffered**: each test acquires a spinlock (`TEST_LOCK`), writes to an 8 KB static buffer, and flushes atomically on release — preventing interleaving from parallel test execution. Thread numbering is **1-based** (thread 1…12). They cover:

1. Architecture detection
2. CPU information gathering
3. Memory detection
4. Firmware probing (ACPI, UEFI, DeviceTree, SMBIOS)
5. Bus enumeration (PCI, PCIe, AMBA, Virtio)
6. DMA engine initialization
7. IOMMU setup
8. Interrupt controller initialization
9. Timer registration
10. Accelerator detection (GPU, TPU, LPU)
11. Topology and security

### `stress_sequential` test (1 test, 7 phases)

This test deliberately pushes the system to verify that the Guardian correctly limits resource usage:

| Phase | Target | Guardian limit | What it tests |
|-------|--------|---------------|--------------|
| 1 | CPU | 80% of logical cores | fork() with waitpid(), Guardian caps at limit (at least 1 fork on single-core) |
| 2 | RAM | 80% of total | mmap_anon() allocation, Guardian blocks at limit |
| 3 | Disk I/O | N/A | File write/read via raw syscalls |
| 4 | Cache | 70% of L3 | Sequential memory access pattern |
| 5 | Context switching | 80% of cores | Rapid fork/exit cycles |
| 6 | Swap | 50% of total | mmap_anon() beyond RAM, Guardian blocks excess |
| 7 | GPU DRM | N/A | GEM buffer allocation, NOP command submission |

## Running tests

```bash
# All tests
cargo test

# Only detect tests
cargo test detect_all

# Only stress test (save work first)
cargo test stress_sequential

# With output
cargo test -- --nocapture
```

## Test requirements

- **Linux x86_64 or aarch64** host (CPU count detected via `sched_getaffinity`)
- **2+ GB free RAM** for stress tests
- **Available swap** for swap stress phase
- **Optional**: DRM device access (AMD Radeon) for GPU phase
- Tests use `hardware::sys::*` paths (not `hardware::cpu::*` etc.)

## Test safety

All tests use the Guardian to cap resource usage. The stress test:
- Allocates up to 80% RAM then stops (capacity ceiling + surge rate limit)
- Uses up to 50% swap then stops
- Forks up to 80% of logical cores then stops
- Always calls `waitpid()` after `fork()`
- Always calls `sys_munmap()` after `sys_mmap_anon()`

If surge limiting is configured, high-frequency allocations may be rejected even below the capacity ceiling. Use `guardian_snapshot()` to inspect which layer blocked.

If a test makes the system unresponsive:
1. Wait 60 seconds for Guardian timeout
2. Try `Alt+SysRq+F` to invoke OOM killer
3. Try `Alt+SysRq+B` for immediate reboot
4. Hold power button 5 seconds as last resort

## Writing new tests

```rust
#[test]
fn my_test() {
    // All imports through sys
    use hardware::sys;
    
    // Initialize system
    let system = sys::System::init();
    
    // Use Guardian for resource-intensive operations
    // ...
}
```

Important rules:
- Import through `hardware::sys`, not internal modules
- Always use Guardian checks for allocations
- Always clean up (munmap, waitpid, close)
- Do not hardcode syscall numbers
- Do not bypass the shim layer
