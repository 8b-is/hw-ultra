#[derive(Copy, Clone, PartialEq)]
pub enum PixelFormat {
    Rgb565,
    Rgb888,
    Rgba8888,
    Bgra8888,
    Xrgb8888,
    Yuv420,
    Yuv422,
}

pub fn bytes_per_pixel(fmt: PixelFormat) -> u8 {
    match fmt {
        PixelFormat::Rgb565 => 2,
        PixelFormat::Rgb888 => 3,
        PixelFormat::Rgba8888 | PixelFormat::Bgra8888 | PixelFormat::Xrgb8888 => 4,
        PixelFormat::Yuv420 => 1,
        PixelFormat::Yuv422 => 2,
    }
}

pub fn compute_stride(width: u32, fmt: PixelFormat) -> u32 {
    let bpp = bytes_per_pixel(fmt) as u32;
    let raw = width * bpp;
    (raw + 63) & !63
}

pub fn framebuffer_size(width: u32, height: u32, fmt: PixelFormat) -> usize {
    (compute_stride(width, fmt) * height) as usize
}

pub fn pixel_offset(x: u32, y: u32, stride_bytes: u32, fmt: PixelFormat) -> usize {
    (y * stride_bytes + x * bytes_per_pixel(fmt) as u32) as usize
}

pub fn write_pixel(fb: usize, x: u32, y: u32, stride_bytes: u32, fmt: PixelFormat, color: u32) {
    let off = pixel_offset(x, y, stride_bytes, fmt);
    let ptr = fb + off;
    match fmt {
        PixelFormat::Rgb565 => unsafe {
            core::ptr::write_volatile(ptr as *mut u16, color as u16);
        },
        PixelFormat::Rgb888 => unsafe {
            core::ptr::write_volatile(ptr as *mut u8, (color & 0xFF) as u8);
            core::ptr::write_volatile((ptr + 1) as *mut u8, ((color >> 8) & 0xFF) as u8);
            core::ptr::write_volatile((ptr + 2) as *mut u8, ((color >> 16) & 0xFF) as u8);
        },
        _ => unsafe {
            core::ptr::write_volatile(ptr as *mut u32, color);
        },
    }
}

pub fn clear(fb: usize, total_size: usize, val: u8) {
    let ptr = fb as *mut u8;
    let mut i = 0usize;
    while i < total_size {
        unsafe { core::ptr::write_volatile(ptr.add(i), val) };
        i += 1;
    }
}
