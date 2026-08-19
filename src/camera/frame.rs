#[derive(Clone, Copy, PartialEq)]
pub enum PixelFormat {
    Raw8,
    Raw10,
    Raw12,
    Raw14,
    Yuv422,
    Rgb888,
    Nv12,
    Nv21,
    Jpeg,
}

#[derive(Clone, Copy)]
pub struct FrameConfig {
    pub width: u16,
    pub height: u16,
    pub format: PixelFormat,
    pub stride: u32,
    pub buffer_addr: usize,
    pub buffer_size: usize,
}

pub fn bytes_per_pixel(format: PixelFormat) -> u8 {
    match format {
        PixelFormat::Raw8 => 1,
        PixelFormat::Raw10 => 2,
        PixelFormat::Raw12 => 2,
        PixelFormat::Raw14 => 2,
        PixelFormat::Yuv422 => 2,
        PixelFormat::Rgb888 => 3,
        PixelFormat::Nv12 => 1,
        PixelFormat::Nv21 => 1,
        PixelFormat::Jpeg => 1,
    }
}

pub fn compute_stride(width: u16, format: PixelFormat) -> u32 {
    let raw = width as u32 * bytes_per_pixel(format) as u32;
    (raw + 63) & !63
}

pub fn compute_buffer_size(config: &FrameConfig) -> usize {
    let plane0 = config.stride as usize * config.height as usize;
    match config.format {
        PixelFormat::Nv12 | PixelFormat::Nv21 => plane0 + plane0 / 2,
        _ => plane0,
    }
}

pub fn frame_offset(config: &FrameConfig, x: u16, y: u16) -> usize {
    y as usize * config.stride as usize + x as usize * bytes_per_pixel(config.format) as usize
}
