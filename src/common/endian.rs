pub fn is_little_endian() -> bool {
    let val: u32 = 1;
    let bytes = val.to_ne_bytes();
    bytes[0] == 1
}

pub fn swap_u16(val: u16) -> u16 {
    val.swap_bytes()
}

pub fn swap_u32(val: u32) -> u32 {
    val.swap_bytes()
}

pub fn swap_u64(val: u64) -> u64 {
    val.swap_bytes()
}

pub fn u16_from_be(val: u16) -> u16 {
    u16::from_be(val)
}

pub fn u32_from_be(val: u32) -> u32 {
    u32::from_be(val)
}

pub fn u16_to_be(val: u16) -> u16 {
    val.to_be()
}

pub fn u32_to_be(val: u32) -> u32 {
    val.to_be()
}
