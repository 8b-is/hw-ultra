use crate::gpu::GpuDevice;

pub struct Device {
    pub info: GpuDevice,
    pub mmio_base: Option<usize>,
}

impl Device {
    pub fn new(info: GpuDevice) -> Self {
        let mmio_base = if info.bar0 != 0 {
            Some(info.bar0 as usize)
        } else {
            None
        };
        Device { info, mmio_base }
    }

    pub fn read_mmio32(&self, off: usize) -> Option<u32> {
        if let Some(base) = self.mmio_base {
            crate::hardware_access::mmio_read32(base + off)
        } else {
            None
        }
    }

    pub fn write_mmio32(&self, off: usize, val: u32) -> bool {
        if let Some(base) = self.mmio_base {
            crate::hardware_access::mmio_write32(base + off, val)
        } else {
            false
        }
    }

    pub fn bar0_base(&self) -> Option<usize> {
        self.mmio_base
    }

    /// Read GPU engine clock (SCLK) from PLL MMIO registers.
    /// CG_SPLL_FUNC_CNTL (0x600): ref_div [5:0], post_div [24:20]
    /// CG_SPLL_FUNC_CNTL_3 (0x608): fb_div [24:0]
    /// SCLK = ref_clk_mhz * fb_div / (ref_div * post_div)
    pub fn gpu_clock_mhz(&self) -> u64 {
        const CG_SPLL_FUNC_CNTL: usize = 0x600;
        const CG_SPLL_FUNC_CNTL_3: usize = 0x608;
        const REF_CLK_MHZ: u64 = 100;
        let cntl = match self.read_mmio32(CG_SPLL_FUNC_CNTL) {
            Some(v) => v,
            None => return 0,
        };
        let cntl3 = match self.read_mmio32(CG_SPLL_FUNC_CNTL_3) {
            Some(v) => v,
            None => return 0,
        };
        let ref_div = (cntl & 0x3F) as u64;
        let post_div = ((cntl >> 20) & 0x1F) as u64;
        let fb_div = (cntl3 & 0x01FF_FFFF) as u64;
        if ref_div == 0 || post_div == 0 {
            return 0;
        }
        REF_CLK_MHZ * fb_div / (ref_div * post_div)
    }

    /// Read GPU temperature from CG_MULT_THERMAL_STATUS (0x714).
    /// Bits [24:16] contain temperature in degrees Celsius.
    /// Returns millidegrees.
    pub fn gpu_temp_millideg(&self) -> u32 {
        const CG_MULT_THERMAL_STATUS: usize = 0x714;
        let raw = match self.read_mmio32(CG_MULT_THERMAL_STATUS) {
            Some(v) => v,
            None => return 0,
        };
        let temp_c = (raw >> 16) & 0x1FF;
        temp_c * 1000
    }

    /// Read current GPU power profile index from
    /// TARGET_AND_CURRENT_PROFILE_INDEX (0x798), bits [6:4].
    pub fn power_state(&self) -> u64 {
        const TARGET_AND_CURRENT_PROFILE_INDEX: usize = 0x798;
        let raw = match self.read_mmio32(TARGET_AND_CURRENT_PROFILE_INDEX) {
            Some(v) => v,
            None => return 0,
        };
        ((raw >> 4) & 0x7) as u64
    }
}
