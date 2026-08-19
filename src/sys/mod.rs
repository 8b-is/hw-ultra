mod consts;
mod file;
mod io;
mod mem_ops;
mod process;
mod raw;
mod socket;
mod time;

pub(crate) use raw::raw_syscall;

pub use consts::{o_creat, o_directory, o_excl, o_nonblock, o_trunc};
pub use consts::{AF_UNIX, F_SETFL, O_RDONLY, O_RDWR, O_WRONLY, SOCK_STREAM};
pub use file::{
    read_file_bytes, scan_dir, sys_close, sys_fsync, sys_getcwd, sys_getdents64, sys_ioctl,
    sys_mkdir, sys_open, sys_read_fd, sys_stat, sys_unlink, sys_write_fd, trim_trailing, DirEntry,
};
pub use io::{
    fmt_u64, parse_decimal_u64, parse_hex_u16_pub, write_stderr, write_stderr_str, write_stderr_u64,
};
pub use mem_ops::{sys_mmap_anon, sys_mmap_device, sys_mmap_shared_anon, sys_munmap};
pub use process::{
    exit, fork, has_hw_privilege, request_hw_privilege, sys_execve, sys_kill, sys_rt_sigaction,
    waitpid,
};
pub use socket::{sys_accept, sys_bind, sys_connect, sys_fcntl, sys_listen, sys_socket};
pub use time::{monotonic_ns, sched_yield, sleep_ns, sleep_secs};

pub use crate::arch::shim::{
    set_mkdir_fn, set_os_constants, set_scan_dir_fn, set_syscall_nrs, OsConstants, SyscallNrTable,
};
pub use crate::arch::{cpuid_count, detect_arch, set_arch, Architecture};
pub use crate::init::{init, init_shims};

pub use crate::memory::info::{
    detect_memory_info, set_detect_memory_fn, DetectMemoryFn, MemoryInfo,
};

pub use crate::runtime::{
    AccelHandle, AcceleratorKind, BiosInfo, BootHal, BuddyAllocator, BumpAllocator, BusHal,
    CacheHierarchy, Component, ComponentSnapshot, DeviceType, DiscoveredDevice, DiscoveryHal,
    DmaBuffer, DmaDescriptor, DmaHal, DtDeviceEntry, Enclave, FadtInfo, FdtHeader, FdtNode,
    FirmwareHal, Frame, GopInfo, HardwareAccessHal, HpetInfo, InterruptHal, IommuDomain, IommuHal,
    IrqVector, IsolationDomain, IsolationLevel, McfgEntry, MemModuleInfo, MemoryHal, NumaNode,
    Platform, Precision, ReapResult, ResourceSnapshot, RuntimeServicesTable, SecurityHal, Slab,
    SmbiosCpuInfo, SmbiosHeader, SmbiosInfo, SpeculationMitigation, System, UefiInfo,
    UefiMemoryDescriptor, UefiMemoryType, VirtAddr,
};

pub use crate::arch::guardian::{guardian_snapshot, GuardianSnapshot, SurgeSnapshot};

pub mod arch {
    pub use crate::arch::*;
    pub mod aarch64 {
        pub use crate::arch::aarch64::*;
    }
    pub mod guardian {
        pub use crate::arch::guardian::*;
    }
    pub mod shim {
        pub use crate::arch::shim::*;
        pub mod privilege {
            pub use crate::arch::shim::privilege::*;
        }
    }
    pub mod x86_64 {
        pub use crate::arch::x86_64::*;
    }
    pub mod architecture {
        pub use crate::arch::architecture::*;
    }
}

pub mod boot {
    pub use crate::boot::*;
    pub mod memmap {
        pub use crate::boot::memmap::*;
    }
}

pub mod bus {
    pub mod amba {
        pub use crate::bus::amba::*;
    }
    pub mod discovery {
        pub use crate::bus::discovery::*;
    }
    pub mod pci {
        pub use crate::bus::pci::*;
    }
    pub mod pcie {
        pub use crate::bus::pcie::*;
    }
    pub mod virtio {
        pub use crate::bus::virtio::*;
    }
}

pub mod common {
    pub mod alignment {
        pub use crate::common::alignment::*;
    }
    pub mod atomic {
        pub use crate::common::atomic::*;
    }
    pub mod barrier {
        pub use crate::common::barrier::*;
    }
    pub mod bitfield {
        pub use crate::common::bitfield::*;
    }
    pub mod endian {
        pub use crate::common::endian::*;
    }
    pub mod error {
        pub use crate::common::error::*;
    }
    pub mod guard {
        pub use crate::common::guard::*;
    }
    pub mod once {
        pub use crate::common::once::*;
    }
    pub mod registers {
        pub use crate::common::registers::*;
    }
    pub mod volatile {
        pub use crate::common::volatile::*;
    }
}

