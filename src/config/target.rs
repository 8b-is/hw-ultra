use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TargetPlatform {
    Generic,
    Server,
    Embedded,
    Desktop,
}

static PLATFORM: AtomicU8 = AtomicU8::new(0);

pub fn set_platform(p: TargetPlatform) {
    let val = match p {
        TargetPlatform::Generic => 0,
        TargetPlatform::Server => 1,
        TargetPlatform::Embedded => 2,
        TargetPlatform::Desktop => 3,
    };
    PLATFORM.store(val, Ordering::Release);
}

pub fn get_platform() -> TargetPlatform {
    match PLATFORM.load(Ordering::Acquire) {
        1 => TargetPlatform::Server,
        2 => TargetPlatform::Embedded,
        3 => TargetPlatform::Desktop,
        _ => TargetPlatform::Generic,
    }
}
