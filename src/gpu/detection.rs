use crate::common::once::OnceCopy;

#[derive(Copy, Clone)]
pub struct GpuDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub bar0: u32,
}

/// Raw GPU identification data returned by the detection callback.
/// The callback probes hardware (device nodes, MMIO, etc.) and returns
/// the raw register value. The library parses it.
#[derive(Copy, Clone)]
pub struct RawGpuId {
    /// PCI-style vendor hint: 0x13B5 = ARM Mali, 0x5143 = Qualcomm Adreno,
    /// 0x1010 = Imagination PowerVR, etc.
    pub vendor: u16,
    /// Raw GPU identification register value.
    /// For Mali: GPU_ID register (offset 0x0000) — product in [31:16],
    ///           major version in [15:8], minor in [7:4], status in [3:0].
    /// For Adreno: RBBM_CHIP_ID register.
    /// If the raw register is unavailable, set to 0.
    pub raw_id: u32,
}

pub type DetectGpuFn = fn() -> Option<RawGpuId>;

static DETECT_GPU_FN: OnceCopy<DetectGpuFn> = OnceCopy::new();

pub fn set_detect_gpu_fn(f: DetectGpuFn) {
    DETECT_GPU_FN.set(f);
}

pub fn detect_gpus_impl(out: &mut [GpuDevice]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let found = crate::bus::pci::scan_video_devices(out);
    if found > 0 {
        return found;
    }
    if let crate::arch::Architecture::X86_64 = crate::arch::detect_arch() {
        if crate::arch::shim::privilege::has_hw_privilege() {
            let status = unsafe { crate::arch::x86_64::io::inb(0x3DA) };
            if status != 0 && status != 0xFF {
                out[0] = GpuDevice {
                    bus: 0,
                    device: 0,
                    function: 0,
                    vendor_id: 0xFFFF,
                    device_id: 0xFFFF,
                    class: 0x03,
                    subclass: 0x00,
                    prog_if: 0x00,
                    bar0: 0,
                };
                return 1;
            }
        }
    }
    detect_gpu_callback(out)
}

fn detect_gpu_callback(out: &mut [GpuDevice]) -> usize {
    let raw = if let Some(f) = DETECT_GPU_FN.get() {
        match f() {
            Some(r) => r,
            None => match probe_gpu_devices() {
                Some(r) => r,
                None => return 0,
            },
        }
    } else {
        match probe_gpu_devices() {
            Some(r) => r,
            None => return 0,
        }
    };
    let device_id = match raw.vendor {
        0x13B5 => parse_mali_gpu_id(raw.raw_id),
        0x5143 => parse_adreno_chip_id(raw.raw_id),
        _ => {
            if raw.raw_id != 0 {
                raw.raw_id as u16
            } else {
                0x0001
            }
        }
    };
    out[0] = GpuDevice {
        bus: 0,
        device: 0,
        function: 0,
        vendor_id: raw.vendor,
        device_id,
        class: 0x03,
        subclass: 0x00,
        prog_if: 0x00,
        bar0: 0,
    };
    1
}

/// Parse Mali GPU_ID register value → device_id (product model number).
/// GPU_ID layout: [31:16] product_id, [15:8] major, [7:4] minor, [3:0] status.
/// Returns the product_id directly, or 0x0001 if the register is zero.
pub fn parse_mali_gpu_id(raw: u32) -> u16 {
    if raw == 0 || raw == 0xFFFFFFFF {
        return 0x0001;
    }
    let product = ((raw >> 16) & 0xFFFF) as u16;
    if product != 0 {
        product
    } else {
        0x0001
    }
}

/// Returns (major, minor) version from a Mali GPU_ID register.
pub fn mali_gpu_version(raw: u32) -> (u8, u8) {
    let major = ((raw >> 8) & 0xFF) as u8;
    let minor = ((raw >> 4) & 0xF) as u8;
    (major, minor)
}

/// Map Mali product_id to a human-readable name.
pub fn mali_product_name(product_id: u16) -> &'static str {
    match product_id {
        // Midgard
        0x0620 => "Mali-T620",
        0x0720 => "Mali-T720",
        0x0750 => "Mali-T760",
        0x0820 => "Mali-T820",
        0x0830 => "Mali-T830",
        0x0860 => "Mali-T860",
        0x0880 => "Mali-T880",
        // Bifrost
        0x6000 => "Mali-G71",
        0x6001 => "Mali-G72",
        0x7000 => "Mali-G51",
        0x7001 => "Mali-G76",
        0x7002 => "Mali-G52",
        0x7003 => "Mali-G31",
        // Valhall
        0x9000 => "Mali-G77",
        0x9001 => "Mali-G57",
        0x9002 => "Mali-G78",
        0x9003 => "Mali-G68",
        0x9004 => "Mali-G78AE",
        0xA002 => "Mali-G710",
        0xA003 => "Mali-G510",
        0xA004 => "Mali-G310",
        0xA007 => "Mali-G610",
        // 5th Gen
        0xB002 => "Mali-G715",
        0xB003 => "Mali-G615",
        0xC002 => "Mali-G720",
        0xC003 => "Mali-G620",
        _ => "Mali (unknown)",
    }
}

/// Parse Adreno RBBM_CHIP_ID register → device_id.
pub fn parse_adreno_chip_id(raw: u32) -> u16 {
    if raw == 0 || raw == 0xFFFFFFFF {
        return 0x0001;
    }
    // Adreno chip ID encodes the GPU model: e.g. 0x06010900 → Adreno 619
    // Extract the core identity from high bits
    (raw >> 16) as u16
}

/// Map Adreno chip_id (upper 16 bits of RBBM_CHIP_ID) to name.
pub fn adreno_product_name(chip: u16) -> &'static str {
    match chip {
        0x0601 => "Adreno 619",
        0x0608 => "Adreno 618",
        0x0612 => "Adreno 612",
        0x0616 => "Adreno 616",
        0x0630 => "Adreno 630",
        0x0640 => "Adreno 640",
        0x0650 => "Adreno 650",
        0x0660 => "Adreno 660",
        0x0680 => "Adreno 680",
        0x0690 => "Adreno 690",
        0x0730 => "Adreno 730",
        0x0740 => "Adreno 740",
        0x0750 => "Adreno 750",
        _ => "Adreno (unknown)",
    }
}

fn probe_gpu_devices() -> Option<RawGpuId> {
    None
}
