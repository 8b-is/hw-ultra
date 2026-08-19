const REG_GCAP: usize = 0x000;
const REG_GCTL: usize = 0x008;
const REG_GSTS: usize = 0x00C;
const REG_INTCTL: usize = 0x020;
const REG_INTSTS: usize = 0x024;
const REG_CORB_BASE: usize = 0x040;
const REG_CORB_WP: usize = 0x048;
const REG_CORB_RP: usize = 0x04A;
const REG_CORB_CTL: usize = 0x04C;
const REG_RIRB_BASE: usize = 0x050;
const REG_RIRB_WP: usize = 0x058;
const REG_RIRB_CTL: usize = 0x05C;
const REG_VERSION: usize = 0x0FC;

const GCTL_RESET: u32 = 1 << 0;

pub fn reset(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_GCTL, 0);
        let mut timeout = 10000u32;
        while timeout > 0 {
            let status = super::super::mmio::mmio_read32(mmio_base + REG_GCTL);
            if status & GCTL_RESET == 0 {
                break;
            }
            timeout -= 1;
        }
        super::super::mmio::mmio_write32(mmio_base + REG_GCTL, GCTL_RESET);
        timeout = 10000;
        while timeout > 0 {
            let status = super::super::mmio::mmio_read32(mmio_base + REG_GCTL);
            if status & GCTL_RESET != 0 {
                break;
            }
            timeout -= 1;
        }
    }
}

pub fn read_caps(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_GCAP) }
}

pub fn enable_interrupts(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_INTCTL, 0x8000_00FF);
    }
}

pub fn read_int_status(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_INTSTS) }
}

pub fn setup_corb(mmio_base: usize, phys_addr: u64) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CORB_BASE, phys_addr as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_CORB_BASE + 4, (phys_addr >> 32) as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_CORB_CTL, 0x02);
    }
}

pub fn setup_rirb(mmio_base: usize, phys_addr: u64) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_RIRB_BASE, phys_addr as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_RIRB_BASE + 4, (phys_addr >> 32) as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_RIRB_CTL, 0x02);
    }
}

pub fn read_version(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_VERSION) }
}

pub fn read_gsts(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_GSTS) }
}

pub fn read_corb_wp(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_CORB_WP) }
}

pub fn read_corb_rp(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_CORB_RP) }
}

pub fn read_rirb_wp(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_RIRB_WP) }
}
