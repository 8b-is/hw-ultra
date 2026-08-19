pub use crate::discovery::registry::{DeviceType, DiscoveredDevice};
pub use crate::dma::buffer::DmaBuffer;
pub use crate::dma::engine::Descriptor as DmaDescriptor;
pub use crate::firmware::acpi::{FadtInfo, HpetInfo, McfgEntry};
pub use crate::firmware::devicetree::{DtDeviceEntry, FdtHeader, FdtNode};
pub use crate::firmware::smbios::{
    BiosInfo, CpuInfo as SmbiosCpuInfo, MemModuleInfo, SmbiosHeader, SmbiosInfo,
};
pub use crate::firmware::uefi::{
    GopInfo, RuntimeServicesTable, UefiInfo, UefiMemoryDescriptor, UefiMemoryType,
};
pub use crate::interrupt::Vector as IrqVector;
pub use crate::iommu::domain::Domain as IommuDomain;
pub use crate::memory::cache::hierarchy::CacheHierarchy;
pub use crate::memory::heap::buddy::BuddyAllocator;
pub use crate::memory::heap::bump::BumpAllocator;
pub use crate::memory::heap::slab::Slab;
pub use crate::memory::numa::node::NumaNode;
pub use crate::memory::phys::frame::Frame;
pub use crate::memory::virt::address::VirtAddr;
pub use crate::security::enclaves::Enclave;
pub use crate::security::isolation::{IsolationDomain, IsolationLevel};
pub use crate::security::speculation::SpeculationMitigation;

pub struct MemoryHal;

impl MemoryHal {
    pub fn phys_region() -> Option<(usize, usize)> {
        crate::memory::phys::allocator::PhysAllocator::region()
    }

    pub fn init_phys() -> bool {
        crate::memory::phys::allocator::PhysAllocator::init()
    }

    pub fn alloc_frame() -> Option<Frame> {
        crate::memory::phys::allocator::PhysAllocator::alloc_frame()
    }

    pub fn free_frame(f: Frame) {
        crate::memory::phys::allocator::PhysAllocator::free_frame(f)
    }

    pub fn map_page(virt: VirtAddr, frame: Frame) -> bool {
        crate::memory::virt::mapping::map_page(virt, frame)
    }

    pub fn unmap_page(virt: VirtAddr) {
        crate::memory::virt::mapping::unmap_page(virt)
    }

    pub fn map(virt: VirtAddr, frame: Frame) -> bool {
        crate::memory::virt::paging::map(virt, frame)
    }

    pub fn unmap(virt: VirtAddr) {
        crate::memory::virt::paging::unmap(virt)
    }

    pub fn mapped_count() -> usize {
        crate::memory::virt::paging::mapped_count()
    }

    pub fn unmapped_count() -> usize {
        crate::memory::virt::paging::unmapped_count()
    }

    pub fn detect_cache() -> CacheHierarchy {
        crate::memory::cache::hierarchy::detect()
    }

    pub fn ensure_cache_coherence() {
        crate::memory::cache::coherence::ensure_coherence()
    }

    pub fn detect_numa_nodes() -> usize {
        crate::memory::numa::node::detect_nodes()
    }

    pub fn numa_node_count() -> usize {
        crate::memory::numa::node::node_count()
    }

    pub fn register_numa_node(node: &NumaNode) -> bool {
        crate::memory::numa::node::register_node(node)
    }

    pub fn new_bump_allocator() -> BumpAllocator {
        crate::memory::heap::bump::BumpAllocator::new()
    }

    pub fn new_buddy_allocator() -> BuddyAllocator {
        crate::memory::heap::buddy::BuddyAllocator::new()
    }

    pub fn new_slab() -> Slab {
        crate::memory::heap::slab::Slab::new()
    }
}

pub struct InterruptHal;

impl InterruptHal {
    pub fn register(irq: usize, handler: fn()) -> bool {
        crate::interrupt::register(irq, handler)
    }

    pub fn unregister(irq: usize) -> bool {
        crate::interrupt::unregister(irq)
    }

    pub fn dispatch(irq: usize) {
        crate::interrupt::dispatch(irq)
    }

    pub fn handle() {
        crate::interrupt::handle()
    }

    pub fn init_controller() {
        crate::interrupt::Controller::init()
    }

    pub fn enable_irq(irq: u8) {
        crate::interrupt::Controller::enable_irq(irq)
    }

    pub fn disable_irq(irq: u8) {
        crate::interrupt::Controller::disable_irq(irq)
    }

    pub fn eoi(irq: u8) {
        crate::interrupt::Controller::eoi(irq)
    }

    pub fn init_idt() {
        crate::interrupt::idt::init()
    }
}

pub struct BusHal;

impl BusHal {
    pub fn register_device(vendor: u16, device: u16, bus: u8) -> bool {
        crate::bus::discovery::register_device(vendor, device, bus)
    }

    pub fn device_count() -> usize {
        crate::bus::discovery::device_count()
    }

    pub fn device_vendor(index: usize) -> Option<u16> {
        crate::bus::discovery::device_vendor(index)
    }

