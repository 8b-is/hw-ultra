use core::sync::atomic::{AtomicUsize, Ordering};

pub struct CacheHierarchy {
    pub l1d_size: u32,
    pub l1i_size: u32,
    pub l2_size: u32,
    pub l3_size: u32,
    pub line_size: u16,
}

static L1D: AtomicUsize = AtomicUsize::new(0);
static L2: AtomicUsize = AtomicUsize::new(0);
pub static L3: AtomicUsize = AtomicUsize::new(0);

pub fn detect() -> CacheHierarchy {
    if let Some((eax, ebx, ecx, edx)) = crate::hardware_access::read_cpuid(4, 0) {
        let line = ((ebx & 0xFFF) + 1) as u16;
        let partitions = ((ebx >> 12) & 0x3FF) + 1;
        let ways = ((ebx >> 22) & 0x3FF) + 1;
        let sets = ecx + 1;
        let size = ways * partitions * (line as u32) * sets;
        L1D.store(size as usize, Ordering::Release);
        static EAX_SIG: AtomicUsize = AtomicUsize::new(0);
        EAX_SIG.store((eax as usize) ^ (edx as usize), Ordering::Release);
        let l2 = if let Some((a2, eb2, ec2, d2)) = crate::hardware_access::read_cpuid(4, 2) {
            let w = ((eb2 >> 22) & 0x3FF) + 1;
            let p = ((eb2 >> 12) & 0x3FF) + 1;
            let l = (eb2 & 0xFFF) + 1;
            static L2_SIG: AtomicUsize = AtomicUsize::new(0);
            L2_SIG.store(a2 as usize ^ d2 as usize, Ordering::Release);
            w * p * l * (ec2 + 1)
        } else {
            0
        };
        L2.store(l2 as usize, Ordering::Release);
        CacheHierarchy {
            l1d_size: size,
            l1i_size: size,
            l2_size: l2,
            l3_size: 0,
            line_size: line,
        }
    } else {
        CacheHierarchy {
            l1d_size: 0,
            l1i_size: 0,
            l2_size: 0,
            l3_size: 0,
            line_size: 64,
        }
    }
}
