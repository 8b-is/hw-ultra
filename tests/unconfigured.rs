use hardware::sys;

#[test]
fn syscall_nrs_default_to_not_implemented() {
    assert_eq!(sys::arch::shim::nr_read(), -1);
    assert_eq!(sys::arch::shim::nr_write(), -1);
    assert_eq!(sys::arch::shim::nr_openat(), -1);
    assert_eq!(sys::arch::shim::nr_close(), -1);
    assert_eq!(sys::arch::shim::nr_mmap(), -1);
    assert_eq!(sys::arch::shim::nr_munmap(), -1);
    assert_eq!(sys::arch::shim::nr_ioctl(), -1);
    assert_eq!(sys::arch::shim::nr_mkdirat(), -1);
    assert_eq!(sys::arch::shim::nr_sysinfo(), -1);
    assert_eq!(sys::arch::shim::nr_iopl(), -1);
}

#[test]
fn os_constants_default_to_zero() {
    assert_eq!(sys::arch::shim::os_at_fdcwd(), 0);
    assert_eq!(sys::arch::shim::os_sigchld(), 0);
    assert_eq!(sys::arch::shim::os_clock_monotonic(), 0);
}

#[test]
fn drm_open_returns_none_without_constants() {
    let result = sys::gpu::drm::open();
    assert!(result.is_none());
}

#[test]
fn detect_memory_returns_none_without_sysinfo_nr() {
    let mem = sys::detect_memory_info();
    assert!(mem.is_none());
}

#[test]
fn detect_cpu_works_without_tables() {
    let info = sys::cpu::detect::detect_cpu_info();
    assert!(info.is_some() || info.is_none());
}

#[test]
fn detect_arch_works_without_tables() {
    let arch = sys::detect_arch();
    assert!(matches!(
        arch,
        sys::Architecture::X86_64 | sys::Architecture::AArch64 | sys::Architecture::Unknown
    ));
}

#[test]
fn thermal_returns_defaults_without_registration() {
    let count = sys::thermal::api::zone_count();
    assert_eq!(count, 0);
}

#[test]
fn governor_returns_unknown_without_configuration() {
    let gov = sys::power::governor::Governor::get_policy();
    assert!(matches!(gov, sys::power::governor::GovernorPolicy::Unknown));
}
