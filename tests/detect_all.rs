use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering as StdOrdering};
use hardware::sys;
use hardware::sys::{detect_arch, Architecture};

fn configure_linux_tables() {
    let arch = detect_arch();
    match arch {
        Architecture::X86_64 => {
            sys::set_syscall_nrs(&sys::SyscallNrTable {
                read: 0,
                write: 1,
                openat: 257,
                close: 3,
                mmap: 9,
                munmap: 11,
                ioctl: 16,
                sched_yield: 24,
                nanosleep: 35,
                clone: 56,
                exit: 60,
                wait4: 61,
                kill: 62,
                fsync: 74,
                unlinkat: 263,
                getdents64: 217,
                clock_gettime: 228,
                sched_setaffinity: 203,
                sched_getaffinity: 204,
                stat: 4,
                socket: 41,
                connect: 42,
                accept: 43,
                bind: 49,
                listen: 50,
                execve: 59,
                fcntl: 72,
                getcwd: 79,
                rt_sigaction: 13,
                iopl: 172,
                mkdirat: 258,
                sysinfo: 99,
            });
            sys::set_os_constants(&sys::OsConstants {
                at_fdcwd: -100,
                sigchld: 17,
                map_private_anon: 0x22,
                map_shared_anon: 0x21,
                map_shared: 0x01,
                prot_read_write: 0x3,
                clock_monotonic: 1,
                o_creat: 0o100,
                o_trunc: 0o1000,
                o_nonblock: 0o4000,
                o_excl: 0o200,
                o_directory: 0o200000,
            });
        }
        Architecture::AArch64 => {
            sys::set_syscall_nrs(&sys::SyscallNrTable {
                read: 63,
                write: 64,
                openat: 56,
                close: 57,
                mmap: 222,
                munmap: 215,
                ioctl: 29,
                sched_yield: 124,
                nanosleep: 101,
                clone: 220,
                exit: 93,
                wait4: 260,
                kill: 129,
                fsync: 82,
                unlinkat: 35,
                getdents64: 61,
                clock_gettime: 113,
                sched_setaffinity: 122,
                sched_getaffinity: 123,
                stat: 79,
                socket: 198,
                connect: 203,
                accept: 202,
                bind: 200,
                listen: 201,
                execve: 221,
                fcntl: 25,
                getcwd: 17,
                rt_sigaction: 134,
                iopl: -1,
                mkdirat: 34,
                sysinfo: 179,
            });
            sys::set_os_constants(&sys::OsConstants {
                at_fdcwd: -100,
                sigchld: 17,
                map_private_anon: 0x22,
                map_shared_anon: 0x21,
                map_shared: 0x01,
                prot_read_write: 0x3,
                clock_monotonic: 1,
                o_creat: 0o100,
                o_trunc: 0o1000,
                o_nonblock: 0o4000,
                o_excl: 0o200,
                o_directory: 0o200000,
            });
        }
        _ => {}
    }
    hardware::sys::gpu::set_drm_constants(hardware::sys::gpu::DrmConstants {
        ioctl_version: 0xC040_6400,
        ioctl_gem_close: 0x4008_6409,
        ioctl_radeon_info: 0xC010_6467,
        ioctl_radeon_gem_info: 0xC018_645C,
        ioctl_radeon_gem_create: 0xC020_645D,
        ioctl_radeon_gem_mmap: 0xC020_645E,
        ioctl_radeon_gem_wait_idle: 0x4008_6464,
        ioctl_radeon_cs: 0xC020_6466,
        radeon_info_device_id: 0x00,
        radeon_info_num_gb_pipes: 0x01,
        radeon_info_vram_usage: 0x1E,
        radeon_info_active_cu_count: 0x20,
        radeon_info_current_gpu_sclk: 0x22,
        radeon_info_current_gpu_mclk: 0x23,
        radeon_info_current_gpu_temp: 0x21,
        radeon_info_max_se: 0x12,
        radeon_info_max_sh_per_se: 0x13,
        radeon_gem_domain_vram: 0x4,
        radeon_gem_domain_gtt: 0x2,
        radeon_chunk_id_relocs: 0x01,
        radeon_chunk_id_ib: 0x02,
        radeon_chunk_id_flags: 0x03,
        radeon_cs_ring_gfx: 0,
        radeon_cs_use_vm: 0x02,
    });
}

