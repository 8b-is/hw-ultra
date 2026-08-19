use crate::common::once::OnceCopy;
use core::sync::atomic::{AtomicI64, Ordering};

pub struct SyscallNrTable {
    pub read: i64,
    pub write: i64,
    pub openat: i64,
    pub close: i64,
    pub mmap: i64,
    pub munmap: i64,
    pub ioctl: i64,
    pub sched_yield: i64,
    pub nanosleep: i64,
    pub clone: i64,
    pub exit: i64,
    pub wait4: i64,
    pub kill: i64,
    pub fsync: i64,
    pub unlinkat: i64,
    pub getdents64: i64,
    pub clock_gettime: i64,
    pub sched_setaffinity: i64,
    pub sched_getaffinity: i64,
    pub stat: i64,
    pub socket: i64,
    pub connect: i64,
    pub accept: i64,
    pub bind: i64,
    pub listen: i64,
    pub execve: i64,
    pub fcntl: i64,
    pub getcwd: i64,
    pub rt_sigaction: i64,
    pub iopl: i64,
    pub mkdirat: i64,
    pub sysinfo: i64,
}

mod nr_defaults {
    use crate::common::error::ERR_NOT_IMPLEMENTED;
    pub const READ: i64 = ERR_NOT_IMPLEMENTED;
    pub const WRITE: i64 = ERR_NOT_IMPLEMENTED;
    pub const OPENAT: i64 = ERR_NOT_IMPLEMENTED;
    pub const CLOSE: i64 = ERR_NOT_IMPLEMENTED;
    pub const MMAP: i64 = ERR_NOT_IMPLEMENTED;
    pub const MUNMAP: i64 = ERR_NOT_IMPLEMENTED;
    pub const IOCTL: i64 = ERR_NOT_IMPLEMENTED;
    pub const SCHED_YIELD: i64 = ERR_NOT_IMPLEMENTED;
    pub const NANOSLEEP: i64 = ERR_NOT_IMPLEMENTED;
    pub const CLONE: i64 = ERR_NOT_IMPLEMENTED;
    pub const EXIT: i64 = ERR_NOT_IMPLEMENTED;
    pub const WAIT4: i64 = ERR_NOT_IMPLEMENTED;
    pub const KILL: i64 = ERR_NOT_IMPLEMENTED;
    pub const FSYNC: i64 = ERR_NOT_IMPLEMENTED;
    pub const UNLINKAT: i64 = ERR_NOT_IMPLEMENTED;
    pub const GETDENTS64: i64 = ERR_NOT_IMPLEMENTED;
    pub const CLOCK_GETTIME: i64 = ERR_NOT_IMPLEMENTED;
    pub const SCHED_SETAFFINITY: i64 = ERR_NOT_IMPLEMENTED;
    pub const SCHED_GETAFFINITY: i64 = ERR_NOT_IMPLEMENTED;
    pub const STAT: i64 = ERR_NOT_IMPLEMENTED;
    pub const SOCKET: i64 = ERR_NOT_IMPLEMENTED;
    pub const CONNECT: i64 = ERR_NOT_IMPLEMENTED;
    pub const ACCEPT: i64 = ERR_NOT_IMPLEMENTED;
    pub const BIND: i64 = ERR_NOT_IMPLEMENTED;
    pub const LISTEN: i64 = ERR_NOT_IMPLEMENTED;
    pub const EXECVE: i64 = ERR_NOT_IMPLEMENTED;
    pub const FCNTL: i64 = ERR_NOT_IMPLEMENTED;
    pub const GETCWD: i64 = ERR_NOT_IMPLEMENTED;
    pub const RT_SIGACTION: i64 = ERR_NOT_IMPLEMENTED;
    pub const IOPL: i64 = ERR_NOT_IMPLEMENTED;
    pub const MKDIRAT: i64 = ERR_NOT_IMPLEMENTED;
    pub const SYSINFO: i64 = ERR_NOT_IMPLEMENTED;
}

