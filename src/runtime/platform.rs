use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Platform {
    Unknown = 0,
    BareMetal = 1,
    Hosted = 2,
}

static PLATFORM: AtomicU8 = AtomicU8::new(0);

pub fn detect() -> Platform {
    let cached = PLATFORM.load(Ordering::Acquire);
    if cached != 0 {
        return from_u8(cached);
    }

    let uefi = crate::firmware::uefi::is_present();
    let smbios = crate::firmware::smbios::is_present();

    let result = if uefi || smbios || crate::boot::total_usable_ram() > 0 {
        Platform::BareMetal
    } else {
        Platform::Hosted
    };

    PLATFORM.store(result as u8, Ordering::Release);
    result
}

pub fn set(p: Platform) {
    PLATFORM.store(p as u8, Ordering::Release);
}

pub fn current() -> Platform {
    from_u8(PLATFORM.load(Ordering::Acquire))
}

fn from_u8(v: u8) -> Platform {
    match v {
        1 => Platform::BareMetal,
        2 => Platform::Hosted,
        _ => Platform::Unknown,
    }
}
