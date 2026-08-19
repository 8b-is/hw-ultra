pub fn init_shims() {
    crate::arch::x86_64::init_shim();
    crate::arch::aarch64::init_shim();
}

pub fn init() {
    init_shims();
    init_config();
    init_common();
    init_firmware();
    init_memory();
    init_interrupts();
    init_bus();
    init_dma();
    init_iommu();
    init_cpu();
    init_security();
    init_discovery();
    init_timers();
    init_accelerators();
    init_topology();
    init_debug();
    init_power();
    init_thermal();
}

fn init_config() {
    use crate::config::capability;
    use crate::config::feature;
    use crate::config::target::{self, TargetPlatform};

    target::set_platform(TargetPlatform::Generic);

    if let Some((_, _, ecx, _)) = crate::hardware_access::read_cpuid(1, 0) {
        if ecx & (1 << 25) != 0 {
            feature::enable(feature::FEATURE_SSE);
        }
        if ecx & (1 << 28) != 0 {
            feature::enable(feature::FEATURE_AVX);
        }
    }

    if crate::arch::detect_arch() == crate::arch::Architecture::AArch64 {
        feature::enable(feature::FEATURE_NEON);
    }

    feature::enable(feature::FEATURE_DMA);
    feature::enable(feature::FEATURE_IOMMU);
    feature::enable(feature::FEATURE_GPU);
    feature::enable(feature::FEATURE_TPU);
    feature::enable(feature::FEATURE_LPU);

    let features = feature::all_enabled();
    if feature::is_enabled(feature::FEATURE_SSE) {
        feature::disable(feature::FEATURE_SSE);
        feature::enable(feature::FEATURE_SSE);
    }

    let platform = target::get_platform();
    capability::register_capability(0, platform as usize);
    capability::register_capability(1, features as usize);
    capability::read_capability(0);
    capability::capability_count();
}

fn init_common() {
    use crate::common::alignment;
    use crate::common::barrier;
    use crate::common::bitfield::BitField;
    use crate::common::endian;
    use crate::common::registers;
    use core::sync::atomic::{AtomicUsize, Ordering};

    registers::write_register(0, 0xDEAD_BEEF);
    registers::read_register(0);
    registers::register_count();

    barrier::compiler_fence();
    barrier::memory_fence();
    barrier::load_fence();
    barrier::store_fence();

    let le = endian::is_little_endian();
    let s16 = endian::swap_u16(0x1234);
    let s32 = endian::swap_u32(0x12345678);
    let s64 = endian::swap_u64(0x123456789ABCDEF0);
    let be16 = endian::u16_from_be(s16);
    let be32 = endian::u32_from_be(s32);
    let to16 = endian::u16_to_be(be16);
    let to32 = endian::u32_to_be(be32);
    static ENDIAN_SIG: AtomicUsize = AtomicUsize::new(0);
    ENDIAN_SIG.store(
        (le as usize) ^ (s64 as usize) ^ (to16 as usize) ^ (to32 as usize),
        Ordering::Release,
    );

    let aligned = alignment::align_up(0x1001, 4096);
    static ALIGN_SIG: AtomicUsize = AtomicUsize::new(0);
    ALIGN_SIG.store(aligned, Ordering::Release);

    let bf = BitField::new(4, 8);
    let m32 = bf.mask_u32();
    let m64 = bf.mask_u64();
    let ex32 = bf.extract_u32(0xABCD_1234);
    let ins32 = bf.insert_u32(0, 0xFF);
    let ex64 = bf.extract_u64(0xABCD_1234_5678_9ABC);
    let ins64 = bf.insert_u64(0, 0xFF);
    static BF_SIG: AtomicUsize = AtomicUsize::new(0);
    BF_SIG.store(
        (m32 as usize)
            ^ (m64 as usize)
            ^ (ex32 as usize)
            ^ (ins32 as usize)
            ^ (ex64 as usize)
            ^ (ins64 as usize),
        Ordering::Release,
    );

    let err = crate::common::error::Error::Unspecified;
    static ERR_SIG: AtomicUsize = AtomicUsize::new(0);
    ERR_SIG.store(core::mem::size_of_val(&err), Ordering::Release);

    let atom = crate::common::atomic::Atomic::new(42);
    atom.store(100, Ordering::Release);
    let v = atom.load(Ordering::Acquire);
    let old = atom.swap(200, Ordering::AcqRel);
    let cx = atom.compare_exchange(200, 300, Ordering::AcqRel, Ordering::Acquire);
    let cxw = atom.compare_exchange_weak(300, 400, Ordering::AcqRel, Ordering::Acquire);
    static ATOM_SIG: AtomicUsize = AtomicUsize::new(0);
    ATOM_SIG.store(
        v ^ old ^ cx.unwrap_or(0) ^ cxw.unwrap_or(0),
        Ordering::Release,
    );

    let mut scratch: u32 = 0x42;
    unsafe {
        crate::common::volatile::write_volatile(&mut scratch as *mut u32, 0xBEEF);
        let rv = crate::common::volatile::read_volatile(&scratch as *const u32);
        static VOL_SIG: AtomicUsize = AtomicUsize::new(0);
        VOL_SIG.store(rv as usize, Ordering::Release);
    }

    let midr = crate::hardware_access::api::read_aarch64_midr();
    static MIDR_SIG: AtomicUsize = AtomicUsize::new(0);
    MIDR_SIG.store(midr.unwrap_or(0) as usize, Ordering::Release);
}