    pub fn device_id(index: usize) -> Option<u16> {
        crate::bus::discovery::device_id(index)
    }
}

pub struct DmaHal;

impl DmaHal {
    pub fn init_engine() {
        crate::dma::engine::init()
    }

    pub fn alloc_buffer(size: usize, align: usize) -> Option<DmaBuffer> {
        crate::dma::buffer::DmaBuffer::new(size, align)
    }

    pub fn prepare_descriptor(buf: &DmaBuffer, flags: u32) -> DmaDescriptor {
        crate::dma::engine::DmaEngine::prepare_descriptor(buf, flags)
    }

    pub fn prepare_descriptor_with_phys(phys: usize, len: usize, flags: u32) -> DmaDescriptor {
        crate::dma::engine::DmaEngine::prepare_descriptor_with_phys(phys, len, flags)
    }

    pub fn submit(descs: &[DmaDescriptor]) -> usize {
        if let Some(engine) = crate::dma::engine::get() {
            engine.submit(descs)
        } else {
            0
        }
    }

    pub fn submit_buffer(buf: &DmaBuffer, flags: u32, align: usize) -> Result<usize, &'static str> {
        if let Some(engine) = crate::dma::engine::get() {
            engine.submit_buffer(buf, flags, align)
        } else {
            Err("engine not initialized")
        }
    }

    pub fn drain(out: &mut [DmaDescriptor]) -> usize {
        if let Some(engine) = crate::dma::engine::get() {
            engine.drain(out)
        } else {
            0
        }
    }

    pub fn unmap_descriptor(desc: &DmaDescriptor) -> bool {
        crate::dma::engine::DmaEngine::unmap_descriptor(desc)
    }

    pub fn unmap_iova(iova: usize) -> bool {
        crate::dma::engine::DmaEngine::unmap_iova(iova)
    }

    pub fn map(virt: usize, phys: usize, size: usize) -> bool {
        crate::dma::mapping::map(virt, phys, size)
    }

    pub fn translate(virt: usize) -> Option<usize> {
        crate::dma::mapping::translate(virt)
    }

    pub fn mapping_count() -> usize {
        crate::dma::mapping::mapping_count()
    }
}

pub struct IommuHal;

impl IommuHal {
    pub fn init() {
        crate::iommu::init()
    }

    pub fn init_all() {
        crate::iommu::init_all()
    }

    pub fn create_domain(base: usize, size: usize, flags: u8) -> Option<IommuDomain> {
        crate::iommu::domain::create_domain(base, size, flags)
    }

    pub fn domain_info(id: u8) -> Option<IommuDomain> {
        crate::iommu::domain::domain_info(id)
    }

    pub fn domain_count() -> u8 {
        crate::iommu::domain::domain_count()
    }

    pub fn add_mapping(iova: usize, phys: usize, size: usize) -> bool {
        crate::iommu::mapping::add_mapping(iova, phys, size)
    }

    pub fn translate(iova: usize) -> Option<usize> {
        crate::iommu::mapping::translate(iova)
    }

    pub fn mapping_count() -> usize {
        crate::iommu::mapping::mapping_count()
    }
}

pub struct FirmwareHal;

impl FirmwareHal {
    pub fn acpi_present() -> bool {
        crate::firmware::acpi::is_present()
    }

    pub fn acpi_revision() -> usize {
        crate::firmware::acpi::acpi_revision()
    }

    pub fn find_ioapic_base() -> Option<usize> {
        crate::firmware::acpi::find_ioapic_base()
    }

    pub fn find_vtd_base() -> Option<usize> {
        crate::firmware::acpi::find_vtd_base()
    }

    pub fn parse_fadt() -> Option<FadtInfo> {
        crate::firmware::acpi::parse_fadt()
    }

    pub fn fadt_dsdt_addr() -> Option<usize> {
        crate::firmware::acpi::fadt_dsdt_addr()
    }

    pub fn parse_hpet() -> Option<HpetInfo> {
        crate::firmware::acpi::parse_hpet()
    }

    pub fn parse_mcfg(out: &mut [McfgEntry; 8]) -> usize {
        crate::firmware::acpi::parse_mcfg(out)
    }

    pub fn acpi_table_count() -> usize {
        crate::firmware::acpi::table_count()
    }

    pub fn parse_devicetree() {
        crate::firmware::devicetree::parse_devicetree()
    }

    pub fn find_smmu_base() -> Option<usize> {
        crate::firmware::devicetree::find_smmu_base()
    }

    pub fn dt_present() -> bool {
        crate::firmware::devicetree::is_present()
    }

    pub fn dt_set_fdt(addr: usize, size: usize) {
        crate::firmware::devicetree::set_fdt(addr, size)
    }

    pub fn dt_fdt_address() -> Option<usize> {
        crate::firmware::devicetree::fdt_address()
    }

    pub fn dt_address_cells() -> u32 {
        crate::firmware::devicetree::address_cells()
    }

    pub fn dt_size_cells() -> u32 {
        crate::firmware::devicetree::size_cells()
    }

