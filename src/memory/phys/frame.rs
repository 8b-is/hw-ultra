#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Frame(usize);

impl Frame {
    pub const fn new(pfn: usize) -> Self {
        Frame(pfn)
    }
    pub const fn as_usize(self) -> usize {
        self.0
    }
}

impl From<usize> for Frame {
    fn from(v: usize) -> Self {
        Frame(v)
    }
}

impl From<Frame> for usize {
    fn from(f: Frame) -> usize {
        f.0
    }
}
