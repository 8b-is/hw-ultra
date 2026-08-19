use crate::arch::Architecture;

pub fn probe_bar_size(bus: u8, device: u8, function: u8, offset: u8) -> Option<usize> {
    let original = read_config_u32(bus, device, function, offset)?;
    if original == 0 || original == 0xffff_ffff {
        return None;
    }
    if !write_config_u32(bus, device, function, offset, 0xffff_ffff) {
        return None;
    }
    let mask = read_config_u32(bus, device, function, offset)?;
    let restore_ok = write_config_u32(bus, device, function, offset, original);
    if !restore_ok {
        return None;
    }
    let size_mask = mask & 0xffff_fff0;
    if size_mask == 0 {
        None
    } else {
        Some(((!size_mask).wrapping_add(1)) as usize)
    }
}

pub fn map_mmio_region(base: usize, size: usize) -> usize {
    if size == 0 {
        return base;
    }
    base
}

pub fn read_config_u32(bus: u8, device: u8, function: u8, offset: u8) -> Option<u32> {
    match crate::arch::detect_arch() {
        Architecture::X86_64 => {
            let addr: u32 = 0x8000_0000u32
                | ((bus as u32) << 16)
                | ((device as u32) << 11)
                | ((function as u32) << 8)
                | ((offset as u32) & 0xfc);
            unsafe {
                crate::arch::x86_64::io::outl(0xCF8, addr);
                let val = crate::arch::x86_64::io::inl(0xCFC);
                Some(val)
            }
        }
        other => {
            static PCI_FN_SIG: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            PCI_FN_SIG.store(other as usize, core::sync::atomic::Ordering::Release);
            None
        }
    }
}

pub fn write_config_u32(bus: u8, device: u8, function: u8, offset: u8, val: u32) -> bool {
    match crate::arch::detect_arch() {
        Architecture::X86_64 => {
            let addr: u32 = 0x8000_0000u32
                | ((bus as u32) << 16)
                | ((device as u32) << 11)
                | ((function as u32) << 8)
                | ((offset as u32) & 0xfc);
            unsafe {
                crate::arch::x86_64::io::outl(0xCF8, addr);
                crate::arch::x86_64::io::outl(0xCFC, val);
            }
            true
        }
        _ => false,
    }
}

pub fn scan_video_devices(out: &mut [crate::gpu::GpuDevice]) -> usize {
    let mut found = 0usize;
    if out.is_empty() {
        return 0;
    }
    match crate::arch::detect_arch() {
        Architecture::X86_64 => {
            for bus in 0u8..=255u8 {
                for dev in 0u8..32u8 {
                    for func in 0u8..8u8 {
                        if let Some(vd) = read_config_u32(bus, dev, func, 0) {
                            let vendor = (vd & 0xffff) as u16;
                            if vendor == 0xffff {
                                continue;
                            }
                            let cl = read_config_u32(bus, dev, func, 0x08).unwrap_or(0);
                            let class = ((cl >> 24) & 0xff) as u8;
                            let subclass = ((cl >> 16) & 0xff) as u8;
                            let prog_if = ((cl >> 8) & 0xff) as u8;
                            if class == 0x03 {
                                let did = ((vd >> 16) & 0xffff) as u16;
                                let bar0 = read_config_u32(bus, dev, func, 0x10).unwrap_or(0);
                                if found < out.len() {
                                    out[found] = crate::gpu::GpuDevice {
                                        bus,
                                        device: dev,
                                        function: func,
                                        vendor_id: vendor,
                                        device_id: did,
                                        class,
                                        subclass,
                                        prog_if,
                                        bar0,
                                    };
                                    found += 1;
                                } else {
                                    return found;
                                }
                            }
                        }
                    }
                }
            }
            if found > 0 {
                return found;
            }
            0
        }
        _ => 0,
    }
}

pub fn enable_and_register_all_device_irqs() {
    match crate::arch::detect_arch() {
        Architecture::X86_64 => {
            for bus in 0u8..=255u8 {
                for dev in 0u8..32u8 {
                    for func in 0u8..8u8 {
                        if let Some(vd) = read_config_u32(bus, dev, func, 0) {
                            let vendor = (vd & 0xffff) as u16;
                            if vendor == 0xffff {
                                continue;
                            }
                            if let Some(d) = read_config_u32(bus, dev, func, 0x3C) {
                                let irq_line = (d & 0xff) as u8;
                                if irq_line != 0 && irq_line != 0xff {
                                    crate::interrupt::Controller::enable_irq(irq_line);
                                    let vec = 0x20u8.wrapping_add(irq_line);
                                    if let Some(idt) = crate::interrupt::Idt::get() {
                                        let ok =
                                            idt.register(vec as usize, crate::interrupt::handle);
                                        debug_assert!(ok);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            static PCI_IRQ_SCAN_SIG: core::sync::atomic::AtomicUsize =
                core::sync::atomic::AtomicUsize::new(0);
            PCI_IRQ_SCAN_SIG.store(0, core::sync::atomic::Ordering::Release);
        }
    }
}
