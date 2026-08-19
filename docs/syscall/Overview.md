# Syscall Module

The `syscall` module provides a syscall dispatch table for registering and handling system calls.

## Submodules

| File | Description |
|------|-------------|
| `api.rs` | Syscall registration, dispatch, and result types |

## SyscallResult

```rust
pub enum SyscallResult {
    Ok(usize),   // Successful, returns value
    Denied,      // Permission denied
    NotFound,    // Syscall number not registered
    Fault,       // Handler faulted
}
```

## API

| Function | Description |
|----------|-------------|
| `register_syscall(nr, handler)` | Registers a handler for syscall number `nr` |
| `handle_syscall(nr, arg0, arg1, arg2)` | Dispatches a syscall with 3 arguments |

## Limits

- Maximum 256 syscalls (`MAX_SYSCALLS`)

## Dispatch flow

1. `handle_syscall(nr, arg0, arg1, arg2)` is called
2. Looks up handler in the 256-entry dispatch table
3. If registered, calls the handler and returns `Ok(result)`
4. If not registered, returns `NotFound`

## Integration with arch shims

The crate uses 33 `AtomicI64` statics in `SyscallNrTable` that store syscall numbers for platform operations (read, write, open, close, mmap, ioctl, iopl, mkdirat, sysinfo, etc.). All default to `ERR_NOT_IMPLEMENTED` (`-1`) — no platform-specific magic numbers. These are populated by the consumer (OS or test) via `set_syscall_nrs()`.

The raw syscall handler is auto-registered by `init_shims()` via native machine code blobs (`X86_64_SYSCALL_BLOB` / `AARCH64_SYSCALL_BLOB`). The blobs use `extern "C"` calling convention and execute the architecture's syscall instruction directly. `set_raw_syscall_fn()` is available as an optional override.

## Safety considerations

- See [Warnings.md](../Warnings.md) warning 12 for raw syscall handler safety
- Handler addresses are stored as `usize` — the caller must ensure they point to valid functions
