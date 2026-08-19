use core::sync::atomic::{AtomicI64, Ordering};

pub struct OsConstants {
    pub at_fdcwd: i64,
    pub sigchld: i64,
    pub map_private_anon: i64,
    pub map_shared_anon: i64,
    pub map_shared: i64,
    pub prot_read_write: i64,
    pub clock_monotonic: i64,
    pub o_creat: i64,
    pub o_trunc: i64,
    pub o_nonblock: i64,
    pub o_excl: i64,
    pub o_directory: i64,
}

static OS_AT_FDCWD: AtomicI64 = AtomicI64::new(0);
static OS_SIGCHLD: AtomicI64 = AtomicI64::new(0);
static OS_MAP_PRIVATE_ANON: AtomicI64 = AtomicI64::new(0);
static OS_MAP_SHARED_ANON: AtomicI64 = AtomicI64::new(0);
static OS_MAP_SHARED: AtomicI64 = AtomicI64::new(0);
static OS_PROT_READ_WRITE: AtomicI64 = AtomicI64::new(0);
static OS_CLOCK_MONOTONIC: AtomicI64 = AtomicI64::new(0);
static OS_O_CREAT: AtomicI64 = AtomicI64::new(0);
static OS_O_TRUNC: AtomicI64 = AtomicI64::new(0);
static OS_O_NONBLOCK: AtomicI64 = AtomicI64::new(0);
static OS_O_EXCL: AtomicI64 = AtomicI64::new(0);
static OS_O_DIRECTORY: AtomicI64 = AtomicI64::new(0);

pub fn set_os_constants(c: &OsConstants) {
    OS_AT_FDCWD.store(c.at_fdcwd, Ordering::Release);
    OS_SIGCHLD.store(c.sigchld, Ordering::Release);
    OS_MAP_PRIVATE_ANON.store(c.map_private_anon, Ordering::Release);
    OS_MAP_SHARED_ANON.store(c.map_shared_anon, Ordering::Release);
    OS_MAP_SHARED.store(c.map_shared, Ordering::Release);
    OS_PROT_READ_WRITE.store(c.prot_read_write, Ordering::Release);
    OS_CLOCK_MONOTONIC.store(c.clock_monotonic, Ordering::Release);
    OS_O_CREAT.store(c.o_creat, Ordering::Release);
    OS_O_TRUNC.store(c.o_trunc, Ordering::Release);
    OS_O_NONBLOCK.store(c.o_nonblock, Ordering::Release);
    OS_O_EXCL.store(c.o_excl, Ordering::Release);
    OS_O_DIRECTORY.store(c.o_directory, Ordering::Release);
}

pub fn os_at_fdcwd() -> i64 {
    OS_AT_FDCWD.load(Ordering::Acquire)
}
pub fn os_sigchld() -> i64 {
    OS_SIGCHLD.load(Ordering::Acquire)
}
pub fn os_map_private_anon() -> i64 {
    OS_MAP_PRIVATE_ANON.load(Ordering::Acquire)
}
pub fn os_map_shared_anon() -> i64 {
    OS_MAP_SHARED_ANON.load(Ordering::Acquire)
}
pub fn os_map_shared() -> i64 {
    OS_MAP_SHARED.load(Ordering::Acquire)
}
pub fn os_prot_read_write() -> i64 {
    OS_PROT_READ_WRITE.load(Ordering::Acquire)
}
pub fn os_clock_monotonic() -> i64 {
    OS_CLOCK_MONOTONIC.load(Ordering::Acquire)
}
pub fn os_o_creat() -> i64 {
    OS_O_CREAT.load(Ordering::Acquire)
}
pub fn os_o_trunc() -> i64 {
    OS_O_TRUNC.load(Ordering::Acquire)
}
pub fn os_o_nonblock() -> i64 {
    OS_O_NONBLOCK.load(Ordering::Acquire)
}
pub fn os_o_excl() -> i64 {
    OS_O_EXCL.load(Ordering::Acquire)
}
pub fn os_o_directory() -> i64 {
    OS_O_DIRECTORY.load(Ordering::Acquire)
}
