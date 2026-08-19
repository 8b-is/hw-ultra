#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtAddr(usize);

impl VirtAddr {
    pub const fn new(addr: usize) -> Self {
        VirtAddr(addr)
    }
    pub const fn as_usize(self) -> usize {
        self.0
    }
}

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        VirtAddr(v)
    }
}

impl From<VirtAddr> for usize {
    fn from(a: VirtAddr) -> usize {
        a.0
    }
}
