#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ExceptionLevel {
    EL0,
    EL1,
    EL2,
    EL3,
}

impl ExceptionLevel {
    pub fn current() -> Self {
        let el = crate::arch::aarch64::cpu::system_regs::read_current_el();
        match el {
            0 => ExceptionLevel::EL0,
            1 => ExceptionLevel::EL1,
            2 => ExceptionLevel::EL2,
            3 => ExceptionLevel::EL3,
            other => {
                static EL_SIG: core::sync::atomic::AtomicUsize =
                    core::sync::atomic::AtomicUsize::new(0);
                EL_SIG.store(other as usize, core::sync::atomic::Ordering::Release);
                ExceptionLevel::EL0
            }
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            ExceptionLevel::EL0 => 0,
            ExceptionLevel::EL1 => 1,
            ExceptionLevel::EL2 => 2,
            ExceptionLevel::EL3 => 3,
        }
    }
}
