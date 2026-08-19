use crate::common::once::OnceCopy;
use crate::sys;

#[derive(Copy, Clone)]
pub struct DrmConstants {
    pub ioctl_version: u64,
    pub ioctl_gem_close: u64,
    pub ioctl_radeon_info: u64,
    pub ioctl_radeon_gem_info: u64,
    pub ioctl_radeon_gem_create: u64,
    pub ioctl_radeon_gem_mmap: u64,
    pub ioctl_radeon_gem_wait_idle: u64,
    pub ioctl_radeon_cs: u64,
    pub radeon_info_device_id: u32,
    pub radeon_info_num_gb_pipes: u32,
    pub radeon_info_vram_usage: u32,
    pub radeon_info_active_cu_count: u32,
    pub radeon_info_current_gpu_sclk: u32,
    pub radeon_info_current_gpu_mclk: u32,
    pub radeon_info_current_gpu_temp: u32,
    pub radeon_info_max_se: u32,
    pub radeon_info_max_sh_per_se: u32,
    pub radeon_gem_domain_vram: u32,
    pub radeon_gem_domain_gtt: u32,
    pub radeon_chunk_id_relocs: u32,
    pub radeon_chunk_id_ib: u32,
    pub radeon_chunk_id_flags: u32,
    pub radeon_cs_ring_gfx: u32,
    pub radeon_cs_use_vm: u32,
}

static DRM_CONSTS: OnceCopy<DrmConstants> = OnceCopy::new();

pub fn set_drm_constants(c: DrmConstants) {
    DRM_CONSTS.set(c);
}

fn drm() -> Option<DrmConstants> {
    DRM_CONSTS.get()
}

pub struct CsProbeResult {
    pub optimal_nops: usize,
    pub max_nops: usize,
    pub requires_vm: bool,
    pub overhead_detected: bool,
}

pub struct DrmDevice {
    pub fd: i64,
    pub driver: DrmDriver,
    pub cs_flags: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DrmDriver {
    Unknown,
    Radeon,
    Amdgpu,
    Nouveau,
    I915,
}

pub struct GpuInfo {
    pub device_id: u32,
    pub vram_bytes: u64,
    pub vram_used: u64,
    pub shader_engines: u32,
    pub sh_per_se: u32,
    pub active_cu: u32,
    pub gpu_sclk_mhz: u32,
    pub gpu_mclk_mhz: u32,
    pub gpu_temp: u32,
    pub gb_pipes: u32,
}

pub struct GemBuffer {
    pub handle: u32,
    pub size: u64,
    pub mapped: *mut u8,
    pub map_size: usize,
    pub domain: u32,
}

#[repr(C)]
struct DrmVersion {
    version_major: i32,
    version_minor: i32,
    version_patchlevel: i32,
    pad: i32,
    name_len: u64,
    name: *mut u8,
    date_len: u64,
    date: *mut u8,
    desc_len: u64,
    desc: *mut u8,
}

#[repr(C)]
struct RadeonInfo {
    request: u32,
    pad: u32,
    value: u64,
}

#[repr(C)]
struct RadeonGemInfo {
    gart_size: u64,
    vram_size: u64,
    vram_visible: u64,
}

#[repr(C)]
struct RadeonGemCreate {
    size: u64,
    alignment: u64,
    handle: u32,
    initial_domain: u32,
    flags: u32,
    _pad: u32,
}

#[repr(C)]
struct RadeonGemMmap {
    handle: u32,
    pad: u32,
    offset: u64,
    size: u64,
    addr_ptr: u64,
}

#[repr(C)]
struct GemClose {
    handle: u32,
    pad: u32,
}

#[repr(C)]
struct RadeonGemWaitIdle {
    handle: u32,
    flags: u32,
}

#[repr(C)]
struct RadeonCs {
    num_chunks: u32,
    cs_id: u32,
    chunks: u64,
    gart_limit: u64,
    vram_limit: u64,
}

#[repr(C)]
struct RadeonCsChunk {
    chunk_id: u32,
    length_dw: u32,
    chunk_data: u64,
}

pub type OpenDrmFn = fn() -> i64;

static OPEN_DRM_FN: OnceCopy<OpenDrmFn> = OnceCopy::new();

pub fn set_open_drm_fn(f: OpenDrmFn) {
    OPEN_DRM_FN.set(f);
}

fn try_open_drm() -> i64 {
    if let Some(f) = OPEN_DRM_FN.get() {
        return f();
    }
    -1
}

pub fn open() -> Option<DrmDevice> {
    let c = drm()?;
    let fd = try_open_drm();
    if fd < 0 {
        return None;
    }

    let mut name_buf = [0u8; 32];
    let mut ver = DrmVersion {
        version_major: 0,
        version_minor: 0,
        version_patchlevel: 0,
        pad: 0,
        name_len: 31,
        name: name_buf.as_mut_ptr(),
        date_len: 0,
        date: core::ptr::null_mut(),
        desc_len: 0,
        desc: core::ptr::null_mut(),
    };

    let ret = sys::sys_ioctl(fd, c.ioctl_version, &mut ver as *mut _ as u64);
    if ret < 0 {
        sys::sys_close(fd);
        return None;
    }

    let driver = if ver.name_len >= 6 && name_buf[..6] == *b"radeon" {
        DrmDriver::Radeon
    } else if ver.name_len >= 6 && name_buf[..6] == *b"amdgpu" {
        DrmDriver::Amdgpu
    } else if ver.name_len >= 7 && name_buf[..7] == *b"nouveau" {
        DrmDriver::Nouveau
    } else if ver.name_len >= 4 && name_buf[..4] == *b"i915" {
        DrmDriver::I915
    } else {
        DrmDriver::Unknown
    };

    Some(DrmDevice {
        fd,
        driver,
        cs_flags: 0,
    })
}

fn radeon_query_u32(fd: i64, request: u32) -> Option<u32> {
    let c = drm()?;
    let mut value: u32 = 0;
    let mut info = RadeonInfo {
        request,
        pad: 0,
        value: &mut value as *mut u32 as u64,
    };
    let ret = sys::sys_ioctl(fd, c.ioctl_radeon_info, &mut info as *mut _ as u64);
    if ret < 0 {
        None
    } else {
        Some(value)
    }
}

fn radeon_query_u64(fd: i64, request: u32) -> Option<u64> {
    let c = drm()?;
    let mut value: u64 = 0;
    let mut info = RadeonInfo {
        request,
        pad: 0,
        value: &mut value as *mut u64 as u64,
    };
    let ret = sys::sys_ioctl(fd, c.ioctl_radeon_info, &mut info as *mut _ as u64);
    if ret < 0 {
        None
    } else {
        Some(value)
    }
}

impl DrmDevice {
    pub fn query_gem_info(&self) -> (u64, u64, u64) {
        let c = match drm() {
            Some(c) => c,
            None => return (0, 0, 0),
        };
        let mut info = RadeonGemInfo {
            gart_size: 0,
            vram_size: 0,
            vram_visible: 0,
        };
        let ret = sys::sys_ioctl(self.fd, c.ioctl_radeon_gem_info, &mut info as *mut _ as u64);
        if ret < 0 {
            (0, 0, 0)
        } else {
            (info.vram_size, info.vram_visible, info.gart_size)
        }
    }

