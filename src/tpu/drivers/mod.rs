use core::sync::atomic::{AtomicUsize, Ordering};

pub fn init_all() -> bool {
    let res = crate::tpu::lifecycle::init(0x1000);
    debug_assert!(res.is_ok());

    if let Some(mut drv) = generic::GenericTpu::probe() {
        drv.init();
        static TPU_DRV_SIG: AtomicUsize = AtomicUsize::new(0);
        TPU_DRV_SIG.store(1, Ordering::Release);
    }

    true
}

pub mod generic;