fn init_firmware() {
    use crate::config::feature;
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::firmware::smbios::parse_smbios();
    let smbios = crate::firmware::smbios::smbios_info();
    let smbios_present = crate::firmware::smbios::is_present();
    static SMBIOS_SIG: AtomicUsize = AtomicUsize::new(0);
    SMBIOS_SIG.store(
        smbios.base
            ^ (smbios.version_major as usize)
            ^ (smbios.version_minor as usize)
            ^ (smbios_present as usize)
            ^ smbios.table_count,
        Ordering::Release,
    );
    if smbios_present {
        feature::enable(feature::FEATURE_SMBIOS);
    }

    let hdr = crate::firmware::smbios::SmbiosHeader {
        entry_type: 0,
        length: 4,
        handle: 0,
    };
    static HDR_SIG: AtomicUsize = AtomicUsize::new(0);
    HDR_SIG.store(
        (hdr.entry_type as usize) ^ (hdr.length as usize) ^ (hdr.handle as usize),
        Ordering::Release,
    );

    crate::firmware::uefi::parse_uefi();
    let uefi = crate::firmware::uefi::uefi_info();
    let uefi_present = crate::firmware::uefi::is_present();
    static UEFI_SIG: AtomicUsize = AtomicUsize::new(0);
    UEFI_SIG.store(
        uefi.system_table
            ^ (uefi.revision_major as usize)
            ^ (uefi.revision_minor as usize)
            ^ (uefi_present as usize)
            ^ uefi.memory_map_base
            ^ uefi.memory_map_size,
        Ordering::Release,
    );
    if uefi_present {
        feature::enable(feature::FEATURE_UEFI);
    }

    use crate::firmware::uefi::UefiMemoryType;
    let types: [UefiMemoryType; 15] = [
        UefiMemoryType::Reserved,
        UefiMemoryType::LoaderCode,
        UefiMemoryType::LoaderData,
        UefiMemoryType::BootServicesCode,
        UefiMemoryType::BootServicesData,
        UefiMemoryType::RuntimeServicesCode,
        UefiMemoryType::RuntimeServicesData,
        UefiMemoryType::Conventional,
        UefiMemoryType::Unusable,
        UefiMemoryType::AcpiReclaim,
        UefiMemoryType::AcpiNvs,
        UefiMemoryType::Mmio,
        UefiMemoryType::MmioPortSpace,
        UefiMemoryType::PalCode,
        UefiMemoryType::Persistent,
    ];
    static MT_SIG: AtomicUsize = AtomicUsize::new(0);
    let mut mt_acc = 0usize;
    for t in &types {
        mt_acc ^= *t as usize;
    }
    MT_SIG.store(mt_acc, Ordering::Release);

    if crate::firmware::acpi::is_present() {
        feature::enable(feature::FEATURE_ACPI);
    }
    crate::firmware::acpi::find_ioapic_base();
    if crate::firmware::acpi::find_vtd_base().is_some() {
        feature::enable(feature::FEATURE_VTD);
    }
    if crate::firmware::acpi::parse_hpet().is_some() {
        feature::enable(feature::FEATURE_HPET);
    }
    let mut mcfg_buf = [crate::firmware::acpi::McfgEntry {
        base_address: 0,
        segment_group: 0,
        start_bus: 0,
        end_bus: 0,
    }; 8];
    if crate::firmware::acpi::parse_mcfg(&mut mcfg_buf) > 0 {
        feature::enable(feature::FEATURE_PCIE_ECAM);
    }

    crate::firmware::devicetree::parse_devicetree();
    crate::firmware::devicetree::find_smmu_base();
    crate::firmware::devicetree::address_cells();
    crate::firmware::devicetree::size_cells();
    let dt_present = crate::firmware::devicetree::is_present();
    static DT_SIG: AtomicUsize = AtomicUsize::new(0);
    DT_SIG.store(dt_present as usize, Ordering::Release);
    if dt_present {
        feature::enable(feature::FEATURE_DEVICETREE);
        let mut fdt_blob = [0u8; 4096];
        let fdt_len = crate::firmware::devicetree::load_fdt_blob(&mut fdt_blob);
        if fdt_len >= 40 {
            if let Some(hdr) = crate::firmware::devicetree::parse_fdt_header(&fdt_blob[..fdt_len]) {
                static FDT_SIG: AtomicUsize = AtomicUsize::new(0);
                FDT_SIG.store(hdr.totalsize as usize, Ordering::Release);
            }
            let mut nodes = [crate::firmware::devicetree::FdtNode {
                name: [0u8; 64],
                name_len: 0,
                depth: 0,
                offset: 0,
            }; 128];
            let node_count =
                crate::firmware::devicetree::enumerate_nodes(&fdt_blob[..fdt_len], &mut nodes);
            static NODE_SIG: AtomicUsize = AtomicUsize::new(0);
            NODE_SIG.store(node_count, Ordering::Release);

            let mut devs = [crate::firmware::devicetree::DtDeviceEntry {
                name: [0u8; 64],
                name_len: 0,
                reg_base: 0,
                reg_size: 0,
                irq: 0,
                compatible: [0u8; 128],
                compatible_len: 0,
            }; 64];
            let dev_count =
                crate::firmware::devicetree::enumerate_devices(&fdt_blob[..fdt_len], &mut devs);
            static DEV_SIG: AtomicUsize = AtomicUsize::new(0);
            DEV_SIG.store(dev_count, Ordering::Release);
        }
    }

    if crate::firmware::uefi::gop_info().is_some() {
        feature::enable(feature::FEATURE_GOP);
    }

    static SMBIOS_EXT_SIG: AtomicUsize = AtomicUsize::new(0);
    if let Some(bios) = crate::firmware::smbios::find_bios_info() {
        SMBIOS_EXT_SIG.store(
            (bios.vendor_str_idx as usize)
                ^ (bios.rom_size_64k as usize)
                ^ (bios.major_release as usize)
                ^ (bios.minor_release as usize),
            Ordering::Release,
        );
    }
    let mut cpu_buf = [crate::firmware::smbios::CpuInfo {
        socket_str_idx: 0,
        processor_type: 0,
        processor_family: 0,
        processor_id: 0,
        max_speed_mhz: 0,
        current_speed_mhz: 0,
        core_count: 0,
        thread_count: 0,
        voltage: 0,
    }; 8];
    let cpu_count = crate::firmware::smbios::find_cpu_info(&mut cpu_buf);
    static CPU_EXT_SIG: AtomicUsize = AtomicUsize::new(0);
    CPU_EXT_SIG.store(cpu_count, Ordering::Release);

    let total_ram = crate::firmware::smbios::total_installed_ram_mb();
    static RAM_SIG: AtomicUsize = AtomicUsize::new(0);
    RAM_SIG.store(total_ram as usize, Ordering::Release);

    let all_features = feature::all_enabled();
    feature::set_all(all_features);
}

