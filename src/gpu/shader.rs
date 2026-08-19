#[derive(Copy, Clone)]
pub struct Shader {
    pub bytecode_ptr: *const u8,
    pub bytecode_len: usize,
}

impl Shader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Shader {
            bytecode_ptr: bytes.as_ptr(),
            bytecode_len: bytes.len(),
        }
    }
    pub fn len(&self) -> usize {
        self.bytecode_len
    }
    pub fn is_empty(&self) -> bool {
        self.bytecode_len == 0
    }
}
