use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Cpu,
    Ram,
    Gpu,
    Tpu,
    Lpu,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Precision {
    Fp32,
    Fp16,
    Bf16,
    Int8,
    Int4,
}

const COMP_COUNT: usize = 5;

fn comp_idx(c: Component) -> usize {
    match c {
        Component::Cpu => 0,
        Component::Ram => 1,
        Component::Gpu => 2,
        Component::Tpu => 3,
        Component::Lpu => 4,
    }
}

static TEMP_MILLIDEG: [AtomicU32; COMP_COUNT] = [const { AtomicU32::new(0) }; COMP_COUNT];
static TEMP_LIMIT: [AtomicU32; COMP_COUNT] = [const { AtomicU32::new(105_000) }; COMP_COUNT];

static FREQ_HZ: [AtomicUsize; COMP_COUNT] = [const { AtomicUsize::new(0) }; COMP_COUNT];
static FREQ_MIN: [AtomicUsize; COMP_COUNT] = [const { AtomicUsize::new(0) }; COMP_COUNT];
static FREQ_MAX: [AtomicUsize; COMP_COUNT] = [const { AtomicUsize::new(usize::MAX) }; COMP_COUNT];

static CYCLES: [AtomicUsize; COMP_COUNT] = [const { AtomicUsize::new(0) }; COMP_COUNT];
static OPS_COUNT: [AtomicUsize; COMP_COUNT] = [const { AtomicUsize::new(0) }; COMP_COUNT];

static PRECISION: [AtomicU32; COMP_COUNT] = [const { AtomicU32::new(0) }; COMP_COUNT];

fn precision_to_u32(p: Precision) -> u32 {
    match p {
        Precision::Fp32 => 0,
        Precision::Fp16 => 1,
        Precision::Bf16 => 2,
        Precision::Int8 => 3,
        Precision::Int4 => 4,
    }
}

fn u32_to_precision(v: u32) -> Precision {
    match v {
        1 => Precision::Fp16,
        2 => Precision::Bf16,
        3 => Precision::Int8,
        4 => Precision::Int4,
        _ => Precision::Fp32,
    }
}

pub fn read_temperature(comp: Component) -> u32 {
    let idx = comp_idx(comp);
    let hw = match comp {
        Component::Cpu => read_cpu_temp(),
        Component::Gpu => read_accel_temp(comp),
        Component::Tpu => read_accel_temp(comp),
        Component::Lpu => read_accel_temp(comp),
        Component::Ram => 0,
    };
    if hw != 0 {
        TEMP_MILLIDEG[idx].store(hw, Ordering::Release);
        return hw;
    }
    TEMP_MILLIDEG[idx].load(Ordering::Acquire)
}

pub fn set_temperature(comp: Component, millideg: u32) {
    TEMP_MILLIDEG[comp_idx(comp)].store(millideg, Ordering::Release);
}

pub fn set_temp_limit(comp: Component, millideg: u32) {
    TEMP_LIMIT[comp_idx(comp)].store(millideg, Ordering::Release);
}

pub fn temp_limit(comp: Component) -> u32 {
    TEMP_LIMIT[comp_idx(comp)].load(Ordering::Acquire)
}

pub fn is_throttled(comp: Component) -> bool {
    let current = read_temperature(comp);
    let limit = temp_limit(comp);
    current > 0 && current >= limit
}

pub fn set_frequency(comp: Component, hz: usize) {
    let idx = comp_idx(comp);
    let min = FREQ_MIN[idx].load(Ordering::Acquire);
    let max = FREQ_MAX[idx].load(Ordering::Acquire);
    let clamped = if hz < min {
        min
    } else if hz > max {
        max
    } else {
        hz
    };
    FREQ_HZ[idx].store(clamped, Ordering::Release);
    match comp {
        Component::Cpu => crate::power::dvfs::set_frequency(clamped as u64),
        Component::Gpu => apply_accel_freq(comp, clamped),
        Component::Tpu => apply_accel_freq(comp, clamped),
        Component::Lpu => apply_accel_freq(comp, clamped),
        Component::Ram => {}
    }
}

pub fn frequency(comp: Component) -> usize {
    let idx = comp_idx(comp);
    let stored = FREQ_HZ[idx].load(Ordering::Acquire);
    if stored != 0 {
        return stored;
    }
    if let Component::Cpu = comp {
        return crate::power::dvfs::current_frequency() as usize;
    }
    0
}

pub fn set_freq_bounds(comp: Component, min_hz: usize, max_hz: usize) {
    let idx = comp_idx(comp);
    FREQ_MIN[idx].store(min_hz, Ordering::Release);
    FREQ_MAX[idx].store(max_hz, Ordering::Release);
    let current = FREQ_HZ[idx].load(Ordering::Acquire);
    if current != 0 {
        let clamped = if current < min_hz {
            min_hz
        } else if current > max_hz {
            max_hz
        } else {
            current
        };
        if clamped != current {
            set_frequency(comp, clamped);
        }
    }
}

pub fn freq_min(comp: Component) -> usize {
    FREQ_MIN[comp_idx(comp)].load(Ordering::Acquire)
}

pub fn freq_max(comp: Component) -> usize {
    FREQ_MAX[comp_idx(comp)].load(Ordering::Acquire)
}

pub fn record_cycles(comp: Component, count: usize) {
    CYCLES[comp_idx(comp)].fetch_add(count, Ordering::AcqRel);
}

pub fn read_cycles(comp: Component) -> usize {
    let idx = comp_idx(comp);
    if let Component::Cpu = comp {
        let hw = crate::debug::perf::read_timestamp();
        CYCLES[idx].store(hw as usize, Ordering::Release);
        return hw as usize;
    }
    CYCLES[idx].load(Ordering::Acquire)
}