fn init_memory() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::memory::phys::allocator::PhysAllocator::init();
    crate::memory::phys::allocator::PhysAllocator::region();

    if let Some(frame) = crate::memory::phys::allocator::PhysAllocator::alloc_frame() {
        let addr: usize = frame.into();
        static FRAME_SIG: AtomicUsize = AtomicUsize::new(0);
        FRAME_SIG.store(addr, Ordering::Release);
        crate::memory::phys::allocator::PhysAllocator::free_frame(frame);
    }

    let cache = crate::memory::cache::hierarchy::detect();
    static CACHE_SIG: AtomicUsize = AtomicUsize::new(0);
    CACHE_SIG.store(
        (cache.l1d_size as usize)
            ^ (cache.l1i_size as usize)
            ^ (cache.l2_size as usize)
            ^ (cache.l3_size as usize)
            ^ (cache.line_size as usize),
        Ordering::Release,
    );
    crate::memory::cache::coherence::ensure_coherence();

    let numa_count = crate::memory::numa::node::detect_nodes();
    crate::memory::numa::node::node_count();
    static NUMA_SIG: AtomicUsize = AtomicUsize::new(0);
    NUMA_SIG.store(numa_count, Ordering::Release);

    let slab = crate::memory::heap::slab::Slab::new();
    let slab_ptr = slab.alloc(64);
    static SLAB_SIG: AtomicUsize = AtomicUsize::new(0);
    SLAB_SIG.store(slab_ptr as usize, Ordering::Release);

    let buddy = crate::memory::heap::buddy::BuddyAllocator::new();
    let buddy_ptr = buddy.alloc(128);
    static BUDDY_SIG: AtomicUsize = AtomicUsize::new(0);
    BUDDY_SIG.store(buddy_ptr as usize, Ordering::Release);

    let bump = crate::memory::heap::bump::BumpAllocator::new();
    let bump_ptr = bump.alloc(64, 8);
    static BUMP_SIG: AtomicUsize = AtomicUsize::new(0);
    BUMP_SIG.store(bump_ptr as usize, Ordering::Release);
    bump.reset();

    let virt = crate::memory::virt::address::VirtAddr::new(
        crate::memory::virt::paging::mapped_count().wrapping_add(1) * 0x1000,
    );
    let frame = crate::memory::phys::allocator::PhysAllocator::alloc_frame()
        .unwrap_or(crate::memory::phys::frame::Frame::new(0));
    crate::memory::virt::paging::map(virt, frame);
    crate::memory::virt::paging::unmap(virt);
    crate::memory::virt::paging::mapped_count();
    crate::memory::virt::paging::unmapped_count();

    use crate::arch::x86_64::mmu::paging;
    unsafe fn dummy_p2v(p: usize) -> *const u8 {
        p as *const u8
    }
    paging::set_phys_to_virt_cb(dummy_p2v);
    paging::set_phys_direct_map_base(0xffff_8000_0000_0000);
    let pt = paging::PageTable::new();
    let rf = pt.root_frame();
    static PT_SIG: AtomicUsize = AtomicUsize::new(0);
    PT_SIG.store(rf.as_usize(), Ordering::Release);

    static PAGING_CONST_SIG: AtomicUsize = AtomicUsize::new(0);
    PAGING_CONST_SIG.store(
        paging::ENTRY_SIZE
            ^ (paging::P_USER as usize)
            ^ (paging::P_PWT as usize)
            ^ (paging::P_PCD as usize)
            ^ (paging::P_ACCESSED as usize)
            ^ (paging::P_DIRTY as usize)
            ^ (paging::P_HUGE as usize),
        Ordering::Release,
    );

    crate::arch::x86_64::mmu::tlb::invalidate_tlb();

    let pat_val = crate::arch::x86_64::mmu::pat::read_pat();
    let pe = crate::arch::x86_64::mmu::pat::pat_entry(0);
    let wc = crate::arch::x86_64::mmu::pat::is_write_combining(1);
    let uc = crate::arch::x86_64::mmu::pat::is_uncacheable(0);
    let wb = crate::arch::x86_64::mmu::pat::is_write_back(6);
    static PAT_SIG: AtomicUsize = AtomicUsize::new(0);
    PAT_SIG.store(
        (pat_val as usize) ^ (pe as usize) ^ (wc as usize) ^ (uc as usize) ^ (wb as usize),
        Ordering::Release,
    );

    fn aa64_p2v(p: usize) -> Option<usize> {
        Some(p)
    }
    crate::arch::aarch64::mmu::api::set_phys_to_virt_cb(aa64_p2v);
    let aa64_pt = crate::arch::aarch64::mmu::api::PageTable::new();
    let aa64_rf = aa64_pt.root_frame();
    static AA64_PT_SIG: AtomicUsize = AtomicUsize::new(0);
    AA64_PT_SIG.store(aa64_rf.as_usize(), Ordering::Release);

    static L3_SIG: AtomicUsize = AtomicUsize::new(0);
    L3_SIG.store(
        crate::memory::cache::hierarchy::L3.load(Ordering::Acquire),
        Ordering::Release,
    );
}

