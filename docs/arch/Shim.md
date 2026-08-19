# Shim Layer

The shim is the central dispatch mechanism for all architecture-dependent operations. It replaces `#[cfg]` conditional compilation with runtime function pointer dispatch.

## Function pointers

All shim functions are stored as `OnceCopy<fn(...)>` statics:

| Shim | Type | Purpose |
|------|------|---------|
| CPUID | `fn(u32, u32) -> Option<(u32,u32,u32,u32)>` | CPU identification |
| ReadMSR | `fn(u32) -> Option<u64>` | Model-Specific Register read |
| MmioRead32 | `fn(usize) -> Option<u32>` | Memory-mapped I/O read |
| MmioWrite32 | `fn(usize, u32) -> bool` | Memory-mapped I/O write |
| ReadAarch64Midr | `fn() -> Option<u64>` | ARM Main ID Register |
| Exit | `fn(i32) -> !` | Process exit |
| Mkdir | `fn(&[u8], u32) -> i64` | Create directory |
| ScanDir | `fn(&[u8], &mut [DirEntry]) -> usize` | Directory listing |

## Public functions

```rust
pub fn init_shims()
pub fn detect_arch() -> Architecture
pub fn arch_cached() -> Option<Architecture>
pub fn cpuid_count(leaf: u32, subleaf: u32) -> Option<(u32, u32, u32, u32)>
pub fn read_msr(msr: u32) -> Option<u64>
pub fn mmio_read32(addr: usize) -> Option<u32>
pub fn mmio_write32(addr: usize, val: u32) -> bool
pub fn read_aarch64_midr() -> Option<u64>
pub fn arch_exit(code: i32) -> !
pub fn mkdir(path: &[u8], mode: u32) -> i64
pub fn scan_dir_dispatch(path: &[u8], out: &mut [DirEntry]) -> usize
```

`arch_cached()` returns the cached architecture without triggering detection. Used by `native_cpuid()` to avoid the recursion cycle (`detect_arch` → `cpuid_count` → `native_cpuid` → `detect_arch`).

## Setter functions

Each function pointer has a corresponding setter:
```rust
pub fn set_cpuid_fn(f: CpuidFn) -> bool
pub fn set_read_msr_fn(f: ReadMsrFn) -> bool
pub fn set_mmio_read32_fn(f: MmioRead32Fn) -> bool
pub fn set_mmio_write32_fn(f: MmioWrite32Fn) -> bool
pub fn set_read_aarch64_midr_fn(f: ReadMidrFn) -> bool
pub fn set_exit_fn(f: ExitFn) -> bool
pub fn set_mkdir_fn(f: MkdirFn) -> bool
pub fn set_scan_dir_fn(f: ScanDirFn) -> bool
```

Each returns `bool` — `true` if the function was registered, `false` if already set.

## Initialization safety

`init_shims()` uses `compare_exchange(0, 1)` on an atomic guard to ensure:
- Exactly one call succeeds
- No recursive re-entry (which would cause stack overflow)
- The `default_*()` fallbacks call `init_shims()` on first use, but `init_shims()` never calls back through the shim
- After registering arch function pointers, `init_shims()` auto-registers the native syscall handler via `register_native_syscall()`, which uses `detect_arch()` to select the correct machine code blob (`X86_64_SYSCALL_BLOB` or `AARCH64_SYSCALL_BLOB`)

## Default behavior

If a shim function is called before registration:
- The `default_*()` function attempts `init_shims()`
- If still unregistered, it returns a safe default: `None`, `false`, `0`

## SyscallNrTable

33 `AtomicI64` statics store syscall numbers for platform operations. All default to `ERR_NOT_IMPLEMENTED` (`-1`) — no platform-specific magic numbers. Fields:

`read`, `write`, `openat`, `close`, `mmap`, `munmap`, `ioctl`, `sched_yield`, `nanosleep`, `clone`, `exit`, `wait4`, `kill`, `fsync`, `unlinkat`, `getdents64`, `clock_gettime`, `sched_setaffinity`, `sched_getaffinity`, `stat`, `socket`, `connect`, `accept`, `bind`, `listen`, `execve`, `fcntl`, `getcwd`, `rt_sigaction`, `iopl`, `mkdirat`, `sysinfo`.

Set once via `set_syscall_nrs(SyscallNrTable { ... })`.

## OsConstants

12 fields storing OS-varying constants, also set once via `set_os_constants(OsConstants { ... })`:

| Field | Purpose |
|-------|---------|
| `at_fdcwd` | `AT_FDCWD` for `openat()` |
| `sigchld` | `SIGCHLD` for `clone()` |
| `map_private_anon` | `MAP_PRIVATE \| MAP_ANONYMOUS` for `mmap()` |
| `prot_read_write` | `PROT_READ \| PROT_WRITE` |
| `wall` | `__WALL` for `waitpid()` |
| `dt_dir` | `DT_DIR` for directory entry type |
| `clock_monotonic` | `CLOCK_MONOTONIC` |
| `o_creat` | `O_CREAT` flag |
| `o_trunc` | `O_TRUNC` flag |
| `o_nonblock` | `O_NONBLOCK` flag |
| `o_excl` | `O_EXCL` flag |
| `o_directory` | `O_DIRECTORY` flag |

The 5 file flags (`o_creat`, `o_trunc`, `o_nonblock`, `o_excl`, `o_directory`) are exposed to callers as runtime functions in `sys::consts` (e.g. `sys::o_creat()`) rather than compile-time constants, since their numeric values differ across platforms.
