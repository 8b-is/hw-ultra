mod arch_detect;
mod functions;
pub(crate) mod os;
pub mod privilege;
mod syscall;

pub use arch_detect::arch_cached;
pub use arch_detect::detect_arch;
pub use arch_detect::set_arch;

pub use functions::{
    arch_exit, cpuid_count, init_shims, mkdir, mmio_read32, mmio_write32, read_aarch64_midr,
    read_msr, scan_dir_dispatch, set_mkdir_fn, set_scan_dir_fn,
};
pub use functions::{
    set_cpuid_fn, set_mmio_read32_fn, set_mmio_write32_fn, set_read_aarch64_midr_fn,
    set_read_msr_fn,
};

pub use os::{
    os_at_fdcwd, os_clock_monotonic, os_map_private_anon, os_map_shared, os_map_shared_anon,
    os_o_creat, os_o_directory, os_o_excl, os_o_nonblock, os_o_trunc, os_prot_read_write,
    os_sigchld, set_os_constants, OsConstants,
};

pub use syscall::{
    nr_accept, nr_bind, nr_clock_gettime, nr_clone, nr_close, nr_connect, nr_execve, nr_fcntl,
    nr_fsync, nr_getcwd, nr_getdents64, nr_ioctl, nr_iopl, nr_kill, nr_listen, nr_mkdirat, nr_mmap,
    nr_munmap, nr_nanosleep, nr_openat, nr_read, nr_rt_sigaction, nr_sched_getaffinity,
    nr_sched_setaffinity, nr_sched_yield, nr_socket, nr_stat, nr_sysinfo, nr_unlinkat, nr_wait4,
    nr_write, set_syscall_nrs, SyscallNrTable,
};
pub use syscall::{raw_syscall, set_raw_syscall_fn};
