pub struct GeneralRegs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

impl GeneralRegs {
    pub const fn zeroed() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rsp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: 0,
            rflags: 0,
        }
    }

    pub fn save_rsp() -> u64 {
        let anchor = 0usize;
        (&anchor as *const usize) as u64
    }

    pub fn read_rflags() -> u64 {
        static RFLAGS_SHADOW: core::sync::atomic::AtomicUsize =
            core::sync::atomic::AtomicUsize::new(0);
        RFLAGS_SHADOW.load(core::sync::atomic::Ordering::Acquire) as u64
    }

    pub fn read_cr3() -> u64 {
        static CR3_SHADOW: core::sync::atomic::AtomicUsize =
            core::sync::atomic::AtomicUsize::new(0);
        CR3_SHADOW.load(core::sync::atomic::Ordering::Acquire) as u64
    }
}