static INIT_DONE: AtomicBool = AtomicBool::new(false);
static INIT_COMPLETE: AtomicBool = AtomicBool::new(false);
static TEST_LOCK: AtomicBool = AtomicBool::new(false);
static mut LOG_BUF: [u8; 8192] = [0u8; 8192];
const LOG_BUF_CAP: usize = 8192;
static LOG_POS: AtomicUsize = AtomicUsize::new(0);

fn acquire_test_lock() {
    while TEST_LOCK
        .compare_exchange_weak(false, true, StdOrdering::Acquire, StdOrdering::Relaxed)
        .is_err()
    {
        core::hint::spin_loop();
    }
    LOG_POS.store(0, StdOrdering::Relaxed);
}

fn release_test_lock() {
    let len = LOG_POS.load(StdOrdering::Relaxed);
    if len > 0 {
        let ptr = core::ptr::addr_of!(LOG_BUF).cast::<u8>();
        let buf = unsafe { core::slice::from_raw_parts(ptr, len) };
        sys::write_stderr(buf);
    }
    LOG_POS.store(0, StdOrdering::Relaxed);
    TEST_LOCK.store(false, StdOrdering::Release);
}

fn ensure_init() {
    if INIT_DONE
        .compare_exchange(false, true, StdOrdering::AcqRel, StdOrdering::Acquire)
        .is_ok()
    {
        hardware::sys::init_shims();
        configure_linux_tables();
        sys::request_hw_privilege();
        INIT_COMPLETE.store(true, StdOrdering::Release);
        return;
    }
    while !INIT_COMPLETE.load(StdOrdering::Acquire) {
        core::hint::spin_loop();
    }
}

fn log(s: &str) {
    let bytes = s.as_bytes();
    let pos = LOG_POS.load(StdOrdering::Relaxed);
    let end = (pos + bytes.len()).min(LOG_BUF_CAP);
    let count = end - pos;
    unsafe {
        let dst = core::ptr::addr_of_mut!(LOG_BUF).cast::<u8>().add(pos);
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, count);
    }
    LOG_POS.store(end, StdOrdering::Relaxed);
}
fn log_u64(n: u64) {
    let mut tmp = [0u8; 20];
    let mut val = n;
    let mut i = 20;
    if val == 0 {
        log("0");
        return;
    }
    while val > 0 {
        i -= 1;
        tmp[i] = b'0' + (val % 10) as u8;
        val /= 10;
    }
    let s = unsafe { core::str::from_utf8_unchecked(&tmp[i..]) };
    log(s);
}
fn logln(s: &str) {
    log(s);
    log("\n");
}

fn log_hex_u16(n: u16) {
    let mut buf = [0u8; 4];
    let mut i = 0;
    while i < 4 {
        let digit = ((n >> (12 - i * 4)) & 0xF) as u8;
        buf[i] = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + digit - 10
        };
        i += 1;
    }
    let s = unsafe { core::str::from_utf8_unchecked(&buf) };
    log(s);
}

fn read_sysfs_u64(path: &[u8]) -> u64 {
    let fd = sys::sys_open(path, sys::O_RDONLY, 0);
    if fd < 0 {
        return 0;
    }
    let mut buf = [0u8; 64];
    let n = sys::sys_read_fd(fd, &mut buf);
    sys::sys_close(fd);
    if n <= 0 {
        return 0;
    }
    let mut val: u64 = 0;
    let mut i = 0;
    while i < n as usize {
        let b = buf[i];
        if !b.is_ascii_digit() {
            break;
        }
        val = val * 10 + (b - b'0') as u64;
        i += 1;
    }
    val
}

fn read_sysfs_max_freq_mhz() -> u64 {
    let paths: [&[u8]; 3] = [
        b"/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq\0",
        b"/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq\0",
        b"/sys/devices/system/cpu/cpu0/cpufreq/base_frequency\0",
    ];
    let mut i = 0;
    while i < 3 {
        let khz = read_sysfs_u64(paths[i]);
        if khz > 0 {
            return khz / 1000;
        }
        i += 1;
    }
    0
}

