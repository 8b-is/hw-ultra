const REG_CAP: usize = 0x000;
const REG_VS: usize = 0x008;
const REG_CC: usize = 0x014;
const REG_CSTS: usize = 0x01C;
const REG_AQA: usize = 0x024;
const REG_ASQ: usize = 0x028;
const REG_ACQ: usize = 0x030;
const REG_SQ0TDBL: usize = 0x1000;
const REG_CQ0HDBL: usize = 0x1004;
const REG_VERSION: usize = 0x0FC;

const CC_ENABLE: u32 = 1 << 0;
const CSTS_READY: u32 = 1 << 0;

pub fn reset(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CC, 0);
        let mut timeout = 10000u32;
        while timeout > 0 {
            let status = super::super::mmio::mmio_read32(mmio_base + REG_CSTS);
            if status & CSTS_READY == 0 {
                break;
            }
            timeout -= 1;
        }
    }
}

pub fn enable(mmio_base: usize) {
    unsafe {
        let cc = super::super::mmio::mmio_read32(mmio_base + REG_CC);
        super::super::mmio::mmio_write32(mmio_base + REG_CC, cc | CC_ENABLE);
        let mut timeout = 10000u32;
        while timeout > 0 {
            let status = super::super::mmio::mmio_read32(mmio_base + REG_CSTS);
            if status & CSTS_READY != 0 {
                break;
            }
            timeout -= 1;
        }
    }
}

pub fn read_caps(mmio_base: usize) -> u64 {
    unsafe {
        let lo = super::super::mmio::mmio_read32(mmio_base + REG_CAP) as u64;
        let hi = super::super::mmio::mmio_read32(mmio_base + REG_CAP + 4) as u64;
        lo | (hi << 32)
    }
}

pub fn setup_admin_queues(mmio_base: usize, sq_phys: u64, cq_phys: u64, depth: u16) {
    let aqa = ((depth as u32 - 1) << 16) | (depth as u32 - 1);
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_AQA, aqa);
        super::super::mmio::mmio_write32(mmio_base + REG_ASQ, sq_phys as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_ASQ + 4, (sq_phys >> 32) as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_ACQ, cq_phys as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_ACQ + 4, (cq_phys >> 32) as u32);
    }
}

pub fn ring_doorbell(mmio_base: usize, queue: u32, tail: u32) {
    let offset = REG_SQ0TDBL + (queue as usize) * 8;
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + offset, tail);
    }
}

pub fn read_version(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_VERSION) }
}

pub fn read_vs(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_VS) }
}

pub fn read_cq_head(mmio_base: usize, queue: u32) -> u32 {
    let offset = REG_CQ0HDBL + (queue as usize) * 8;
    unsafe { super::super::mmio::mmio_read32(mmio_base + offset) }
}
