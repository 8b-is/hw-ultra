const REG_CTRL: usize = 0x000;
const REG_STATUS: usize = 0x004;
const REG_COMPUTE_CMD: usize = 0x010;
const REG_COMPUTE_ADDR: usize = 0x014;
const REG_COMPUTE_SIZE: usize = 0x018;
const REG_COMPUTE_FLAGS: usize = 0x01C;
const REG_INTR_STATUS: usize = 0x020;
const REG_INTR_ENABLE: usize = 0x024;
const REG_VERSION: usize = 0x0FC;

const CTRL_RESET: u32 = 1 << 0;
const CTRL_ENABLE: u32 = 1 << 1;
const CTRL_HALT: u32 = 1 << 31;

const STATUS_READY: u32 = 1 << 0;
const STATUS_BUSY: u32 = 1 << 1;
const STATUS_ERROR: u32 = 1 << 2;

const CMD_MATMUL: u32 = 0x01;
const CMD_CONV2D: u32 = 0x02;
const CMD_ACTIVATION: u32 = 0x03;

pub fn reset(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CTRL, CTRL_RESET);
        let mut timeout = 10000u32;
        while timeout > 0 {
            let status = super::super::mmio::mmio_read32(mmio_base + REG_STATUS);
            if status & STATUS_READY != 0 {
                break;
            }
            timeout -= 1;
        }
        super::super::mmio::mmio_write32(mmio_base + REG_CTRL, CTRL_ENABLE);
    }
}

pub fn is_ready(mmio_base: usize) -> bool {
    let status = unsafe { super::super::mmio::mmio_read32(mmio_base + REG_STATUS) };
    status & STATUS_READY != 0 && status & STATUS_BUSY == 0
}

pub fn is_error(mmio_base: usize) -> bool {
    let status = unsafe { super::super::mmio::mmio_read32(mmio_base + REG_STATUS) };
    status & STATUS_ERROR != 0
}

pub fn submit_compute(mmio_base: usize, cmd: u32, data_phys: u64, size: u32, flags: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_COMPUTE_ADDR, data_phys as u32);
        super::super::mmio::mmio_write32(mmio_base + REG_COMPUTE_SIZE, size);
        super::super::mmio::mmio_write32(mmio_base + REG_COMPUTE_FLAGS, flags);
        super::super::mmio::mmio_write32(mmio_base + REG_COMPUTE_CMD, cmd);
    }
}

pub fn submit_matmul(mmio_base: usize, data_phys: u64, size: u32) {
    submit_compute(mmio_base, CMD_MATMUL, data_phys, size, 0);
}

pub fn submit_conv2d(mmio_base: usize, data_phys: u64, size: u32) {
    submit_compute(mmio_base, CMD_CONV2D, data_phys, size, 0);
}

pub fn submit_activation(mmio_base: usize, data_phys: u64, size: u32) {
    submit_compute(mmio_base, CMD_ACTIVATION, data_phys, size, 0);
}

pub fn enable_interrupts(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_INTR_ENABLE, 0x07);
    }
}

pub fn clear_interrupts(mmio_base: usize) -> u32 {
    let status = unsafe { super::super::mmio::mmio_read32(mmio_base + REG_INTR_STATUS) };
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_INTR_STATUS, status);
    }
    status
}

pub fn read_version(mmio_base: usize) -> u32 {
    unsafe { super::super::mmio::mmio_read32(mmio_base + REG_VERSION) }
}

pub fn halt(mmio_base: usize) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_CTRL, CTRL_HALT);
    }
}