fn arch_name(a: Architecture) -> &'static str {
    match a {
        Architecture::X86_64 => "x86_64",
        Architecture::AArch64 => "aarch64",
        _ => "unknown",
    }
}

fn resolve_gpu_vendor(vendor_id: u16) -> &'static str {
    match vendor_id {
        0x1002 => "AMD/ATI GPU",
        0x10de => "NVIDIA GPU",
        0x8086 => "Intel GPU",
        0x1a03 => "ASPEED GPU",
        0x1ab8 => "Parallels GPU",
        0x15ad => "VMware GPU",
        0x1af4 => "VirtIO GPU",
        0x13B5 => "ARM Mali GPU",
        0x5143 => "Qualcomm Adreno GPU",
        0x14C3 => "Mediatek GPU",
        0x1010 => "Imagination PowerVR GPU",
        0x1131 => "Vivante GPU",
        _ => "GPU",
    }
}

fn read_sysfs_class_byte(path: &[u8]) -> u8 {
    let fd = sys::sys_open(path, sys::O_RDONLY, 0);
    if fd < 0 {
        return 0;
    }
    let mut buf = [0u8; 32];
    let n = sys::sys_read_fd(fd, &mut buf);
    sys::sys_close(fd);
    if n <= 0 {
        return 0;
    }
    let start = if n >= 2 && buf[0] == b'0' && (buf[1] == b'x' || buf[1] == b'X') {
        2
    } else {
        0
    };
    let hex_digit = |b: u8| -> u8 {
        match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => 0,
        }
    };
    if (start + 2) <= n as usize {
        hex_digit(buf[start]) << 4 | hex_digit(buf[start + 1])
    } else {
        0
    }
}

fn pci_class_name(class_byte: u8) -> &'static str {
    match class_byte {
        0x01 => "Storage",
        0x02 => "Network",
        0x03 => "Display/GPU",
        0x04 => "Multimedia",
        0x05 => "Memory",
        0x06 => "Bridge",
        0x07 => "Communication",
        0x08 => "System peripheral",
        0x0c => "Serial bus (USB/etc)",
        0x0d => "Wireless",
        _ => "Other",
    }
}

#[repr(C)]
struct LinuxDirent64 {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
}

fn raw_getdents64(fd: i64, buf: &mut [u8]) -> i64 {
    sys::sys_getdents64(fd, buf)
}

fn build_sysfs_path(name: &[u8], name_len: usize, suffix: &[u8], out: &mut [u8; 128]) -> usize {
    let prefix = b"/sys/bus/pci/devices/";
    let mut p = 0;
    let mut i = 0;
    while i < prefix.len() && p < 120 {
        out[p] = prefix[i];
        p += 1;
        i += 1;
    }
    i = 0;
    while i < name_len && p < 120 {
        out[p] = name[i];
        p += 1;
        i += 1;
    }
    i = 0;
    while i < suffix.len() && p < 127 {
        out[p] = suffix[i];
        p += 1;
        i += 1;
    }
    p
}

#[test]
fn detect_architecture() {
    ensure_init();
    acquire_test_lock();
    let arch = detect_arch();
    assert!(
        arch == Architecture::X86_64 || arch == Architecture::AArch64,
        "Architecture must be x86_64 or aarch64"
    );
    log("[OK]  architecture: ");
    logln(arch_name(arch));
    release_test_lock();
}