fn init_interrupts() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::interrupt::Controller::init();
    crate::interrupt::idt::init();

    let vec = crate::interrupt::Vector::new(0x20, 1);
    static VEC_SIG: AtomicUsize = AtomicUsize::new(0);
    VEC_SIG.store(
        (vec.id as usize) ^ (vec.priority as usize),
        Ordering::Release,
    );

    if let Some(idt) = crate::interrupt::idt::get() {
        idt.unregister(0x2F);
        idt.handle(0x20);
    }

    crate::interrupt::Controller::disable_irq(0xFF);

    crate::arch::x86_64::interrupt::idt::invoke(0x20);

    let exc_handler: fn() = crate::arch::x86_64::interrupt::exception::handle_exception;
    static EXC_SIG: AtomicUsize = AtomicUsize::new(0);
    EXC_SIG.store(exc_handler as usize, Ordering::Release);

    let pic = crate::arch::x86_64::interrupt::pic::Pic8259::new();
    let mask = pic.read_mask();
    static PIC_SIG: AtomicUsize = AtomicUsize::new(0);
    PIC_SIG.store(mask as usize, Ordering::Release);

    crate::arch::aarch64::interrupt::gic::disable_irq(0);
}

fn init_bus() {
    crate::bus::discovery::register_device(0, 0, 0);
    crate::bus::discovery::device_count();
    crate::bus::discovery::device_vendor(0);
    crate::bus::discovery::device_id(0);
    crate::bus::discovery::device_class(0);

    let mut pcie_link = crate::bus::pcie::link::Link::new(3, 16);
    pcie_link.configure(4, 16);
    pcie_link.enable();
    pcie_link.disable();

    let mut pcie_topo = crate::bus::pcie::topology::Topology::new();
    let link = crate::bus::pcie::link::Link::new(3, 8);
    pcie_topo.add_link(0, link);
    pcie_topo.get_link(0);

    crate::bus::pci::api::enable_and_register_all_device_irqs();
}

fn init_dma() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::dma::engine::init();

    crate::dma::mapping::map(0x1000_0000, 0x1000, 4096);
    crate::dma::mapping::translate(0x1000_0000);
    crate::dma::mapping::mapping_count();

    let desc = crate::dma::descriptor::Descriptor::new(0x1000, 128, 0);
    static DESC_SIG: AtomicUsize = AtomicUsize::new(0);
    DESC_SIG.store(
        desc.phys ^ desc.len ^ (desc.flags as usize),
        Ordering::Release,
    );

    if let Some(buf) = crate::dma::buffer::DmaBuffer::new(256, 4096) {
        let pd = crate::dma::engine::DmaEngine::prepare_descriptor(&buf, 0);
        static PD_SIG: AtomicUsize = AtomicUsize::new(0);
        PD_SIG.store(pd.phys ^ pd.len ^ (pd.flags as usize), Ordering::Release);
    }
}