static NR_READ: AtomicI64 = AtomicI64::new(nr_defaults::READ);
static NR_WRITE: AtomicI64 = AtomicI64::new(nr_defaults::WRITE);
static NR_OPENAT: AtomicI64 = AtomicI64::new(nr_defaults::OPENAT);
static NR_CLOSE: AtomicI64 = AtomicI64::new(nr_defaults::CLOSE);
static NR_MMAP: AtomicI64 = AtomicI64::new(nr_defaults::MMAP);
static NR_MUNMAP: AtomicI64 = AtomicI64::new(nr_defaults::MUNMAP);
static NR_IOCTL: AtomicI64 = AtomicI64::new(nr_defaults::IOCTL);
static NR_SCHED_YIELD: AtomicI64 = AtomicI64::new(nr_defaults::SCHED_YIELD);
static NR_NANOSLEEP: AtomicI64 = AtomicI64::new(nr_defaults::NANOSLEEP);
static NR_CLONE: AtomicI64 = AtomicI64::new(nr_defaults::CLONE);
static NR_EXIT: AtomicI64 = AtomicI64::new(nr_defaults::EXIT);
static NR_WAIT4: AtomicI64 = AtomicI64::new(nr_defaults::WAIT4);
static NR_KILL: AtomicI64 = AtomicI64::new(nr_defaults::KILL);
static NR_FSYNC: AtomicI64 = AtomicI64::new(nr_defaults::FSYNC);
static NR_UNLINKAT: AtomicI64 = AtomicI64::new(nr_defaults::UNLINKAT);
static NR_GETDENTS64: AtomicI64 = AtomicI64::new(nr_defaults::GETDENTS64);
static NR_CLOCK_GETTIME: AtomicI64 = AtomicI64::new(nr_defaults::CLOCK_GETTIME);
static NR_SCHED_SETAFFINITY: AtomicI64 = AtomicI64::new(nr_defaults::SCHED_SETAFFINITY);
static NR_SCHED_GETAFFINITY: AtomicI64 = AtomicI64::new(nr_defaults::SCHED_GETAFFINITY);
static NR_STAT: AtomicI64 = AtomicI64::new(nr_defaults::STAT);
static NR_SOCKET: AtomicI64 = AtomicI64::new(nr_defaults::SOCKET);
static NR_CONNECT: AtomicI64 = AtomicI64::new(nr_defaults::CONNECT);
static NR_ACCEPT: AtomicI64 = AtomicI64::new(nr_defaults::ACCEPT);
static NR_BIND: AtomicI64 = AtomicI64::new(nr_defaults::BIND);
static NR_LISTEN: AtomicI64 = AtomicI64::new(nr_defaults::LISTEN);
static NR_EXECVE: AtomicI64 = AtomicI64::new(nr_defaults::EXECVE);
static NR_FCNTL: AtomicI64 = AtomicI64::new(nr_defaults::FCNTL);
static NR_GETCWD: AtomicI64 = AtomicI64::new(nr_defaults::GETCWD);
static NR_RT_SIGACTION: AtomicI64 = AtomicI64::new(nr_defaults::RT_SIGACTION);
static NR_IOPL: AtomicI64 = AtomicI64::new(nr_defaults::IOPL);
static NR_MKDIRAT: AtomicI64 = AtomicI64::new(nr_defaults::MKDIRAT);
static NR_SYSINFO: AtomicI64 = AtomicI64::new(nr_defaults::SYSINFO);