#[test]
fn detect_cpu_full() {
    ensure_init();
    acquire_test_lock();
    let info = hardware::sys::cpu::detect_cpu_info();
    assert!(info.is_some(), "CPU detection must find a CPU");
    let info = info.unwrap();

    let model = hardware::sys::cpu::api::model_name_str(&info);

    assert!(
        info.physical_cores > 0,
        "Must have at least 1 physical core"
    );
    assert!(
        info.logical_cores >= info.physical_cores,
        "Logical cores >= physical cores"
    );
    assert!(info.threads_per_core >= 1, "Threads per core >= 1");
    assert!(!info.vendor.is_empty(), "Vendor must not be empty");

    let sysfs_freq = read_sysfs_max_freq_mhz();

    logln("[OK]  cpu");
    log("      vendor        : ");
    logln(info.vendor);
    log("      model         : ");
    logln(if model.is_empty() { "(unknown)" } else { model });
    log("      physical cores: ");
    log_u64(info.physical_cores as u64);
    log("\n");
    log("      logical cores : ");
    log_u64(info.logical_cores as u64);
    log("\n");
    log("      threads/core  : ");
    log_u64(info.threads_per_core as u64);
    log("\n");
    if info.frequency_hz > 0 {
        log("      frequency     : ");
        log_u64(info.frequency_hz / 1_000_000);
        logln(" MHz (cpuid)");
    } else {
        logln("      frequency     : unknown via cpuid");
    }
    if sysfs_freq > 0 {
        log("      frequency     : ");
        log_u64(sysfs_freq);
        logln(" MHz (sysfs)");
    }
    log("      HyperThreading: ");
    logln(if info.has_ht { "yes" } else { "no" });
    log("      L1 cache      : ");
    log_u64(info.l1_cache_kb as u64);
    logln(" KB");
    log("      L2 cache      : ");
    log_u64(info.l2_cache_kb as u64);
    logln(" KB");
    log("      L3 cache      : ");
    log_u64(info.l3_cache_kb as u64);
    logln(" KB");
    release_test_lock();
}

#[test]
fn detect_cpu_cores() {
    ensure_init();
    acquire_test_lock();
    let mut cores = [hardware::sys::cpu::api::CoreInfo {
        core_id: 0,
        frequency_hz: 0,
        raw_temp: None,
    }; 64];
    let found = hardware::sys::cpu::api::detect_cores(&mut cores);
    assert!(found > 0, "Must detect at least 1 core");
    let sysfs_freq = read_sysfs_max_freq_mhz();
    log("[OK]  cpu_cores: ");
    log_u64(found as u64);
    logln(" thread(s) detected");
    let mut i = 0;
    while i < found && i < cores.len() {
        let freq = if cores[i].frequency_hz > 0 {
            cores[i].frequency_hz / 1_000_000
        } else {
            sysfs_freq
        };
        log("      thread ");
        log_u64(cores[i].core_id as u64 + 1);
        log(": freq=");
        log_u64(freq);
        logln("MHz");
        i += 1;
    }
    release_test_lock();
}

#[test]
fn detect_topology() {
    ensure_init();
    acquire_test_lock();
    let topo = hardware::sys::topology::detect_topology();
    assert!(topo.sockets > 0, "Must detect at least 1 socket");
    assert!(
        topo.cores_per_socket > 0,
        "Must detect at least 1 core per socket"
    );
    log("[OK]  topology: ");
    log_u64(topo.sockets as u64);
    log(" socket(s), ");
    log_u64(topo.cores_per_socket as u64);
    logln(" physical cores/socket");
    release_test_lock();
}

#[test]
fn detect_system_topology() {
    ensure_init();
    acquire_test_lock();
    let sys = hardware::sys::topology::system::detect();
    assert!(sys.total_cores > 0, "System must have at least 1 core");
    assert!(sys.total_threads > 0, "System must have at least 1 thread");
    assert!(sys.total_threads >= sys.total_cores, "Threads >= cores");
    log("[OK]  system: ");
    log_u64(sys.sockets as u64);
    log(" sockets, ");
    log_u64(sys.total_cores as u64);
    log(" physical cores, ");
    log_u64(sys.total_threads as u64);
    logln(" threads");
    release_test_lock();
}

#[test]
fn detect_ram() {
    ensure_init();
    acquire_test_lock();
    let mem = hardware::sys::detect_memory_info();
    assert!(mem.is_some(), "Must detect RAM");
    let mem = mem.unwrap();
    assert!(mem.total_bytes > 0, "Must detect RAM");
    let gb = mem.total_bytes / (1024 * 1024 * 1024);
    let mb_rem = (mem.total_bytes % (1024 * 1024 * 1024)) / (1024 * 1024);
    log("[OK]  ram: ");
    log_u64(gb);
    log(".");
    log_u64(mb_rem / 100);
    log(" GB (");
    log_u64(mem.total_bytes);
    logln(" bytes)");
    release_test_lock();
}

