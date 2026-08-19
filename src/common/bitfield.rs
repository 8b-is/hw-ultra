#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BitField {
    pub offset: u8,
    pub width: u8,
}

impl BitField {
    pub const fn new(offset: u8, width: u8) -> Self {
        Self { offset, width }
    }

    pub const fn mask_u64(&self) -> u64 {
        if self.width == 0 {
            0
        } else {
            ((1u64 << (self.width as u64)) - 1) << (self.offset as u64)
        }
    }

    pub const fn mask_u32(&self) -> u32 {
        self.mask_u64() as u32
    }

    pub fn extract_u32(&self, value: u32) -> u32 {
        (value >> self.offset) & ((1u32 << self.width) - 1)
    }

    pub fn insert_u32(&self, value: u32, field: u32) -> u32 {
        let mask = self.mask_u32();
        let f = (field << self.offset) & mask;
        (value & !mask) | f
    }

    pub fn extract_u64(&self, value: u64) -> u64 {
        (value >> (self.offset as u64)) & ((1u64 << (self.width as u64)) - 1)
    }

    pub fn insert_u64(&self, value: u64, field: u64) -> u64 {
        let mask = self.mask_u64();
        let f = (field << (self.offset as u64)) & mask;
        (value & !mask) | f
    }
}
