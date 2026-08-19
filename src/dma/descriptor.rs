#[derive(Copy, Clone, Debug)]
pub struct Descriptor {
    pub phys: usize,
    pub len: usize,
    pub flags: u32,
}

impl Descriptor {
    pub fn new(phys: usize, len: usize, flags: u32) -> Self {
        Descriptor { phys, len, flags }
    }
}