    pub fn query_gpu_info(&self) -> GpuInfo {
        let c = match drm() {
            Some(c) => c,
            None => {
                return GpuInfo {
                    device_id: 0,
                    vram_bytes: 0,
                    vram_used: 0,
                    shader_engines: 0,
                    sh_per_se: 0,
                    active_cu: 0,
                    gpu_sclk_mhz: 0,
                    gpu_mclk_mhz: 0,
                    gpu_temp: 0,
                    gb_pipes: 0,
                }
            }
        };
        let device_id = radeon_query_u32(self.fd, c.radeon_info_device_id).unwrap_or(0);
        let (vram_bytes, _, _) = self.query_gem_info();
        let vram_used = radeon_query_u64(self.fd, c.radeon_info_vram_usage).unwrap_or(0);
        let shader_engines = radeon_query_u32(self.fd, c.radeon_info_max_se).unwrap_or(0);
        let sh_per_se = radeon_query_u32(self.fd, c.radeon_info_max_sh_per_se).unwrap_or(0);
        let active_cu = radeon_query_u32(self.fd, c.radeon_info_active_cu_count).unwrap_or(0);
        let gpu_sclk_mhz = radeon_query_u32(self.fd, c.radeon_info_current_gpu_sclk).unwrap_or(0);
        let gpu_mclk_mhz = radeon_query_u32(self.fd, c.radeon_info_current_gpu_mclk).unwrap_or(0);
        let gpu_temp = radeon_query_u32(self.fd, c.radeon_info_current_gpu_temp).unwrap_or(0);
        let gb_pipes = radeon_query_u32(self.fd, c.radeon_info_num_gb_pipes).unwrap_or(0);
        GpuInfo {
            device_id,
            vram_bytes,
            vram_used,
            shader_engines,
            sh_per_se,
            active_cu,
            gpu_sclk_mhz,
            gpu_mclk_mhz,
            gpu_temp,
            gb_pipes,
        }
    }