pub mod config {
    pub mod capability {
        pub use crate::config::capability::*;
    }
    pub mod feature {
        pub use crate::config::feature::*;
    }
    pub mod target {
        pub use crate::config::target::*;
    }
}

pub mod cpu {
    pub use crate::cpu::*;
    pub mod affinity {
        pub use crate::cpu::affinity::*;
    }
    pub mod context {
        pub use crate::cpu::context::*;
    }
    pub mod core {
        pub use crate::cpu::core::*;
    }
    pub mod cores {
        pub use crate::cpu::cores::*;
    }
    pub mod detect {
        pub use crate::cpu::detect::*;
    }
    pub mod features {
        pub use crate::cpu::features::*;
    }
    pub mod frequency {
        pub use crate::cpu::frequency::*;
    }
    pub mod info {
        pub use crate::cpu::info::*;
    }
    pub mod interrupt {
        pub use crate::cpu::interrupt::*;
    }
    pub mod power {
        pub use crate::cpu::power::*;
    }
    pub mod ram {
        pub use crate::cpu::ram::*;
    }
    pub mod scheduler {
        pub use crate::cpu::scheduler::*;
    }
    pub mod speculation {
        pub use crate::cpu::speculation::*;
    }
    pub mod thermal {
        pub use crate::cpu::thermal::*;
    }
    pub mod topology {
        pub use crate::cpu::topology::*;
    }
    pub mod vector {
        pub use crate::cpu::vector::*;
    }
    pub mod arch_aarch64 {
        pub use crate::cpu::arch_aarch64::*;
    }
    pub mod arch_x86_64 {
        pub use crate::cpu::arch_x86_64::*;
    }
    pub mod api {
        pub use crate::cpu::api::*;
    }
}

pub mod debug {
    pub mod counters {
        pub use crate::debug::counters::*;
    }
    pub mod perf {
        pub use crate::debug::perf::*;
    }
    pub mod trace {
        pub use crate::debug::trace::*;
    }
}

pub mod discovery {
    pub mod registry {
        pub use crate::discovery::registry::*;
    }
}

pub mod dma {
    pub mod buffer {
        pub use crate::dma::buffer::*;
    }
    pub mod descriptor {
        pub use crate::dma::descriptor::*;
    }
    pub mod engine {
        pub use crate::dma::engine::*;
    }
    pub mod mapping {
        pub use crate::dma::mapping::*;
    }
}

pub mod firmware {
    pub mod acpi {
        pub use crate::firmware::acpi::*;
    }
    pub mod devicetree {
        pub use crate::firmware::devicetree::*;
    }
    pub mod smbios {
        pub use crate::firmware::smbios::*;
    }
    pub mod uefi {
        pub use crate::firmware::uefi::*;
    }
}

pub mod gpu {
    pub use crate::gpu::*;
    pub mod command {
        pub use crate::gpu::command::*;
    }
    pub mod compute {
        pub use crate::gpu::compute::*;
    }
    pub mod device {
        pub use crate::gpu::device::*;
    }
    pub mod drivers {
        pub use crate::gpu::drivers::*;
    }
    pub mod drm {
        pub use crate::gpu::drm::*;
    }
    pub mod hw {
        pub use crate::gpu::hw::*;
    }
    pub mod memory {
        pub use crate::gpu::memory::*;
    }
    pub mod pipeline {
        pub use crate::gpu::pipeline::*;
    }
    pub mod queue {
        pub use crate::gpu::queue::*;
    }
    pub mod scheduler {
        pub use crate::gpu::scheduler::*;
    }
    pub mod shader {
        pub use crate::gpu::shader::*;
    }
}

pub mod hardware_access {
    pub use crate::hardware_access::*;
    pub mod api {
        pub use crate::hardware_access::api::*;
    }
}

pub mod init_mod {
    pub use crate::init::*;
    pub mod detect_test {
        pub use crate::init::detect_test::*;
    }
}

pub mod interrupt {
    pub use crate::interrupt::*;
    pub mod idt {
        pub use crate::interrupt::idt::*;
    }
}

pub mod iommu {
    pub use crate::iommu::*;
    pub mod controller {
        pub use crate::iommu::controller::*;
    }
    pub mod domain {
        pub use crate::iommu::domain::*;
    }
    pub mod mapping {
        pub use crate::iommu::mapping::*;
    }
}

