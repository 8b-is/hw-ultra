use crate::gpu::hw::{GpuHwInfo, GpuMmio};

const CONFIG_MEMSIZE: usize = 0x5428;
const GRBM_STATUS: usize = 0x8010;
const GRBM_STATUS2: usize = 0x8008;
const GB_ADDR_CONFIG: usize = 0x98F8;
const MM_INDEX: usize = 0x0000;
const MM_DATA: usize = 0x0004;

const SCRATCH_REG: [usize; 8] = [
    0x8500, 0x8504, 0x8508, 0x850C, 0x8510, 0x8514, 0x8518, 0x851C,
];

const GRBM_GUI_ACTIVE: u32 = 1 << 31;
const GRBM_SE0_BUSY: u32 = 1 << 25;
const GRBM_SE1_BUSY: u32 = 1 << 26;

pub fn read_indirect(mmio: &GpuMmio, reg: u32) -> u32 {
    mmio.write32(MM_INDEX, reg);
    mmio.read32(MM_DATA)
}

pub fn detect_capabilities(mmio: &GpuMmio) -> GpuHwInfo {
    let memsize_raw = mmio.read32(CONFIG_MEMSIZE);
    let vram_bytes = memsize_raw as u64 * 1024 * 1024;

    let grbm = mmio.read32(GRBM_STATUS);
    let gb_addr = mmio.read32(GB_ADDR_CONFIG);

    let num_se = match (gb_addr >> 12) & 0x3 {
        0 => 1u32,
        1 => 2,
        2 => 4,
        _ => 1,
    };

    let num_sh_per_se = match (gb_addr >> 14) & 0x3 {
        0 => 1u32,
        1 => 2,
        _ => 1,
    };

    let cu_total = num_se * num_sh_per_se * 8;

    let se0_busy = (grbm & GRBM_SE0_BUSY) != 0;
    let se1_busy = (grbm & GRBM_SE1_BUSY) != 0;

    GpuHwInfo {
        vram_bytes,
        shader_engines: num_se,
        compute_units: cu_total,
        gpu_active: (grbm & GRBM_GUI_ACTIVE) != 0 || se0_busy || se1_busy,
        status_reg: grbm,
    }
}

pub fn stress_scratch_registers(mmio: &GpuMmio, iterations: usize) -> usize {
    let mut verified = 0usize;
    let mut iter = 0;
    while iter < iterations {
        let mut ri = 0;
        while ri < 8 {
            let pattern = (iter as u32)
                .wrapping_mul(0x9E3779B9)
                .wrapping_add(ri as u32);
            mmio.write32(SCRATCH_REG[ri], pattern);
            ri += 1;
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        ri = 0;
        while ri < 8 {
            let pattern = (iter as u32)
                .wrapping_mul(0x9E3779B9)
                .wrapping_add(ri as u32);
            let readback = mmio.read32(SCRATCH_REG[ri]);
            if readback == pattern {
                verified += 1;
            }
            ri += 1;
        }
        iter += 1;
    }
    verified
}

pub fn read_grbm_status(mmio: &GpuMmio) -> u32 {
    mmio.read32(GRBM_STATUS)
}

pub fn read_grbm_status2(mmio: &GpuMmio) -> u32 {
    mmio.read32(GRBM_STATUS2)
}
