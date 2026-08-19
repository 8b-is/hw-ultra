use core::sync::atomic::{AtomicBool, Ordering as StdOrdering};
use hardware::sys;

fn configure_linux_tables() {
    let arch = sys::detect_arch();
    match arch {
        sys::Architecture::X86_64 => {
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
        sys::Architecture::AArch64 => {
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
    sys::write_stderr_str(s);
}

fn log_u64(n: u64) {
    sys::write_stderr_u64(n);
}

fn logln(s: &str) {
    log(s);
    log("\n");
}

fn log_hex(mut n: u64) {
    let mut buf = [0u8; 16];
    if n == 0 {
        log("0");
        return;
    }
    let mut pos = 16;
    while n > 0 && pos > 0 {
        pos -= 1;
        let digit = (n & 0xF) as u8;
        buf[pos] = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + digit - 10
        };
        n >>= 4;
    }
    sys::write_stderr(&buf[pos..16]);
}

fn cpu_busy_until(end_ns: u64) {
    let mut x: u64 = 1;
    while sys::monotonic_ns() < end_ns {
        let mut i = 0;
        while i < 1000 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            i += 1;
        }
        core::hint::black_box(x);
    }
}

fn elapsed_ms(start: u64, end: u64) -> u64 {
    (end - start) / 1_000_000
}

#[test]
fn stress_sequential_guardian_enforcement() {
    ensure_init();
    let cpu = hardware::sys::cpu::detect_cpu_info().expect("CPU detection must succeed");
    let mem = hardware::sys::detect_memory_info().expect("Memory detection must succeed");
    let total_cpus = cpu.logical_cores as usize;

    logln("\n========== Guardian Enforcement Stress Test (100% attempt) ==========");
    log("  CPU: ");
    log(cpu.vendor);
    log(" ");
    log(hardware::sys::cpu::api::model_name_str(&cpu));
    log(" (");
    log_u64(cpu.physical_cores as u64);
    log(" phys / ");
    log_u64(cpu.logical_cores as u64);
    logln(" logical)");

    log("  RAM: ");
    log_u64(mem.total_bytes / (1024 * 1024 * 1024));
    log(" GB total, ");
    log_u64(mem.available_bytes / (1024 * 1024 * 1024));
    logln(" GB available");

    log("  Swap: ");
    log_u64(mem.swap_total_bytes / (1024 * 1024 * 1024));
    log(" GB total, ");
    log_u64(mem.swap_free_bytes / (1024 * 1024 * 1024));
    logln(" GB free");

    log("  Caches: L1=");
    log_u64(cpu.l1_cache_kb as u64);
    log("KB L2=");
    log_u64(cpu.l2_cache_kb as u64);
    log("KB L3=");
    log_u64(cpu.l3_cache_kb as u64);
    logln("KB");

    hardware::sys::runtime::resources::set_memory_capacity(mem.available_bytes);
    hardware::sys::runtime::resources::set_swap_capacity(mem.swap_total_bytes);
    hardware::sys::runtime::resources::set_cpu_capacity(cpu.logical_cores as u64);
    logln("  Guardian: memory limit 80% of available, swap limit 50%, CPU limit 80%");

    logln("\n--- Phase 1: CPU (100% attempt — guardian should cap at 80%) ---");
    let num_workers = total_cpus;
    let duration_ns: u64 = 3_000_000_000;
    log("  Attempting to fork ");
    log_u64(num_workers as u64);
    log(" workers for 3s (");
    log_u64(num_workers as u64);
    log("/");
    log_u64(total_cpus as u64);
    logln(" cores)...");

    let start = sys::monotonic_ns();
    let end_target = start + duration_ns;

    let mut pids = [0i64; 256];
    let mut pid_count = 0usize;
    let mut guardian_cpu_blocked = false;
    let mut i = 0;
    while i < num_workers && pid_count < 256 {
        let pid = sys::fork();
        if pid == 0 {
            cpu_busy_until(end_target);
            sys::exit(0);
        } else if pid > 0 {
            pids[pid_count] = pid;
            pid_count += 1;
        } else {
            guardian_cpu_blocked = true;
            log("  Guardian: CPU limit (80%) reached after ");
            log_u64(pid_count as u64);
            logln(" forks — blocked");
            break;
        }
        i += 1;
    }

    let mut j = 0;
    while j < pid_count {
        sys::waitpid(pids[j]);
        hardware::sys::runtime::resources::free_cpu(1);
        j += 1;
    }
    let cpu_elapsed = sys::monotonic_ns() - start;
    log("  CPU: ");
    log_u64(pid_count as u64);
    log("/");
    log_u64(num_workers as u64);
    log(" workers forked in ");
    log_u64(elapsed_ms(0, cpu_elapsed));
    if guardian_cpu_blocked {
        logln("ms (guardian enforced 80% CPU limit)");
    } else {
        logln("ms");
    }
    if total_cpus > 1 {
        assert!(guardian_cpu_blocked, "guardian must block CPU at 80%");
    }

    logln("\n--- Phase 2: RAM (100% attempt — guardian should cap at 80%) ---");
    let target_bytes = mem.total_bytes as usize;
    let page_size: usize = 4096;
    let chunk_size: usize = 64 * 1024 * 1024;
    let num_chunks = (target_bytes / chunk_size).max(1);
    let actual_bytes = num_chunks * chunk_size;
    log("  Mapping ");
    log_u64((actual_bytes / (1024 * 1024)) as u64);
    log(" MB (");
    log_u64(num_chunks as u64);
    logln(" chunks of 64MB)...");

    let start = sys::monotonic_ns();
    let mut chunks = [core::ptr::null_mut::<u8>(); 1024];
    let mut alloc_count = 0usize;
    let mut guardian_blocked = false;
    let mut ci = 0;
    while ci < num_chunks && ci < 1024 {
        if !hardware::sys::runtime::resources::try_alloc_memory(chunk_size) {
            log("  Guardian: memory limit (80%) reached after ");
            log_u64((ci * chunk_size / (1024 * 1024)) as u64);
            logln(" MB — stopping allocation");
            guardian_blocked = true;
            break;
        }
        let ptr = sys::sys_mmap_anon(chunk_size);
        if ptr.is_null() {
            hardware::sys::runtime::resources::free_memory(chunk_size);
            break;
        }
        let pattern = (ci & 0xFF) as u8;
        let mut offset = 0;
        while offset < chunk_size {
            unsafe {
                *ptr.add(offset) = pattern;
            }
            offset += page_size;
        }
        chunks[ci] = ptr;
        alloc_count += 1;
        ci += 1;
    }
    let alloc_ns = sys::monotonic_ns() - start;
    log("  Allocated in ");
    log_u64(elapsed_ms(0, alloc_ns));
    logln("ms");

    logln("  Verifying memory contents...");
    let start = sys::monotonic_ns();
    let mut vi = 0;
    while vi < alloc_count {
        let pattern = (vi & 0xFF) as u8;
        let ptr = chunks[vi];
        let mut offset = 0;
        while offset < chunk_size {
            let val = unsafe { *ptr.add(offset) };
            assert_eq!(val, pattern);
            offset += page_size;
        }
        vi += 1;
    }
    let verify_ns = sys::monotonic_ns() - start;
    log("  Verified in ");
    log_u64(elapsed_ms(0, verify_ns));
    logln("ms");

    let mut fi = 0;
    while fi < alloc_count {
        sys::sys_munmap(chunks[fi], chunk_size);
        chunks[fi] = core::ptr::null_mut();
        fi += 1;
    }
    hardware::sys::runtime::resources::reset_counters();
    sys::sched_yield();
    log("  RAM stress done (");
    log_u64((alloc_count * chunk_size / (1024 * 1024)) as u64);
    if guardian_blocked {
        logln(" MB mapped + verified — guardian enforced 80% memory limit)");
    } else {
        logln(" MB mapped + verified)");
    }
    assert!(guardian_blocked, "guardian must block memory at 80%");

    logln("\n--- Phase 3: Disk I/O (512 MB write/read) ---");
    let disk_bytes: usize = 512 * 1024 * 1024;
    let io_chunk: usize = 4 * 1024 * 1024;
    let iterations = disk_bytes / io_chunk;
    let path = b"/tmp/hardware_stress_diskio.tmp\0";

    let write_buf = sys::sys_mmap_anon(io_chunk);
    assert!(!write_buf.is_null());
    unsafe {
        let mut bi = 0;
        while bi < io_chunk {
            *write_buf.add(bi) = 0xAB;
            bi += 1;
        }
    }

    log("  Writing ");
    log_u64((disk_bytes / (1024 * 1024)) as u64);
    logln(" MB...");
    let start = sys::monotonic_ns();
    let fd = sys::sys_open(path, sys::O_WRONLY | sys::o_creat() | sys::o_trunc(), 0o644);
    assert!(fd >= 0);
    let mut wi = 0;
    while wi < iterations {
        let buf = unsafe { core::slice::from_raw_parts(write_buf, io_chunk) };
        let written = sys::sys_write_fd(fd, buf);
        assert!(written > 0);
        wi += 1;
    }
    sys::sys_fsync(fd);
    sys::sys_close(fd);
    let write_ns = sys::monotonic_ns() - start;
    let write_ms = elapsed_ms(0, write_ns);
    log("  Write done in ");
    log_u64(write_ms);
    log("ms (");
    if write_ms > 0 {
        log_u64((disk_bytes as u64 / 1024 / 1024) * 1000 / write_ms);
    } else {
        log("?");
    }
    logln(" MB/s)");

    log("  Reading ");
    log_u64((disk_bytes / (1024 * 1024)) as u64);
    logln(" MB...");
    let read_buf = sys::sys_mmap_anon(io_chunk);
    assert!(!read_buf.is_null());
    let start = sys::monotonic_ns();
    let fd = sys::sys_open(path, sys::O_RDONLY, 0);
    assert!(fd >= 0);
    let mut total_read: usize = 0;
    loop {
        let buf = unsafe { core::slice::from_raw_parts_mut(read_buf, io_chunk) };
        let n = sys::sys_read_fd(fd, buf);
        if n <= 0 {
            break;
        }
        total_read += n as usize;
        let first = unsafe { *read_buf };
        assert_eq!(first, 0xAB);
    }
    sys::sys_close(fd);
    let read_ns = sys::monotonic_ns() - start;
    let read_ms = elapsed_ms(0, read_ns);
    assert_eq!(total_read, disk_bytes);
    log("  Read done in ");
    log_u64(read_ms);
    log("ms (");
    if read_ms > 0 {
        log_u64((disk_bytes as u64 / 1024 / 1024) * 1000 / read_ms);
    } else {
        log("?");
    }
    logln(" MB/s)");

    sys::sys_unlink(path);
    sys::sys_munmap(write_buf, io_chunk);
    sys::sys_munmap(read_buf, io_chunk);
    logln("  Disk I/O stress done");

    logln("\n--- Phase 4: CPU cache stress (L1/L2/L3) ---");
    let l3_bytes = (cpu.l3_cache_kb as usize) * 1024;
    let cache_target = ((l3_bytes / 100) * 70).max(1024 * 1024);
    let cache_u64s = cache_target / 8;
    log("  Thrashing ");
    log_u64((cache_target / (1024 * 1024)) as u64);
    log(" MB (70% of L3=");
    log_u64(cpu.l3_cache_kb as u64);
    logln("KB)...");

    let cache_ptr = sys::sys_mmap_anon(cache_target) as *mut u64;
    assert!(!cache_ptr.is_null());
    let start = sys::monotonic_ns();
    let passes = 10usize;
    let mut pass = 0;
    while pass < passes {
        let stride = 64 / 8;
        let mut idx = 0;
        while idx < cache_u64s {
            unsafe {
                let p = cache_ptr.add(idx);
                *p = (*p).wrapping_add(pass as u64);
            }
            idx += stride;
        }
        pass += 1;
    }
    core::hint::black_box(unsafe { *cache_ptr });
    let cache_ns = sys::monotonic_ns() - start;
    log("  Cache stress done in ");
    log_u64(elapsed_ms(0, cache_ns));
    log("ms (");
    if cache_ns > 0 {
        log_u64((cache_target * passes) as u64 * 1000 / (cache_ns / 1_000_000));
    }
    logln(" MB/s)");
    sys::sys_munmap(cache_ptr as *mut u8, cache_target);

    logln("\n--- Phase 5: Context switching (100% attempt — guardian should cap) ---");
    let yield_workers = total_cpus;
    let yield_duration_ns: u64 = 2_000_000_000;
    log("  Attempting ");
    log_u64(yield_workers as u64);
    logln(" workers yielding for 2s...");

    let start = sys::monotonic_ns();
    let yield_end = start + yield_duration_ns;

    let mut ypids = [0i64; 256];
    let mut ypid_count = 0usize;
    let mut guardian_yield_blocked = false;
    let mut yi = 0;
    while yi < yield_workers && ypid_count < 256 {
        let pid = sys::fork();
        if pid == 0 {
            while sys::monotonic_ns() < yield_end {
                sys::sched_yield();
            }
            sys::exit(0);
        } else if pid > 0 {
            ypids[ypid_count] = pid;
            ypid_count += 1;
        } else {
            guardian_yield_blocked = true;
            log("  Guardian: CPU limit reached after ");
            log_u64(ypid_count as u64);
            logln(" yield forks — blocked");
            break;
        }
        yi += 1;
    }

    let mut yj = 0;
    while yj < ypid_count {
        sys::waitpid(ypids[yj]);
        hardware::sys::runtime::resources::free_cpu(1);
        yj += 1;
    }
    let yield_ns = sys::monotonic_ns() - start;
    log("  Context switch: ");
    log_u64(ypid_count as u64);
    log("/");
    log_u64(yield_workers as u64);
    log(" workers in ");
    log_u64(elapsed_ms(0, yield_ns));
    if guardian_yield_blocked {
        logln("ms (guardian enforced CPU limit)");
    } else {
        logln("ms");
    }

    logln("\n--- Phase 6: Swap space stress (100% attempt — guardian should cap at 50%) ---");
    let fresh_mem = hardware::sys::detect_memory_info();
    let swap_total = if let Some(ref m) = fresh_mem {
        m.swap_total_bytes
    } else {
        0
    };
    if swap_total == 0 {
        logln("  Skipped (no swap configured)");
    } else {
        let fresh = fresh_mem.unwrap();
        let swap_free = fresh.swap_free_bytes;
        let avail_now = fresh.available_bytes as usize;
        let already_used = swap_total - swap_free;
        log("  Swap: ");
        log_u64(swap_total / (1024 * 1024));
        log(" MB total, ");
        log_u64(swap_free / (1024 * 1024));
        log(" MB free, ");
        log_u64(already_used / (1024 * 1024));
        logln(" MB used");
        log("  RAM available now: ");
        log_u64(avail_now as u64 / (1024 * 1024));
        logln(" MB");

        let target_swap = swap_total;
        log("  Target: push 100% = ");
        log_u64(target_swap / (1024 * 1024));
        logln(" MB into swap (guardian should block at 50%)...");

        let need = if target_swap > already_used {
            (target_swap - already_used) as usize
        } else {
            0
        };
        let hard_cap: usize = 8 * 1024 * 1024 * 1024;
        let alloc_target = (avail_now + need).min(hard_cap);

        let swap_chunk: usize = 16 * 1024 * 1024;
        let swap_n = (alloc_target / swap_chunk).clamp(1, 4096);

        log("  Allocating ");
        log_u64((swap_n * swap_chunk / (1024 * 1024)) as u64);
        logln(" MB...");

        let start = sys::monotonic_ns();
        let mut swap_ptrs = [core::ptr::null_mut::<u8>(); 4096];
        let mut swap_count = 0usize;
        let mut reached = false;
        let mut guardian_swap_blocked = false;
        let mut si = 0;
        while si < swap_n {
            if !hardware::sys::runtime::resources::try_alloc_swap(swap_chunk) {
                log("  Guardian: swap limit (50%) reached after ");
                log_u64((si * swap_chunk / (1024 * 1024)) as u64);
                logln(" MB — stopping allocation to prevent crash");
                guardian_swap_blocked = true;
                break;
            }
            let ptr = sys::sys_mmap_anon(swap_chunk);
            if ptr.is_null() {
                hardware::sys::runtime::resources::free_swap(swap_chunk);
                break;
            }
            let pattern = ((si * 37 + 13) & 0xFF) as u8;
            let mut off = 0;
            while off < swap_chunk {
                unsafe {
                    *ptr.add(off) = pattern;
                }
                off += page_size;
            }
            swap_ptrs[si] = ptr;
            swap_count += 1;

            if (si + 1) % 50 == 0 {
                if let Some(cur_mem) = hardware::sys::detect_memory_info() {
                    let used = cur_mem.swap_total_bytes - cur_mem.swap_free_bytes;
                    let pct = used * 100 / swap_total;
                    log("    ... ");
                    log_u64(((si + 1) * swap_chunk / (1024 * 1024)) as u64);
                    log(" MB allocated, swap: ");
                    log_u64(pct);
                    logln("%");
                    if pct >= 100 {
                        reached = true;
                        break;
                    }
                }
            }
            si += 1;
        }
        let swap_ns = sys::monotonic_ns() - start;
        log("  Allocated in ");
        log_u64(elapsed_ms(0, swap_ns));
        logln("ms");

        if let Some(final_mem) = hardware::sys::detect_memory_info() {
            let final_used = final_mem.swap_total_bytes - final_mem.swap_free_bytes;
            log("  Swap usage: ");
            log_u64(final_used / (1024 * 1024));
            log(" / ");
            log_u64(swap_total / (1024 * 1024));
            log(" MB (");
            if swap_total > 0 {
                log_u64(final_used * 100 / swap_total);
            }
            logln("%)");
        }

        logln("  Verifying...");
        let start = sys::monotonic_ns();
        let mut svi = 0;
        while svi < swap_count {
            let pattern = ((svi * 37 + 13) & 0xFF) as u8;
            let val = unsafe { *swap_ptrs[svi] };
            assert_eq!(val, pattern);
            svi += 1;
        }
        let sverify_ns = sys::monotonic_ns() - start;
        log("  Verified in ");
        log_u64(elapsed_ms(0, sverify_ns));
        logln("ms");

        let mut sfi = 0;
        while sfi < swap_count {
            sys::sys_munmap(swap_ptrs[sfi], swap_chunk);
            sfi += 1;
        }
        hardware::sys::runtime::resources::reset_counters();
        if guardian_swap_blocked {
            logln("  Swap stress done (guardian enforced 50% limit — crash prevented)");
        } else {
            log("  Swap stress done (reached target: ");
            log(if reached { "yes" } else { "no" });
            logln(")");
        }
        assert!(guardian_swap_blocked, "guardian must block swap at 50%");
    }

    logln("\n--- Phase 7: GPU stress (real hardware via DRM) ---");
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
        if !sys::has_hw_privilege() {
            logln("  PCI scan skipped (no iopl, need root)");
        } else {
            logln("  No GPU detected");
        }
    } else {
        log("  Found ");
        log_u64(gpu_count as u64);
        logln(" GPU(s)");

        let mut gi = 0;
        while gi < gpu_count {
            let g = &gpus[gi];
            log("  GPU ");
            log_u64(gi as u64);
            log(": vendor=0x");
            log_hex(g.vendor_id as u64);
            log(" device=0x");
            log_hex(g.device_id as u64);
            log(" BAR0=0x");
            log_hex(g.bar0 as u64);
            log("\n");
            gi += 1;
        }
    }

    logln("  Opening DRM device...");
    if let Some(mut drm) = hardware::sys::gpu::drm::open() {
        let driver_name = match drm.driver {
            hardware::sys::gpu::drm::DrmDriver::Radeon => "radeon",
            hardware::sys::gpu::drm::DrmDriver::Amdgpu => "amdgpu",
            hardware::sys::gpu::drm::DrmDriver::Nouveau => "nouveau",
            hardware::sys::gpu::drm::DrmDriver::I915 => "i915",
            hardware::sys::gpu::drm::DrmDriver::Unknown => "unknown",
        };
        log("  DRM driver: ");
        logln(driver_name);

        logln("  Querying GPU hardware info...");
        let info = drm.query_gpu_info();
        log("  Device ID: 0x");
        log_hex(info.device_id as u64);
        log("\n");
        log("  VRAM total: ");
        log_u64(info.vram_bytes / 1024 / 1024);
        logln(" MB");
        log("  VRAM used: ");
        log_u64(info.vram_used / 1024 / 1024);
        logln(" MB");
        log("  Shader engines: ");
        log_u64(info.shader_engines as u64);
        log(", SH/SE: ");
        log_u64(info.sh_per_se as u64);
        log(", Active CUs: ");
        log_u64(info.active_cu as u64);
        log("\n");
        log("  GPU sclk: ");
        log_u64(info.gpu_sclk_mhz as u64 / 100);
        log(" MHz, mclk: ");
        log_u64(info.gpu_mclk_mhz as u64 / 100);
        logln(" MHz");
        log("  GPU temp: ");
        log_u64(info.gpu_temp as u64 / 1000);
        logln(" C");
        log("  GB pipes: ");
        log_u64(info.gb_pipes as u64);
        log("\n");

        logln("  VRAM stress (GEM buffer alloc + write + verify)...");
        let vram_buf_size: u64 = 16 * 1024 * 1024;
        let vram_iters = 50usize;
        let start_vram = sys::monotonic_ns();
        let (written, verified) = drm.stress_vram(vram_buf_size, vram_iters);
        let vram_ns = sys::monotonic_ns() - start_vram;
        log("  VRAM: ");
        log_u64(written as u64);
        log(" written, ");
        log_u64(verified as u64);
        log(" verified in ");
        log_u64(elapsed_ms(0, vram_ns));
        logln("ms");
        if written > 0 {
            let mb_per_sec =
                (written as u64 * 4 * 1000) / (vram_ns / 1_000_000).max(1) / (1024 * 1024);
            log("  VRAM throughput: ~");
            log_u64(mb_per_sec);
            logln(" MB/s");
        }

        logln("  GPU command submission (NOP packets)...");
        logln("  Probing optimal CS packet size...");
        let probe = drm.probe_cs_packet_size();
        log("  CS probe: requires_vm=");
        log(if probe.requires_vm { "yes" } else { "no" });
        log(", max_nops=");
        log_u64(probe.max_nops as u64);
        log(", optimal_nops=");
        log_u64(probe.optimal_nops as u64);
        log(", overhead=");
        logln(if probe.overhead_detected { "yes" } else { "no" });

        let nop_count = if probe.optimal_nops > 0 {
            probe.optimal_nops
        } else {
            1024
        };
        let cs_passes = 10_000usize;
        let start_cs = sys::monotonic_ns();
        let mut cs_ok = 0usize;
        let mut cs_first_err: i64 = 0;
        let mut csi = 0;
        while csi < cs_passes {
            let ret = drm.submit_nop_packets(nop_count);
            if ret >= 0 {
                cs_ok += 1;
            } else if cs_first_err == 0 {
                cs_first_err = ret;
            }
            csi += 1;
        }
        if cs_first_err < 0 {
            log("  CS ioctl failed: errno=");
            log_u64((-cs_first_err) as u64);
            log("\n");
        }
        let cs_ns = sys::monotonic_ns() - start_cs;
        log("  CS submitted: ");
        log_u64(cs_ok as u64);
        log("/");
        log_u64(cs_passes as u64);
        log(" batches (");
        log_u64((cs_ok * nop_count) as u64);
        log(" NOPs) in ");
        log_u64(elapsed_ms(0, cs_ns));
        logln("ms");

        let info_after = drm.query_gpu_info();
        log("  Post-stress GPU temp: ");
        log_u64(info_after.gpu_temp as u64 / 1000);
        logln(" C");
        log("  Post-stress VRAM used: ");
        log_u64(info_after.vram_used / 1024 / 1024);
        logln(" MB");

        drm.close();
        logln("  GPU stress done (real hardware)");
    } else {
        logln("  DRM open failed (no /dev/dri/renderD128 access)");
    }

    logln("\n========== Guardian Enforcement Test Complete ==========\n");
}