    pub fn parse_smbios() {
        crate::firmware::smbios::parse_smbios()
    }

    pub fn smbios_info() -> SmbiosInfo {
        crate::firmware::smbios::smbios_info()
    }

    pub fn smbios_present() -> bool {
        crate::firmware::smbios::is_present()
    }

    pub fn smbios_bios_info() -> Option<BiosInfo> {
        crate::firmware::smbios::find_bios_info()
    }

    pub fn smbios_total_ram_mb() -> u64 {
        crate::firmware::smbios::total_installed_ram_mb()
    }

    pub fn parse_uefi() {
        crate::firmware::uefi::parse_uefi()
    }

    pub fn uefi_info() -> UefiInfo {
        crate::firmware::uefi::uefi_info()
    }

    pub fn uefi_present() -> bool {
        crate::firmware::uefi::is_present()
    }

    pub fn uefi_set_memory_map(base: usize, size: usize, desc_size: usize) {
        crate::firmware::uefi::set_memory_map(base, size, desc_size)
    }

    pub fn uefi_memory_descriptor_count() -> usize {
        crate::firmware::uefi::memory_map_descriptor_count()
    }

    pub fn uefi_read_memory_descriptor(index: usize) -> Option<UefiMemoryDescriptor> {
        crate::firmware::uefi::read_memory_descriptor(index)
    }

    pub fn uefi_total_conventional_memory() -> u64 {
        crate::firmware::uefi::total_conventional_memory()
    }

    pub fn uefi_set_runtime_services(addr: usize) {
        crate::firmware::uefi::set_runtime_services(addr)
    }

    pub fn uefi_runtime_services() -> Option<RuntimeServicesTable> {
        crate::firmware::uefi::runtime_services()
    }

    pub fn uefi_set_gop(fb_base: usize, fb_size: usize, h_res: u32, v_res: u32, stride: u32) {
        crate::firmware::uefi::set_gop(fb_base, fb_size, h_res, v_res, stride)
    }

    pub fn uefi_gop_info() -> Option<GopInfo> {
        crate::firmware::uefi::gop_info()
    }

    pub fn firmware_feature_enabled(feature: u32) -> bool {
        crate::config::feature::is_enabled(feature)
    }

    pub fn enable_firmware_feature(feature: u32) {
        crate::config::feature::enable(feature)
    }

    pub fn disable_firmware_feature(feature: u32) {
        crate::config::feature::disable(feature)
    }
}

pub struct DiscoveryHal;

impl DiscoveryHal {
    pub fn register_device(dev_type: DeviceType, base_addr: usize) -> Option<DiscoveredDevice> {
        crate::discovery::registry::register_device(dev_type, base_addr)
    }

    pub fn device_count() -> u8 {
        crate::discovery::registry::device_count()
    }

    pub fn device_info(id: u8) -> Option<DiscoveredDevice> {
        crate::discovery::registry::device_info(id)
    }

    pub fn discover_all() {
        crate::discovery::registry::discover_all()
    }
}

pub struct BootHal;

impl BootHal {
    pub fn total_usable_ram() -> usize {
        crate::boot::total_usable_ram()
    }
}

pub struct SecurityHal;

impl SecurityHal {
    pub fn create_isolation_domain(level: IsolationLevel) -> Option<IsolationDomain> {
        crate::security::isolation::create_domain(level)
    }

    pub fn isolation_domain_count() -> u8 {
        crate::security::isolation::domain_count()
    }

    pub fn isolation_domain_level(id: u8) -> IsolationLevel {
        crate::security::isolation::domain_level(id)
    }

    pub fn isolate() {
        crate::security::isolation::isolate()
    }

    pub fn mitigations_active() -> bool {
        crate::security::speculation::mitigations_active()
    }

    pub fn active_mitigation() -> SpeculationMitigation {
        crate::security::speculation::active_mitigation()
    }

    pub fn apply_mitigations() {
        crate::security::speculation::mitigations()
    }

    pub fn sgx_supported() -> bool {
        crate::security::enclaves::sgx_supported()
    }

    pub fn create_enclave(base: usize, size: usize) -> Option<Enclave> {
        crate::security::enclaves::create_enclave(base, size)
    }

    pub fn enclave_count() -> u8 {
        crate::security::enclaves::enclave_count()
    }

    pub fn enclave_info(id: u8) -> Option<Enclave> {
        crate::security::enclaves::enclave_info(id)
    }
}

pub struct HardwareAccessHal;

impl HardwareAccessHal {
    pub fn mmio_read32(addr: usize) -> Option<u32> {
        crate::hardware_access::mmio_read32(addr)
    }

    pub fn mmio_write32(addr: usize, val: u32) -> bool {
        crate::hardware_access::mmio_write32(addr, val)
    }

    pub fn read_cpuid(leaf: u32, subleaf: u32) -> Option<(u32, u32, u32, u32)> {
        crate::hardware_access::read_cpuid(leaf, subleaf)
    }

    pub fn read_msr(msr: u32) -> Option<u64> {
        crate::hardware_access::read_msr(msr)
    }
}