pub mod lpu {
    pub use crate::lpu::*;
    pub mod device {
        pub use crate::lpu::device::*;
    }
    pub mod drivers {
        pub use crate::lpu::drivers::*;
    }
    pub mod inference {
        pub use crate::lpu::inference::*;
    }
    pub mod memory {
        pub use crate::lpu::memory::*;
    }
    pub mod pipeline {
        pub use crate::lpu::pipeline::*;
    }
    pub mod quantization {
        pub use crate::lpu::quantization::*;
    }
    pub mod scheduler {
        pub use crate::lpu::scheduler::*;
    }
    pub mod lifecycle {
        pub use crate::lpu::lifecycle::*;
    }
}

pub mod memory {
    pub mod cache {
        pub use crate::memory::cache::*;
    }
    pub mod heap {
        pub use crate::memory::heap::*;
    }
    pub mod info {
        pub use crate::memory::info::*;
    }
    pub mod numa {
        pub use crate::memory::numa::*;
    }
    pub mod phys {
        pub use crate::memory::phys::*;
    }
    pub mod virt {
        pub use crate::memory::virt::*;
    }
}

pub mod net {
    pub mod ethernet {
        pub use crate::net::ethernet::*;
    }
    pub mod ipv4 {
        pub use crate::net::ipv4::*;
    }
    pub mod tcp {
        pub use crate::net::tcp::*;
    }
}

pub mod power {
    pub use crate::power::*;
    pub mod core {
        pub use crate::power::core::*;
    }
    pub mod dvfs {
        pub use crate::power::dvfs::*;
    }
    pub mod governor {
        pub use crate::power::governor::*;
    }
    pub mod idle {
        pub use crate::power::idle::*;
    }
    pub mod sleep {
        pub use crate::power::sleep::*;
    }
    pub mod thermal {
        pub use crate::power::thermal::*;
    }
}

pub mod runtime {
    pub use crate::runtime::*;
    pub mod entry {
        pub use crate::runtime::entry::*;
    }
    pub mod hal {
        pub use crate::runtime::hal::*;
    }
    pub mod handle {
        pub use crate::runtime::handle::*;
    }
    pub mod monitor {
        pub use crate::runtime::monitor::*;
    }
    pub mod panic {
        pub use crate::runtime::panic::*;
    }
    pub mod platform {
        pub use crate::runtime::platform::*;
    }
    pub mod resources {
        pub use crate::runtime::resources::*;
    }
    pub mod system {
        pub use crate::runtime::system::*;
    }
    pub mod worker {
        pub use crate::runtime::worker::*;
    }
}

pub mod security {
    pub mod enclaves {
        pub use crate::security::enclaves::*;
    }
    pub mod isolation {
        pub use crate::security::isolation::*;
    }
    pub mod speculation {
        pub use crate::security::speculation::*;
    }
}

pub mod syscall {
    pub mod api {
        pub use crate::syscall::api::*;
    }
}

pub mod thermal {
    pub mod api {
        pub use crate::thermal::api::*;
    }
}

pub mod timer {
    pub mod arm_generic {
        pub use crate::timer::arm_generic::*;
    }
    pub mod clockevent {
        pub use crate::timer::clockevent::*;
    }
    pub mod clocksource {
        pub use crate::timer::clocksource::*;
    }
    pub mod hpet {
        pub use crate::timer::hpet::*;
    }
    pub mod pit {
        pub use crate::timer::pit::*;
    }
}

pub mod topology {
    pub use crate::topology::*;
    pub mod interconnect {
        pub use crate::topology::interconnect::*;
    }
    pub mod node {
        pub use crate::topology::node::*;
    }
    pub mod system {
        pub use crate::topology::system::*;
    }
}

pub mod tpu {
    pub use crate::tpu::*;
    pub mod compiler {
        pub use crate::tpu::compiler::*;
    }
    pub mod device {
        pub use crate::tpu::device::*;
    }
    pub mod dma {
        pub use crate::tpu::dma::*;
    }
    pub mod drivers {
        pub use crate::tpu::drivers::*;
    }
    pub mod executor {
        pub use crate::tpu::executor::*;
    }
    pub mod graph {
        pub use crate::tpu::graph::*;
    }
    pub mod memory {
        pub use crate::tpu::memory::*;
    }
    pub mod runtime {
        pub use crate::tpu::runtime::*;
    }
    pub mod tensor {
        pub use crate::tpu::tensor::*;
    }
    pub mod lifecycle {
        pub use crate::tpu::lifecycle::*;
    }
}