#[test]
fn detect_gpu() {
    ensure_init();
    acquire_test_lock();
    let mut gpus = [hardware::sys::gpu::GpuDevice {
        bus: 0,
        device: 0,
        function: 0,
        vendor_id: 0,
        device_id: 0,
        class: 0,
        subclass: 0,
        prog_if: 0,
        bar0: 0,
    }; 8];
    let count = hardware::sys::gpu::detect_gpus(&mut gpus);
    if count == 0 {
        if !sys::has_hw_privilege() {
            logln("[--]  gpu: skipped (no I/O port access, need root)");
        } else {
            logln("[--]  gpu: none detected");
        }
    } else {
        log("[OK]  gpu: ");
        log_u64(count as u64);
        logln(" device(s)");
        let mut i = 0;
        while i < count {
            log("      ");
            log(resolve_gpu_vendor(gpus[i].vendor_id));
            log(" [0x");
            log_hex_u16(gpus[i].vendor_id);
            log(":0x");
            log_hex_u16(gpus[i].device_id);
            log("]");
            if gpus[i].vendor_id == 0x13B5 && gpus[i].device_id != 0x0001 {
                log(" ");
                log(hardware::sys::gpu::mali_product_name(gpus[i].device_id));
            } else if gpus[i].vendor_id == 0x5143 && gpus[i].device_id != 0x0001 {
                log(" ");
                log(hardware::sys::gpu::adreno_product_name(gpus[i].device_id));
            }
            logln("");
            i += 1;
        }
    }
    release_test_lock();
}

#[test]
fn detect_pci_devices_summary() {
    ensure_init();
    acquire_test_lock();
    let class_ids: [u8; 11] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x0c, 0x0d, 0xFF,
    ];
    let mut counts = [0u32; 11];
    let mut total: u32 = 0;

    let fd = sys::sys_open(b"/sys/bus/pci/devices\0", sys::O_RDONLY, 0);
    if fd >= 0 {
        let mut dirbuf = [0u8; 4096];
        loop {
            let n = raw_getdents64(fd, &mut dirbuf);
            if n <= 0 {
                break;
            }
            let n = n as usize;
            let mut offset = 0;
            while offset < n {
                let entry = unsafe { &*(dirbuf.as_ptr().add(offset) as *const LinuxDirent64) };
                let reclen = entry.d_reclen as usize;
                if reclen == 0 {
                    break;
                }

                let name_start = offset + 19;
                let mut name_end = name_start;
                while name_end < offset + reclen && dirbuf[name_end] != 0 {
                    name_end += 1;
                }
                let name_len = name_end - name_start;

                if name_len > 2 && dirbuf[name_start] != b'.' {
                    let mut path = [0u8; 128];
                    let plen =
                        build_sysfs_path(&dirbuf[name_start..], name_len, b"/class\0", &mut path);
                    let class_byte = read_sysfs_class_byte(&path[..plen]);

                    let mut found = false;
                    let mut ci = 0;
                    while ci < 10 {
                        if class_byte == class_ids[ci] {
                            counts[ci] += 1;
                            found = true;
                            break;
                        }
                        ci += 1;
                    }
                    if !found {
                        counts[10] += 1;
                    }
                    total += 1;
                }
                offset += reclen;
            }
        }
        sys::sys_close(fd);
    }

    log("[OK]  pci: ");
    log_u64(total as u64);
    logln(" devices");
    let mut i = 0;
    while i < 11 {
        if counts[i] > 0 {
            let name = pci_class_name(class_ids[i]);
            log("      ");
            log(name);
            let pad = 20usize.saturating_sub(name.len());
            let mut p = 0;
            while p < pad {
                log(" ");
                p += 1;
            }
            log(" : ");
            log_u64(counts[i] as u64);
            log("\n");
        }
        i += 1;
    }
    release_test_lock();
}