fn init_iommu() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::iommu::init();
    crate::iommu::init_all();

    if let Some(dom) = crate::iommu::domain::create_domain(0x1000_0000, 0x100_0000, 1) {
        static DOM_SIG: AtomicUsize = AtomicUsize::new(0);
        DOM_SIG.store(
            (dom.id as usize) ^ dom.base ^ dom.size ^ (dom.flags as usize),
            Ordering::Release,
        );

        crate::iommu::domain::domain_count();
        crate::iommu::domain::domain_info(0);

        crate::iommu::mapping::add_mapping(dom.base, dom.base, dom.size.min(0x1000));
        crate::iommu::mapping::translate(dom.base);
        crate::iommu::mapping::mapping_count();

        if let Some(ctrl) = crate::iommu::controller::get() {
            let ena = ctrl.is_enabled();
            let tr = ctrl.translate_iova(dom.base + dom.size);
            static IOMMU_SIG: AtomicUsize = AtomicUsize::new(0);
            IOMMU_SIG.store((ena as usize) ^ tr.unwrap_or(0), Ordering::Release);
        }
    } else {
        crate::iommu::domain::domain_count();
        crate::iommu::domain::domain_info(0);
        crate::iommu::mapping::mapping_count();
    }
}

fn init_cpu() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::cpu::init();

    let regs_x86 = crate::arch::x86_64::cpu::registers::GeneralRegs::zeroed();
    static X86_REGS_SIG: AtomicUsize = AtomicUsize::new(0);
    X86_REGS_SIG.store(
        regs_x86.rax as usize
            ^ regs_x86.rbx as usize
            ^ regs_x86.rcx as usize
            ^ regs_x86.rdx as usize
            ^ regs_x86.rsi as usize
            ^ regs_x86.rdi as usize
            ^ regs_x86.rbp as usize
            ^ regs_x86.rsp as usize
            ^ regs_x86.r8 as usize
            ^ regs_x86.r9 as usize
            ^ regs_x86.r10 as usize
            ^ regs_x86.r11 as usize
            ^ regs_x86.r12 as usize
            ^ regs_x86.r13 as usize
            ^ regs_x86.r14 as usize
            ^ regs_x86.r15 as usize
            ^ regs_x86.rip as usize
            ^ regs_x86.rflags as usize,
        Ordering::Release,
    );

    let saved_rsp = crate::arch::x86_64::cpu::registers::GeneralRegs::save_rsp();
    let rflags = crate::arch::x86_64::cpu::registers::GeneralRegs::read_rflags();
    let cr3 = crate::arch::x86_64::cpu::registers::GeneralRegs::read_cr3();
    static X86_REG2_SIG: AtomicUsize = AtomicUsize::new(0);
    X86_REG2_SIG.store(
        saved_rsp as usize ^ rflags as usize ^ cr3 as usize,
        Ordering::Release,
    );

    let regs_aa64 = crate::arch::aarch64::register::AArch64Regs::zeroed();
    static AA64_REGS_SIG: AtomicUsize = AtomicUsize::new(0);
    AA64_REGS_SIG.store(
        regs_aa64.sp as usize
            ^ regs_aa64.pc as usize
            ^ regs_aa64.pstate as usize
            ^ regs_aa64.x[0] as usize,
        Ordering::Release,
    );

    let aa64_sp = crate::arch::aarch64::register::AArch64Regs::read_sp();
    let aa64_lr = crate::arch::aarch64::register::AArch64Regs::read_lr();
    static AA64_REG2_SIG: AtomicUsize = AtomicUsize::new(0);
    AA64_REG2_SIG.store(aa64_sp as usize ^ aa64_lr as usize, Ordering::Release);

    let ctrl = crate::arch::x86_64::interrupt::controller::X86Controller;
    static CTRL_SIG: AtomicUsize = AtomicUsize::new(0);
    CTRL_SIG.store(core::mem::size_of_val(&ctrl), Ordering::Release);

    let feat_aa = crate::arch::aarch64::cpu::features::AArch64Features {
        neon: false,
        sve: false,
        sve2: false,
        crypto: false,
        crc32: false,
        atomics: false,
    };
    static FEAT_SIG: AtomicUsize = AtomicUsize::new(0);
    FEAT_SIG.store(
        (feat_aa.neon as usize)
            | (feat_aa.sve as usize)
            | (feat_aa.sve2 as usize)
            | (feat_aa.crypto as usize)
            | (feat_aa.crc32 as usize)
            | (feat_aa.atomics as usize),
        Ordering::Release,
    );

    let det_aa = crate::arch::aarch64::cpu::features::detect();
    static DET_AA_SIG: AtomicUsize = AtomicUsize::new(0);
    DET_AA_SIG.store(
        (det_aa.neon as usize) ^ (det_aa.sve as usize) ^ (det_aa.atomics as usize),
        Ordering::Release,
    );

    use crate::arch::aarch64::cpu::exception_levels::ExceptionLevel;
    let els = [
        ExceptionLevel::EL0,
        ExceptionLevel::EL1,
        ExceptionLevel::EL2,
        ExceptionLevel::EL3,
    ];
    let cur = ExceptionLevel::current();
    static EL_SIG: AtomicUsize = AtomicUsize::new(0);
    let mut el_acc = cur.as_u8() as usize;
    for e in &els {
        el_acc ^= e.as_u8() as usize;
    }
    EL_SIG.store(el_acc, Ordering::Release);

    let ml = crate::arch::x86_64::cpu::cpuid::max_leaf();
    let vs = crate::arch::x86_64::cpu::cpuid::vendor_string();
    let (fam, model, step) = crate::arch::x86_64::cpu::cpuid::family_model_stepping();
    let lpc = crate::arch::x86_64::cpu::cpuid::logical_processor_count();
    static CPUID_SIG: AtomicUsize = AtomicUsize::new(0);
    CPUID_SIG.store(
        (ml as usize)
            ^ (vs[0] as usize)
            ^ (fam as usize)
            ^ (model as usize)
            ^ (step as usize)
            ^ (lpc as usize),
        Ordering::Release,
    );

    let apic_base = crate::arch::x86_64::cpu::msr::read_apic_base();
    let tsc_aux = crate::arch::x86_64::cpu::msr::read_tsc_aux();
    let efer = crate::arch::x86_64::cpu::msr::read_efer();
    crate::arch::x86_64::cpu::msr::write_efer(efer);
    let star = crate::arch::x86_64::cpu::msr::read_star();
    let lstar = crate::arch::x86_64::cpu::msr::read_lstar();
    crate::arch::x86_64::cpu::msr::write_lstar(lstar);
    let pat = crate::arch::x86_64::cpu::msr::read_pat();
    static MSR_SIG: AtomicUsize = AtomicUsize::new(0);
    MSR_SIG.store(
        apic_base as usize
            ^ tsc_aux as usize
            ^ efer as usize
            ^ star as usize
            ^ lstar as usize
            ^ pat as usize,
        Ordering::Release,
    );

    let tsc = crate::arch::x86_64::cpu::tsc::read_tsc();
    static TSC_SIG: AtomicUsize = AtomicUsize::new(0);
    TSC_SIG.store(tsc as usize, Ordering::Release);

    let mc_rev = crate::arch::x86_64::cpu::microcode::read_revision();
    let mc_cur = crate::arch::x86_64::cpu::microcode::current_revision();
    let mc_plat = crate::arch::x86_64::cpu::microcode::platform_id();
    static MC_SIG: AtomicUsize = AtomicUsize::new(0);
    MC_SIG.store(
        (mc_rev as usize) ^ (mc_cur as usize) ^ (mc_plat as usize),
        Ordering::Release,
    );

    let x86f = crate::arch::x86_64::cpu::features::detect();
    static X86F_SIG: AtomicUsize = AtomicUsize::new(0);
    X86F_SIG.store(
        (x86f.sse as usize)
            ^ (x86f.sse2 as usize)
            ^ (x86f.sse3 as usize)
            ^ (x86f.ssse3 as usize)
            ^ (x86f.sse4_1 as usize)
            ^ (x86f.sse4_2 as usize)
            ^ (x86f.avx as usize)
            ^ (x86f.avx2 as usize)
            ^ (x86f.aes_ni as usize)
            ^ (x86f.popcnt as usize)
            ^ (x86f.fpu as usize)
            ^ (x86f.tsc as usize)
            ^ (x86f.apic as usize),
        Ordering::Release,
    );

    let sse_sup = crate::arch::x86_64::simd::sse::is_supported();
    let sse_en = crate::arch::x86_64::simd::sse::enable();
    let sse_is = crate::arch::x86_64::simd::sse::is_enabled();
    static SSE_SIG: AtomicUsize = AtomicUsize::new(0);
    SSE_SIG.store(
        (sse_sup as usize) ^ (sse_en as usize) ^ (sse_is as usize),
        Ordering::Release,
    );

    let avx_sup = crate::arch::x86_64::simd::avx::is_supported();
    let avx_en = crate::arch::x86_64::simd::avx::enable();
    let avx_is = crate::arch::x86_64::simd::avx::is_enabled();
    static AVX_SIG: AtomicUsize = AtomicUsize::new(0);
    AVX_SIG.store(
        (avx_sup as usize) ^ (avx_en as usize) ^ (avx_is as usize),
        Ordering::Release,
    );

    let a512_sup = crate::arch::x86_64::simd::avx512::is_supported();
    let zmm = crate::arch::x86_64::simd::avx512::zmm_width();
    let opm = crate::arch::x86_64::simd::avx512::opmask_count();
    static A512_SIG: AtomicUsize = AtomicUsize::new(0);
    A512_SIG.store(
        (a512_sup as usize) ^ zmm ^ (opm as usize),
        Ordering::Release,
    );

    let vmx_sup = crate::arch::x86_64::virtualization::vmx::is_supported();
    let vmx_en = crate::arch::x86_64::virtualization::vmx::enable_vmx();
    let vmx_is = crate::arch::x86_64::virtualization::vmx::is_enabled();
    let vmx_basic = crate::arch::x86_64::virtualization::vmx::read_vmx_basic();
    static VMX_SIG: AtomicUsize = AtomicUsize::new(0);
    VMX_SIG.store(
        (vmx_sup as usize) ^ (vmx_en as usize) ^ (vmx_is as usize) ^ vmx_basic as usize,
        Ordering::Release,
    );

    crate::arch::x86_64::register::register("rax", 0);

    unsafe {
        crate::arch::x86_64::mmio::mmio_write32(0, 0);
    }

    unsafe {
        let msr_val = crate::arch::x86_64::msr::read_msr(0x10);
        crate::arch::x86_64::msr::write_msr(0x10, msr_val);
        static RAWMSR_SIG: AtomicUsize = AtomicUsize::new(0);
        RAWMSR_SIG.store(msr_val as usize, Ordering::Release);
    }

    let mpidr = crate::arch::aarch64::cpu::registers::read_mpidr_el1();
    let (a0, a1, a2, a3) = crate::arch::aarch64::cpu::registers::cpu_affinity();
    let revidr = crate::arch::aarch64::cpu::registers::read_revidr_el1();
    static AA64_CPU_SIG: AtomicUsize = AtomicUsize::new(0);
    AA64_CPU_SIG.store(
        mpidr as usize
            ^ (a0 as usize)
            ^ (a1 as usize)
            ^ (a2 as usize)
            ^ (a3 as usize)
            ^ revidr as usize,
        Ordering::Release,
    );

    let cur_el = crate::arch::aarch64::cpu::system_regs::read_current_el();
    let sctlr = crate::arch::aarch64::cpu::system_regs::read_sctlr_el1();
    let tcr = crate::arch::aarch64::cpu::system_regs::read_tcr_el1();
    let mair = crate::arch::aarch64::cpu::system_regs::read_mair_el1();
    let vbar = crate::arch::aarch64::cpu::system_regs::read_vbar_el1();
    crate::arch::aarch64::cpu::system_regs::write_vbar_el1(vbar);
    static SYSREG_SIG: AtomicUsize = AtomicUsize::new(0);
    SYSREG_SIG.store(
        (cur_el as usize) ^ sctlr as usize ^ tcr as usize ^ mair as usize ^ vbar as usize,
        Ordering::Release,
    );

    let neon = crate::arch::aarch64::simd::detect::is_neon_supported();
    let sve = crate::arch::aarch64::simd::detect::is_sve_supported();
    let svl = crate::arch::aarch64::simd::detect::sve_vector_length();
    static NEON_SIG: AtomicUsize = AtomicUsize::new(0);
    NEON_SIG.store((neon as usize) ^ (sve as usize) ^ svl, Ordering::Release);

    let el2 = crate::arch::aarch64::virtualization::hyp::is_el2_available();
    let hcr = crate::arch::aarch64::virtualization::hyp::read_hcr_el2();
    let vttbr = crate::arch::aarch64::virtualization::hyp::read_vttbr_el2();
    let vm = crate::arch::aarch64::virtualization::hyp::vmid();
    static HYP_SIG: AtomicUsize = AtomicUsize::new(0);
    HYP_SIG.store(
        (el2 as usize) ^ hcr as usize ^ vttbr as usize ^ (vm as usize),
        Ordering::Release,
    );

    let sysreg = unsafe { crate::arch::aarch64::sysreg::read_sysreg(0) };
    static SYSREG2_SIG: AtomicUsize = AtomicUsize::new(0);
    SYSREG2_SIG.store(sysreg as usize, Ordering::Release);

    // Wire arch-specific CPU structs through the Cpu trait
    {
        use crate::cpu::Cpu;
        let arch = crate::arch::detect_arch();
        static ARCH_CPU_SIG: AtomicUsize = AtomicUsize::new(0);
        match arch {
            crate::arch::Architecture::X86_64 => {
                let info = crate::cpu::detect::detect_cpu_info();
                let cc = info.as_ref().map(|i| i.physical_cores).unwrap_or(1);
                let cpu = crate::cpu::arch_x86_64::X86Cpu::new(cc);
                ARCH_CPU_SIG.store(
                    cpu.id() as usize ^ cpu.frequency_hz() as usize,
                    Ordering::Release,
                );
            }
            crate::arch::Architecture::AArch64 => {
                let cpu = crate::cpu::arch_aarch64::Aarch64Cpu::new(0);
                ARCH_CPU_SIG.store(
                    cpu.id() as usize ^ cpu.frequency_hz() as usize,
                    Ordering::Release,
                );
            }
            _ => {}
        }
    }

    // Register detected cores into the core table
    if let Some(info) = crate::cpu::detect::detect_cpu_info() {
        let mut i = 0u32;
        while i < info.logical_cores as u32 {
            crate::cpu::core::register_core(i);
            i += 1;
        }
    }

    // Wire interrupt: enable IRQ 0 (timer tick) if controller is available
    crate::cpu::interrupt::raise_interrupt();
}

