pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub format: u32,
}

impl Texture {
    pub fn new(width: usize, height: usize, format: u32) -> Self {
        Texture {
            width,
            height,
            format,
        }
    }

    pub fn size_bytes(&self) -> usize {
        self.width * self.height * 4
    }
}