pub mod audio {
    pub use crate::audio::*;
    pub mod codec {
        pub use crate::audio::codec::*;
    }
    pub mod device {
        pub use crate::audio::device::*;
    }
    pub mod drivers {
        pub use crate::audio::drivers::*;
    }
    pub mod format {
        pub use crate::audio::format::*;
    }
    pub mod hw {
        pub use crate::audio::hw::*;
    }
    pub mod mixer {
        pub use crate::audio::mixer::*;
    }
    pub mod stream {
        pub use crate::audio::stream::*;
    }
}

pub mod camera {
    pub use crate::camera::*;
    pub mod device {
        pub use crate::camera::device::*;
    }
    pub mod drivers {
        pub use crate::camera::drivers::*;
    }
    pub mod frame {
        pub use crate::camera::frame::*;
    }
    pub mod hw {
        pub use crate::camera::hw::*;
    }
    pub mod pipeline {
        pub use crate::camera::pipeline::*;
    }
    pub mod sensor {
        pub use crate::camera::sensor::*;
    }
}

pub mod display {
    pub use crate::display::*;
    pub mod backlight {
        pub use crate::display::backlight::*;
    }
    pub mod device {
        pub use crate::display::device::*;
    }
    pub mod drivers {
        pub use crate::display::drivers::*;
    }
    pub mod framebuffer {
        pub use crate::display::framebuffer::*;
    }
    pub mod hw {
        pub use crate::display::hw::*;
    }
    pub mod timing {
        pub use crate::display::timing::*;
    }
}

pub mod input {
    pub use crate::input::*;
    pub mod button {
        pub use crate::input::button::*;
    }
    pub mod device {
        pub use crate::input::device::*;
    }
    pub mod drivers {
        pub use crate::input::drivers::*;
    }
    pub mod event {
        pub use crate::input::event::*;
    }
    pub mod hw {
        pub use crate::input::hw::*;
    }
    pub mod keyboard {
        pub use crate::input::keyboard::*;
    }
    pub mod touchscreen {
        pub use crate::input::touchscreen::*;
    }
}

pub mod modem {
    pub use crate::modem::*;
    pub mod device {
        pub use crate::modem::device::*;
    }
    pub mod drivers {
        pub use crate::modem::drivers::*;
    }
    pub mod hw {
        pub use crate::modem::hw::*;
    }
    pub mod network {
        pub use crate::modem::network::*;
    }
    pub mod protocol {
        pub use crate::modem::protocol::*;
    }
    pub mod signal {
        pub use crate::modem::signal::*;
    }
    pub mod sim {
        pub use crate::modem::sim::*;
    }
}

pub mod nfc {
    pub use crate::nfc::*;
    pub mod device {
        pub use crate::nfc::device::*;
    }
    pub mod drivers {
        pub use crate::nfc::drivers::*;
    }
    pub mod hw {
        pub use crate::nfc::hw::*;
    }
    pub mod polling {
        pub use crate::nfc::polling::*;
    }
    pub mod protocol {
        pub use crate::nfc::protocol::*;
    }
    pub mod tag {
        pub use crate::nfc::tag::*;
    }
}

pub mod sensor {
    pub use crate::sensor::*;
    pub mod calibration {
        pub use crate::sensor::calibration::*;
    }
    pub mod data {
        pub use crate::sensor::data::*;
    }
    pub mod device {
        pub use crate::sensor::device::*;
    }
    pub mod drivers {
        pub use crate::sensor::drivers::*;
    }
    pub mod hw {
        pub use crate::sensor::hw::*;
    }
    pub mod sampling {
        pub use crate::sensor::sampling::*;
    }
}

pub mod storage {
    pub use crate::storage::*;
    pub mod command {
        pub use crate::storage::command::*;
    }
    pub mod device {
        pub use crate::storage::device::*;
    }
    pub mod drivers {
        pub use crate::storage::drivers::*;
    }
    pub mod hw {
        pub use crate::storage::hw::*;
    }
    pub mod partition {
        pub use crate::storage::partition::*;
    }
    pub mod queue {
        pub use crate::storage::queue::*;
    }
}

pub mod usb {
    pub use crate::usb::*;
    pub mod descriptor {
        pub use crate::usb::descriptor::*;
    }
    pub mod device {
        pub use crate::usb::device::*;
    }
    pub mod drivers {
        pub use crate::usb::drivers::*;
    }
    pub mod endpoint {
        pub use crate::usb::endpoint::*;
    }
    pub mod hub {
        pub use crate::usb::hub::*;
    }
    pub mod hw {
        pub use crate::usb::hw::*;
    }
    pub mod transfer {
        pub use crate::usb::transfer::*;
    }
}