    pub fn gem_create(&self, size: u64, domain: u32) -> Option<GemBuffer> {
        let c = drm()?;
        let mut req = RadeonGemCreate {
            size,
            alignment: 4096,
            handle: 0,
            initial_domain: domain,
            flags: 0,
            _pad: 0,
        };
        let ret = sys::sys_ioctl(
            self.fd,
            c.ioctl_radeon_gem_create,
            &mut req as *mut _ as u64,
        );
        if ret < 0 || req.handle == 0 {
            return None;
        }
        Some(GemBuffer {
            handle: req.handle,
            size: req.size,
            mapped: core::ptr::null_mut(),
            map_size: 0,
            domain,
        })
    }

    pub fn gem_create_vram(&self, size: u64) -> Option<GemBuffer> {
        let c = drm()?;
        self.gem_create(size, c.radeon_gem_domain_vram)
    }

    pub fn gem_create_gtt(&self, size: u64) -> Option<GemBuffer> {
        let c = drm()?;
        self.gem_create(size, c.radeon_gem_domain_gtt)
    }

    pub fn gem_mmap(&self, buf: &mut GemBuffer) -> bool {
        let c = match drm() {
            Some(c) => c,
            None => return false,
        };
        let mut req = RadeonGemMmap {
            handle: buf.handle,
            pad: 0,
            offset: 0,
            size: buf.size,
            addr_ptr: 0,
        };
        let ret = sys::sys_ioctl(self.fd, c.ioctl_radeon_gem_mmap, &mut req as *mut _ as u64);
        if ret < 0 {
            return false;
        }

        let mmap_ret = unsafe {
            sys::raw_syscall(
                crate::arch::shim::nr_mmap(),
                0,
                buf.size,
                3,
                1,
                self.fd as u64,
                req.addr_ptr,
            )
        };
        if mmap_ret < 0 {
            return false;
        }

        buf.mapped = mmap_ret as *mut u8;
        buf.map_size = buf.size as usize;
        true
    }

    pub fn gem_wait_idle(&self, buf: &GemBuffer) {
        let c = match drm() {
            Some(c) => c,
            None => return,
        };
        let mut req = RadeonGemWaitIdle {
            handle: buf.handle,
            flags: 0,
        };
        sys::sys_ioctl(
            self.fd,
            c.ioctl_radeon_gem_wait_idle,
            &mut req as *mut _ as u64,
        );
    }

    pub fn gem_close(&self, buf: &mut GemBuffer) {
        if !buf.mapped.is_null() {
            sys::sys_munmap(buf.mapped, buf.map_size);
            buf.mapped = core::ptr::null_mut();
            buf.map_size = 0;
        }
        if buf.handle != 0 {
            let mut req = GemClose {
                handle: buf.handle,
                pad: 0,
            };
            if let Some(c) = drm() {
                sys::sys_ioctl(self.fd, c.ioctl_gem_close, &mut req as *mut _ as u64);
            }
            buf.handle = 0;
        }
    }

    pub fn submit_ib(&self, ib_data: *const u32, dwords: u32) -> i64 {
        self.submit_ib_flags(ib_data, dwords, self.cs_flags)
    }

    pub fn submit_ib_flags(&self, ib_data: *const u32, dwords: u32, cs_flags: u32) -> i64 {
        let c = match drm() {
            Some(c) => c,
            None => return -1,
        };
        let flags: [u32; 2] = [cs_flags, 0];

        let chunks = [
            RadeonCsChunk {
                chunk_id: c.radeon_chunk_id_relocs,
                length_dw: 0,
                chunk_data: 0,
            },
            RadeonCsChunk {
                chunk_id: c.radeon_chunk_id_ib,
                length_dw: dwords,
                chunk_data: ib_data as u64,
            },
            RadeonCsChunk {
                chunk_id: c.radeon_chunk_id_flags,
                length_dw: 2,
                chunk_data: flags.as_ptr() as u64,
            },
        ];

        let chunk_ptrs: [u64; 3] = [
            &chunks[0] as *const RadeonCsChunk as u64,
            &chunks[1] as *const RadeonCsChunk as u64,
            &chunks[2] as *const RadeonCsChunk as u64,
        ];

        let mut cs = RadeonCs {
            num_chunks: 3,
            cs_id: 0,
            chunks: chunk_ptrs.as_ptr() as u64,
            gart_limit: 0,
            vram_limit: 0,
        };

        sys::sys_ioctl(self.fd, c.ioctl_radeon_cs, &mut cs as *mut _ as u64)
    }