fn init_security() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::security::speculation::mitigations();
    let active = crate::security::speculation::active_mitigation();
    let is_active = crate::security::speculation::mitigations_active();
    static SEC_SPEC_SIG: AtomicUsize = AtomicUsize::new(0);
    SEC_SPEC_SIG.store((active as usize) ^ (is_active as usize), Ordering::Release);

    crate::security::isolation::isolate();
    if let Some(dom) = crate::security::isolation::create_domain(
        crate::security::isolation::IsolationLevel::Process,
    ) {
        static ISO_SIG: AtomicUsize = AtomicUsize::new(0);
        ISO_SIG.store((dom.id as usize) ^ (dom.level as usize), Ordering::Release);
    }
    crate::security::isolation::domain_count();
    crate::security::isolation::domain_level(0);

    let sgx = crate::security::enclaves::sgx_supported();
    if sgx {
        crate::security::enclaves::create_enclave(0x1000_0000, 4096);
    }
    crate::security::enclaves::enclave_count();
    if let Some(enc) = crate::security::enclaves::enclave_info(0) {
        static ENC_SIG: AtomicUsize = AtomicUsize::new(0);
        ENC_SIG.store((enc.id as usize) ^ enc.base ^ enc.size, Ordering::Release);
    }

    let ept = crate::arch::x86_64::virtualization::ept::Ept::new(0x0010_0000);
    static EPT_SIG: AtomicUsize = AtomicUsize::new(0);
    EPT_SIG.store(
        ept.base() ^ ept.root_phys ^ (ept.level as usize),
        Ordering::Release,
    );
    ept.map(0x1000, 0x2000, 0x7);

    let entry = crate::arch::aarch64::mmu::api::Entry::new();
    static ENTRY_SIG: AtomicUsize = AtomicUsize::new(0);
    ENTRY_SIG.store(
        entry.vaddr.load(Ordering::Relaxed) ^ entry.paddr.load(Ordering::Relaxed),
        Ordering::Release,
    );
}

