#[derive(Copy, Clone)]
pub struct Command {
    pub opcode: u32,
    pub data_ptr: *const u8,
    pub data_len: usize,
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}

impl Command {
    pub fn new(op: u32, data: *const u8, len: usize) -> Self {
        Command {
            opcode: op,
            data_ptr: data,
            data_len: len,
        }
    }

    pub fn nop() -> Self {
        Command {
            opcode: 0,
            data_ptr: core::ptr::null(),
            data_len: 0,
        }
    }

    pub fn is_nop(&self) -> bool {
        self.opcode == 0
    }

    pub fn submit(&self, sched: &crate::gpu::scheduler::GpuScheduler) -> bool {
        sched.submit(*self)
    }
}