pub fn set_syscall_nrs(t: &SyscallNrTable) {
    NR_READ.store(t.read, Ordering::Release);
    NR_WRITE.store(t.write, Ordering::Release);
    NR_OPENAT.store(t.openat, Ordering::Release);
    NR_CLOSE.store(t.close, Ordering::Release);
    NR_MMAP.store(t.mmap, Ordering::Release);
    NR_MUNMAP.store(t.munmap, Ordering::Release);
    NR_IOCTL.store(t.ioctl, Ordering::Release);
    NR_SCHED_YIELD.store(t.sched_yield, Ordering::Release);
    NR_NANOSLEEP.store(t.nanosleep, Ordering::Release);
    NR_CLONE.store(t.clone, Ordering::Release);
    NR_EXIT.store(t.exit, Ordering::Release);
    NR_WAIT4.store(t.wait4, Ordering::Release);
    NR_KILL.store(t.kill, Ordering::Release);
    NR_FSYNC.store(t.fsync, Ordering::Release);
    NR_UNLINKAT.store(t.unlinkat, Ordering::Release);
    NR_GETDENTS64.store(t.getdents64, Ordering::Release);
    NR_CLOCK_GETTIME.store(t.clock_gettime, Ordering::Release);
    NR_SCHED_SETAFFINITY.store(t.sched_setaffinity, Ordering::Release);
    NR_SCHED_GETAFFINITY.store(t.sched_getaffinity, Ordering::Release);
    NR_STAT.store(t.stat, Ordering::Release);
    NR_SOCKET.store(t.socket, Ordering::Release);
    NR_CONNECT.store(t.connect, Ordering::Release);
    NR_ACCEPT.store(t.accept, Ordering::Release);
    NR_BIND.store(t.bind, Ordering::Release);
    NR_LISTEN.store(t.listen, Ordering::Release);
    NR_EXECVE.store(t.execve, Ordering::Release);
    NR_FCNTL.store(t.fcntl, Ordering::Release);
    NR_GETCWD.store(t.getcwd, Ordering::Release);
    NR_RT_SIGACTION.store(t.rt_sigaction, Ordering::Release);
    NR_IOPL.store(t.iopl, Ordering::Release);
    NR_MKDIRAT.store(t.mkdirat, Ordering::Release);
    NR_SYSINFO.store(t.sysinfo, Ordering::Release);
}

type RawSyscallFn = unsafe extern "C" fn(i64, u64, u64, u64, u64, u64, u64) -> i64;
static RAW_SYSCALL_FN: OnceCopy<RawSyscallFn> = OnceCopy::new();

pub fn set_raw_syscall_fn(f: RawSyscallFn) -> bool {
    RAW_SYSCALL_FN.set(f)
}

#[repr(C, align(4))]
struct AlignedBlob<const N: usize>([u8; N]);

#[cfg_attr(target_os = "macos", link_section = "__TEXT,__text")]
#[cfg_attr(not(target_os = "macos"), link_section = ".text")]
#[used]
static X86_64_SYSCALL_BLOB: AlignedBlob<26> = AlignedBlob([
    0x48, 0x89, 0xf8, 0x48, 0x89, 0xf7, 0x48, 0x89, 0xd6, 0x48, 0x89, 0xca, 0x4d, 0x89, 0xc2, 0x4d,
    0x89, 0xc8, 0x4c, 0x8b, 0x4c, 0x24, 0x08, 0x0f, 0x05, 0xc3,
]);

#[cfg_attr(target_os = "macos", link_section = "__TEXT,__text")]
#[cfg_attr(not(target_os = "macos"), link_section = ".text")]
#[used]
static AARCH64_SYSCALL_BLOB: AlignedBlob<36> = AlignedBlob([
    0xe8, 0x03, 0x00, 0xaa, 0xe0, 0x03, 0x01, 0xaa, 0xe1, 0x03, 0x02, 0xaa, 0xe2, 0x03, 0x03, 0xaa,
    0xe3, 0x03, 0x04, 0xaa, 0xe4, 0x03, 0x05, 0xaa, 0xe5, 0x03, 0x06, 0xaa, 0x01, 0x00, 0x00, 0xd4,
    0xc0, 0x03, 0x5f, 0xd6,
]);

pub(crate) fn register_native_syscall() {
    let arch = super::detect_arch();
    match arch {
        crate::arch::Architecture::X86_64 => {
            let f: RawSyscallFn = unsafe { core::mem::transmute(X86_64_SYSCALL_BLOB.0.as_ptr()) };
            RAW_SYSCALL_FN.set(f);
        }
        crate::arch::Architecture::AArch64 => {
            let f: RawSyscallFn = unsafe { core::mem::transmute(AARCH64_SYSCALL_BLOB.0.as_ptr()) };
            RAW_SYSCALL_FN.set(f);
        }
        _ => {}
    }
}