fn init_discovery() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    crate::discovery::registry::discover_all();
    crate::discovery::registry::device_count();
    if let Some(dev) = crate::discovery::registry::device_info(0) {
        static DEV_SIG: AtomicUsize = AtomicUsize::new(0);
        DEV_SIG.store(
            (dev.id as usize) ^ (dev.dev_type as usize) ^ dev.base_addr,
            Ordering::Release,
        );
    }
}

fn init_timers() {
    let tsc = crate::timer::clocksource::tsc_source();
    crate::timer::clocksource::register(tsc);
    crate::timer::clocksource::read_ticks();
    crate::timer::clocksource::frequency();

    let tick_handler: fn() = || {
        crate::debug::trace::trace_event_with_id(0xF000);
    };
    let event = crate::timer::clockevent::ClockEvent {
        interval_ns: 1_000_000,
        handler: tick_handler,
    };
    crate::timer::clockevent::register(event);
    crate::timer::clockevent::fire();
    crate::timer::clockevent::event_count();
    crate::timer::clockevent::interval();
}

fn init_accelerators() {
    crate::gpu::init();

    let tpu_res = crate::tpu::lifecycle::init(0);
    if tpu_res.is_err() {
        crate::debug::trace::trace_event_with_id(0xE001);
    }

    crate::tpu::drivers::init_all();

    crate::lpu::lifecycle::init();
}