    fn try_nop_with_flags(&self, count: usize, cs_flags: u32) -> i64 {
        let dwords = count * 2;
        let buf_size = dwords * 4;
        let ptr = sys::sys_mmap_anon(buf_size);
        if ptr.is_null() {
            return -12;
        }
        let dw_ptr = ptr as *mut u32;
        let mut i = 0;
        while i < count {
            let header = (3u32 << 30) | (0x10 << 8);
            unsafe {
                core::ptr::write_volatile(dw_ptr.add(i * 2), header);
                core::ptr::write_volatile(dw_ptr.add(i * 2 + 1), 0);
            }
            i += 1;
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        let ret = self.submit_ib_flags(dw_ptr, dwords as u32, cs_flags);
        sys::sys_munmap(ptr, buf_size);
        ret
    }

    pub fn probe_cs_packet_size(&mut self) -> CsProbeResult {
        let c = match drm() {
            Some(c) => c,
            None => {
                return CsProbeResult {
                    optimal_nops: 0,
                    max_nops: 0,
                    requires_vm: false,
                    overhead_detected: false,
                }
            }
        };
        let mut requires_vm = false;
        if self.try_nop_with_flags(1, c.radeon_cs_ring_gfx) < 0 {
            if self.try_nop_with_flags(1, c.radeon_cs_ring_gfx | c.radeon_cs_use_vm) >= 0 {
                requires_vm = true;
            } else {
                return CsProbeResult {
                    optimal_nops: 0,
                    max_nops: 0,
                    requires_vm: false,
                    overhead_detected: false,
                };
            }
        }
        let active_flags = if requires_vm {
            c.radeon_cs_ring_gfx | c.radeon_cs_use_vm
        } else {
            c.radeon_cs_ring_gfx
        };
        self.cs_flags = active_flags;

        let mut size: usize = 8192;
        let mut last_good: usize = 0;
        while size >= 1 {
            if self.try_nop_with_flags(size, active_flags) >= 0 {
                last_good = size;
                break;
            }
            size /= 2;
        }
        if last_good == 0 {
            return CsProbeResult {
                optimal_nops: 0,
                max_nops: 0,
                requires_vm,
                overhead_detected: false,
            };
        }

        let mut lo = last_good;
        let mut hi = if last_good < 8192 {
            last_good * 2
        } else {
            8192
        };
        while lo + 1 < hi {
            let mid = lo + (hi - lo) / 2;
            if self.try_nop_with_flags(mid, active_flags) >= 0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let max_nops = lo;

        let mut overhead_detected = false;
        let optimal_nops = if max_nops > 4 {
            let t0 = sys::monotonic_ns();
            self.try_nop_with_flags(max_nops, active_flags);
            let t_full = sys::monotonic_ns() - t0;

            let safe = max_nops - 2;
            let t1 = sys::monotonic_ns();
            self.try_nop_with_flags(safe, active_flags);
            let t_safe = sys::monotonic_ns() - t1;

            if t_full > t_safe * 2 && t_safe > 0 {
                overhead_detected = true;
                safe
            } else {
                max_nops
            }
        } else {
            max_nops
        };

        CsProbeResult {
            optimal_nops,
            max_nops,
            requires_vm,
            overhead_detected,
        }
    }

    pub fn close(&mut self) {
        if self.fd >= 0 {
            sys::sys_close(self.fd);
            self.fd = -1;
        }
    }

    pub fn stress_vram(&self, size: u64, iterations: usize) -> (usize, usize) {
        let mut buf = match self.gem_create_vram(size) {
            Some(b) => b,
            None => return (0, 0),
        };
        if !self.gem_mmap(&mut buf) {
            self.gem_close(&mut buf);
            return (0, 0);
        }

        let words = buf.map_size / 4;
        let ptr = buf.mapped as *mut u32;
        let mut total_written = 0usize;
        let mut total_verified = 0usize;

        let mut iter = 0;
        while iter < iterations {
            let mut wi = 0;
            while wi < words {
                let pattern = (iter as u32)
                    .wrapping_mul(0x9E3779B9)
                    .wrapping_add(wi as u32);
                unsafe {
                    core::ptr::write_volatile(ptr.add(wi), pattern);
                }
                wi += 1;
                total_written += 1;
            }

            core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);

            let mut ri = 0;
            while ri < words {
                let pattern = (iter as u32)
                    .wrapping_mul(0x9E3779B9)
                    .wrapping_add(ri as u32);
                let readback = unsafe { core::ptr::read_volatile(ptr.add(ri)) };
                if readback == pattern {
                    total_verified += 1;
                }
                ri += 1;
            }
            iter += 1;
        }

        self.gem_wait_idle(&buf);
        self.gem_close(&mut buf);
        (total_written, total_verified)
    }

    pub fn submit_nop_packets(&self, count: usize) -> i64 {
        self.try_nop_with_flags(count, self.cs_flags)
    }
}