pub fn reset_cycles(comp: Component) {
    CYCLES[comp_idx(comp)].store(0, Ordering::Release);
}

pub fn record_ops(comp: Component, count: usize) {
    OPS_COUNT[comp_idx(comp)].fetch_add(count, Ordering::AcqRel);
}

pub fn read_ops(comp: Component) -> usize {
    OPS_COUNT[comp_idx(comp)].load(Ordering::Acquire)
}

pub fn reset_ops(comp: Component) {
    OPS_COUNT[comp_idx(comp)].store(0, Ordering::Release);
}

pub fn set_precision(comp: Component, prec: Precision) {
    PRECISION[comp_idx(comp)].store(precision_to_u32(prec), Ordering::Release);
}

pub fn precision(comp: Component) -> Precision {
    u32_to_precision(PRECISION[comp_idx(comp)].load(Ordering::Acquire))
}

pub fn element_size(comp: Component) -> usize {
    match precision(comp) {
        Precision::Fp32 => 4,
        Precision::Fp16 => 2,
        Precision::Bf16 => 2,
        Precision::Int8 => 1,
        Precision::Int4 => 1,
    }
}

pub struct ComponentSnapshot {
    pub component: Component,
    pub temp_millideg: u32,
    pub temp_limit: u32,
    pub throttled: bool,
    pub freq_hz: usize,
    pub freq_min: usize,
    pub freq_max: usize,
    pub cycles: usize,
    pub ops: usize,
    pub precision: Precision,
}

pub fn snapshot(comp: Component) -> ComponentSnapshot {
    ComponentSnapshot {
        component: comp,
        temp_millideg: read_temperature(comp),
        temp_limit: temp_limit(comp),
        throttled: is_throttled(comp),
        freq_hz: frequency(comp),
        freq_min: freq_min(comp),
        freq_max: freq_max(comp),
        cycles: read_cycles(comp),
        ops: read_ops(comp),
        precision: precision(comp),
    }
}

pub fn snapshot_all() -> [ComponentSnapshot; COMP_COUNT] {
    [
        snapshot(Component::Cpu),
        snapshot(Component::Ram),
        snapshot(Component::Gpu),
        snapshot(Component::Tpu),
        snapshot(Component::Lpu),
    ]
}

pub fn reset_all() {
    for i in 0..COMP_COUNT {
        TEMP_MILLIDEG[i].store(0, Ordering::Release);
        TEMP_LIMIT[i].store(105_000, Ordering::Release);
        FREQ_HZ[i].store(0, Ordering::Release);
        FREQ_MIN[i].store(0, Ordering::Release);
        FREQ_MAX[i].store(usize::MAX, Ordering::Release);
        CYCLES[i].store(0, Ordering::Release);
        OPS_COUNT[i].store(0, Ordering::Release);
        PRECISION[i].store(0, Ordering::Release);
    }
}

fn read_cpu_temp() -> u32 {
    if let Some(zone_temp) = crate::thermal::api::read_thermal_zone(0) {
        return zone_temp;
    }
    0
}

fn read_accel_temp(comp: Component) -> u32 {
    let reg_offset: usize = 0x08;
    let arch = crate::arch::detect_arch();
    let raw = match (comp, arch) {
        (Component::Gpu, crate::arch::Architecture::X86_64) => {
            crate::arch::x86_64::gpu::read_gpu_reg(reg_offset)
        }
        (Component::Gpu, crate::arch::Architecture::AArch64) => {
            crate::arch::aarch64::gpu::read_gpu_reg(reg_offset)
        }
        (Component::Tpu, crate::arch::Architecture::X86_64) => {
            crate::arch::x86_64::tpu::read_tpu_reg(reg_offset)
        }
        (Component::Tpu, crate::arch::Architecture::AArch64) => {
            crate::arch::aarch64::tpu::read_tpu_reg(reg_offset)
        }
        (Component::Lpu, crate::arch::Architecture::X86_64) => {
            crate::arch::x86_64::lpu::read_lpu_reg(reg_offset)
        }
        (Component::Lpu, crate::arch::Architecture::AArch64) => {
            crate::arch::aarch64::lpu::read_lpu_reg(reg_offset)
        }
        _ => 0,
    };
    if raw > 0 && raw < 200_000 {
        return raw;
    }
    0
}

fn apply_accel_freq(comp: Component, hz: usize) {
    let reg_offset: usize = 0x0C;
    let val = (hz / 1_000_000) as u32;
    let arch = crate::arch::detect_arch();
    match (comp, arch) {
        (Component::Gpu, crate::arch::Architecture::X86_64) => {
            crate::arch::x86_64::gpu::write_gpu_reg(reg_offset, val)
        }
        (Component::Gpu, crate::arch::Architecture::AArch64) => {
            crate::arch::aarch64::gpu::write_gpu_reg(reg_offset, val)
        }
        (Component::Tpu, crate::arch::Architecture::X86_64) => {
            crate::arch::x86_64::tpu::write_tpu_reg(reg_offset, val)
        }
        (Component::Tpu, crate::arch::Architecture::AArch64) => {
            crate::arch::aarch64::tpu::write_tpu_reg(reg_offset, val)
        }
        (Component::Lpu, crate::arch::Architecture::X86_64) => {
            crate::arch::x86_64::lpu::write_lpu_reg(reg_offset, val)
        }
        (Component::Lpu, crate::arch::Architecture::AArch64) => {
            crate::arch::aarch64::lpu::write_lpu_reg(reg_offset, val)
        }
        _ => {}
    }
}
