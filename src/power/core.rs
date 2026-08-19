use crate::arch::x86_64::io::{outb, outl};

pub fn reboot() {
    unsafe {
        outb(0x64, 0xFE);
    }
}

pub fn shutdown() {
    unsafe {
        outl(0x604, 0x2000);
        outb(0x64, 0xFE);
    }
    loop {
        core::hint::spin_loop();
    }
}