/// # Safety
/// Caller must ensure `nr` is a valid syscall number and arguments are correct.
pub unsafe fn raw_syscall(nr: i64, a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    if let Some(f) = RAW_SYSCALL_FN.get() {
        return f(nr, a0, a1, a2, a3, a4, a5);
    }
    super::init_shims();
    if let Some(f) = RAW_SYSCALL_FN.get() {
        return f(nr, a0, a1, a2, a3, a4, a5);
    }
    crate::common::error::ERR_NOT_IMPLEMENTED
}

pub fn nr_read() -> i64 {
    NR_READ.load(Ordering::Acquire)
}
pub fn nr_write() -> i64 {
    NR_WRITE.load(Ordering::Acquire)
}
pub fn nr_openat() -> i64 {
    NR_OPENAT.load(Ordering::Acquire)
}
pub fn nr_close() -> i64 {
    NR_CLOSE.load(Ordering::Acquire)
}
pub fn nr_mmap() -> i64 {
    NR_MMAP.load(Ordering::Acquire)
}
pub fn nr_munmap() -> i64 {
    NR_MUNMAP.load(Ordering::Acquire)
}
pub fn nr_ioctl() -> i64 {
    NR_IOCTL.load(Ordering::Acquire)
}
pub fn nr_sched_yield() -> i64 {
    NR_SCHED_YIELD.load(Ordering::Acquire)
}
pub fn nr_nanosleep() -> i64 {
    NR_NANOSLEEP.load(Ordering::Acquire)
}
pub fn nr_clone() -> i64 {
    NR_CLONE.load(Ordering::Acquire)
}
pub fn nr_exit() -> i64 {
    NR_EXIT.load(Ordering::Acquire)
}
pub fn nr_wait4() -> i64 {
    NR_WAIT4.load(Ordering::Acquire)
}
pub fn nr_kill() -> i64 {
    NR_KILL.load(Ordering::Acquire)
}
pub fn nr_fsync() -> i64 {
    NR_FSYNC.load(Ordering::Acquire)
}
pub fn nr_unlinkat() -> i64 {
    NR_UNLINKAT.load(Ordering::Acquire)
}
pub fn nr_getdents64() -> i64 {
    NR_GETDENTS64.load(Ordering::Acquire)
}
pub fn nr_clock_gettime() -> i64 {
    NR_CLOCK_GETTIME.load(Ordering::Acquire)
}
pub fn nr_sched_setaffinity() -> i64 {
    NR_SCHED_SETAFFINITY.load(Ordering::Acquire)
}
pub fn nr_sched_getaffinity() -> i64 {
    NR_SCHED_GETAFFINITY.load(Ordering::Acquire)
}
pub fn nr_stat() -> i64 {
    NR_STAT.load(Ordering::Acquire)
}
pub fn nr_socket() -> i64 {
    NR_SOCKET.load(Ordering::Acquire)
}
pub fn nr_connect() -> i64 {
    NR_CONNECT.load(Ordering::Acquire)
}
pub fn nr_accept() -> i64 {
    NR_ACCEPT.load(Ordering::Acquire)
}
pub fn nr_bind() -> i64 {
    NR_BIND.load(Ordering::Acquire)
}
pub fn nr_listen() -> i64 {
    NR_LISTEN.load(Ordering::Acquire)
}
pub fn nr_execve() -> i64 {
    NR_EXECVE.load(Ordering::Acquire)
}
pub fn nr_fcntl() -> i64 {
    NR_FCNTL.load(Ordering::Acquire)
}
pub fn nr_getcwd() -> i64 {
    NR_GETCWD.load(Ordering::Acquire)
}
pub fn nr_rt_sigaction() -> i64 {
    NR_RT_SIGACTION.load(Ordering::Acquire)
}
pub fn nr_iopl() -> i64 {
    NR_IOPL.load(Ordering::Acquire)
}
pub fn nr_mkdirat() -> i64 {
    NR_MKDIRAT.load(Ordering::Acquire)
}
pub fn nr_sysinfo() -> i64 {
    NR_SYSINFO.load(Ordering::Acquire)
}