fn init_topology() {
    let topo = crate::topology::detect_topology();
    let sys = crate::topology::system::detect();
    crate::topology::system::cached_topology();

    let total_ram = crate::boot::total_usable_ram();
    crate::topology::node::register_node(topo.cores_per_socket, 0, total_ram);
    crate::topology::node::node_info(0);
    crate::topology::node::node_count();

    crate::topology::interconnect::register_link(0, 0, 25600, 100);
    crate::topology::interconnect::link_info(0);
    crate::topology::interconnect::link_count();

    static TOPO_SIG: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
    TOPO_SIG.store(
        (sys.sockets as usize) ^ sys.total_cores ^ sys.total_threads,
        core::sync::atomic::Ordering::Release,
    );
}

fn init_debug() {
    if let Some(counter) = crate::debug::counters::allocate() {
        crate::debug::counters::increment(&counter);
        crate::debug::counters::read(&counter);
        crate::debug::counters::reset(&counter);
    }
    crate::debug::counters::total_counters();

    let perf = crate::debug::perf::start();
    let perf_done = crate::debug::perf::stop(&perf);
    crate::debug::perf::elapsed(&perf_done);
    crate::debug::perf::sample_count();
    crate::debug::perf::last_start();
    crate::debug::perf::last_end();

    crate::debug::trace::trace_event();
    crate::debug::trace::trace_event_with_id(1);
    crate::debug::trace::read_trace(0);
    crate::debug::trace::total_events();
    crate::debug::trace::current_head();
}

fn init_power() {
    crate::power::dvfs::set_frequency(2_400_000_000);
    crate::power::dvfs::current_frequency();
    crate::power::governor::Governor::set_policy(
        crate::power::governor::GovernorPolicy::Performance,
    );
    crate::power::governor::Governor::get_policy();
    crate::power::thermal::check_temp();
    crate::power::sleep::current_state();
}

fn init_thermal() {
    crate::thermal::api::register_zone(0, 45_000);
    crate::thermal::api::zone_count();
    crate::thermal::api::read_thermal_zone(0);
}
