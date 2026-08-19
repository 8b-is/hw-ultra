const REG_CTRL: usize = 0x000;
const REG_STATUS: usize = 0x004;
const REG_TOKEN_IN: usize = 0x100;
const REG_TOKEN_OUT: usize = 0x104;
const REG_HIDDEN_SIZE: usize = 0x108;
const REG_BATCH_SIZE: usize = 0x10C;
const REG_QUANT_MODE: usize = 0x110;
const REG_INFER_CMD: usize = 0x120;
const REG_INTR_STATUS: usize = 0x020;
const REG_INTR_ENABLE: usize = 0x024;
const REG_VERSION: usize = 0x0FC;

const CTRL_RESET: u32 = 1 << 0;
const CTRL_ENABLE: u32 = 1 << 1;
const CTRL_HALT: u32 = 1 << 31;

const STATUS_READY: u32 = 1 << 0;
const STATUS_INFERRING: u32 = 1 << 1;
const STATUS_ERROR: u32 = 1 << 2;

const CMD_PREFILL: u32 = 0x01;
const CMD_DECODE: u32 = 0x02;
const CMD_SPECULATIVE: u32 = 0x03;

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
    status & STATUS_READY != 0 && status & STATUS_INFERRING == 0
}

pub fn is_error(mmio_base: usize) -> bool {
    let status = unsafe { super::super::mmio::mmio_read32(mmio_base + REG_STATUS) };
    status & STATUS_ERROR != 0
}

pub fn configure_inference(mmio_base: usize, hidden_size: u32, batch_size: u32, quant_mode: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_HIDDEN_SIZE, hidden_size);
        super::super::mmio::mmio_write32(mmio_base + REG_BATCH_SIZE, batch_size);
        super::super::mmio::mmio_write32(mmio_base + REG_QUANT_MODE, quant_mode);
    }
}

pub fn submit_prefill(mmio_base: usize, token_addr: u32, count: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_TOKEN_IN, token_addr);
        super::super::mmio::mmio_write32(mmio_base + REG_TOKEN_OUT, count);
        super::super::mmio::mmio_write32(mmio_base + REG_INFER_CMD, CMD_PREFILL);
    }
}

pub fn submit_decode(mmio_base: usize, token_addr: u32, count: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_TOKEN_IN, token_addr);
        super::super::mmio::mmio_write32(mmio_base + REG_TOKEN_OUT, count);
        super::super::mmio::mmio_write32(mmio_base + REG_INFER_CMD, CMD_DECODE);
    }
}

pub fn submit_speculative(mmio_base: usize, token_addr: u32, count: u32) {
    unsafe {
        super::super::mmio::mmio_write32(mmio_base + REG_TOKEN_IN, token_addr);
        super::super::mmio::mmio_write32(mmio_base + REG_TOKEN_OUT, count);
        super::super::mmio::mmio_write32(mmio_base + REG_INFER_CMD, CMD_SPECULATIVE);
    }
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
