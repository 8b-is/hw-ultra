#![no_std]

mod arch;
mod audio;
mod boot;
mod bus;
mod camera;
mod common;
mod config;
mod cpu;
mod debug;
mod discovery;
mod display;
mod dma;
mod firmware;
mod gpu;
pub mod hardware_access;
mod init;
mod input;
mod interrupt;
mod iommu;
mod lpu;
pub mod memory;
mod modem;
mod net;
mod nfc;
mod power;
mod runtime;
mod security;
mod sensor;
mod storage;
mod syscall;
mod thermal;
mod timer;
mod topology;
mod tpu;
mod usb;

pub mod sys;
