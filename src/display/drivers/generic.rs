use crate::display::detection::DisplayController;

pub struct LinearFbDriver {
    base: usize,
    width: u32,
    height: u32,
    stride: u32,
    initialized: bool,
}

impl LinearFbDriver {
    pub fn probe(ctrl: &DisplayController) -> Option<Self> {
        let base = if ctrl.pci {
            (ctrl.framebuffer_bar & 0xFFFF_FFF0) as usize
        } else {
            ctrl.reg_base as usize
        };
        if base == 0 {
            return None;
        }
        Some(LinearFbDriver {
            base,
            width: 0,
            height: 0,
            stride: 0,
            initialized: false,
        })
    }

    pub fn init(
        &mut self,
        width: u32,
        height: u32,
        fmt: crate::display::framebuffer::PixelFormat,
    ) -> bool {
        self.width = width;
        self.height = height;
        self.stride = crate::display::framebuffer::compute_stride(width, fmt);
        crate::display::hw::set_framebuffer(
            self.base,
            crate::display::framebuffer::framebuffer_size(width, height, fmt),
            self.stride,
        );
        self.initialized = true;
        true
    }

    pub fn base(&self) -> usize {
        self.base
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn stride(&self) -> u32 {
        self.stride
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
