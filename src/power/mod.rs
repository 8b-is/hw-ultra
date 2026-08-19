pub mod core;
pub fn reboot() {
    core::reboot();
}

pub fn shutdown() {
    core::shutdown();
}

pub mod dvfs;
pub mod governor;
pub mod idle;
pub mod sleep;
pub mod thermal;