#[test]
fn detect_cpu_features() {
    ensure_init();
    acquire_test_lock();
    let arch = hardware::sys::detect_arch();
    let x86_features: &[&str] = &["sse", "sse2", "avx", "avx2", "aes", "fma"];
    let arm_features: &[&str] = &["asimd", "aes", "sha1", "sha2", "crc32", "atomics", "fp"];
    let features: &[&str] = match arch {
        hardware::sys::Architecture::X86_64 => x86_features,
        hardware::sys::Architecture::AArch64 => arm_features,
        _ => x86_features,
    };
    logln("[OK]  cpu features:");
    let mut i = 0;
    while i < features.len() {
        let has = hardware::sys::cpu::features::has_feature(features[i]);
        log("      ");
        log(features[i]);
        log(": ");
        logln(if has { "yes" } else { "no" });
        i += 1;
    }
    release_test_lock();
}

#[test]
fn detect_power_governor() {
    ensure_init();
    acquire_test_lock();
    let policy = hardware::sys::power::governor::Governor::get_policy();
    let name = match policy {
        hardware::sys::power::governor::GovernorPolicy::Performance => "performance",
        hardware::sys::power::governor::GovernorPolicy::Powersave => "powersave",
        hardware::sys::power::governor::GovernorPolicy::OnDemand => "ondemand",
        hardware::sys::power::governor::GovernorPolicy::Conservative => "conservative",
        hardware::sys::power::governor::GovernorPolicy::Schedutil => "schedutil",
        hardware::sys::power::governor::GovernorPolicy::Unknown => "unknown",
    };
    log("[OK]  power_governor: ");
    logln(name);
    release_test_lock();
}

#[test]
fn detect_all_components_summary() {
    ensure_init();
    acquire_test_lock();
    logln("\n========== Hardware Detection Summary ==========");

    let arch = detect_arch();
    log("  Architecture : ");
    logln(arch_name(arch));

    if let Some(info) = hardware::sys::cpu::detect_cpu_info() {
        let model = hardware::sys::cpu::api::model_name_str(&info);
        let freq = if info.frequency_hz > 0 {
            info.frequency_hz / 1_000_000
        } else {
            read_sysfs_max_freq_mhz()
        };
        log("  CPU          : ");
        log(info.vendor);
        log(" ");
        log(if model.is_empty() { "unknown" } else { model });
        log(" (");
        log_u64(info.physical_cores as u64);
        log(" phys / ");
        log_u64(info.logical_cores as u64);
        log(" logical @ ");
        log_u64(freq);
        logln("MHz)");
        log("  Caches       : L1=");
        log_u64(info.l1_cache_kb as u64);
        log("KB L2=");
        log_u64(info.l2_cache_kb as u64);
        log("KB L3=");
        log_u64(info.l3_cache_kb as u64);
        logln("KB");
    }

    let topo = hardware::sys::topology::system::detect();
    log("  Topology     : ");
    log_u64(topo.sockets as u64);
    log(" socket(s), ");
    log_u64(topo.total_cores as u64);
    log(" cores, ");
    log_u64(topo.total_threads as u64);
    logln(" threads");

    if let Some(mem) = hardware::sys::detect_memory_info() {
        log("  RAM          : ");
        log_u64(mem.total_bytes / (1024 * 1024 * 1024));
        logln(" GB");
    }

    let mut gpus = [hardware::sys::gpu::GpuDevice {
        bus: 0,
        device: 0,
        function: 0,
        vendor_id: 0,
        device_id: 0,
        class: 0,
        subclass: 0,
        prog_if: 0,
        bar0: 0,
    }; 8];
    let gpu_count = hardware::sys::gpu::detect_gpus(&mut gpus);
    if gpu_count == 0 {
        if sys::has_hw_privilege() {
            logln("  GPU          : none");
        } else {
            logln("  GPU          : skipped (no I/O port access)");
        }
    } else {
        let mut i = 0;
        while i < gpu_count {
            log("  GPU          : ");
            logln(resolve_gpu_vendor(gpus[i].vendor_id));
            i += 1;
        }
    }

    logln("=================================================\n");
    release_test_lock();
}
