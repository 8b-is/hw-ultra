use crate::audio::detection::AudioController;

pub struct HdaDriver {
    base: usize,
    output_count: u8,
    input_count: u8,
    initialized: bool,
}

impl HdaDriver {
    pub fn probe(ctrl: &AudioController) -> Option<Self> {
        let base = if ctrl.pci {
            (ctrl.hda_bar & 0xFFFF_FFF0) as usize
        } else {
            ctrl.reg_base as usize
        };
        if base == 0 {
            return None;
        }
        Some(HdaDriver {
            base,
            output_count: 0,
            input_count: 0,
            initialized: false,
        })
    }

    pub fn init(&mut self) -> bool {
        crate::audio::hw::reset_controller(self.base);
        let gcap = crate::audio::hw::global_capabilities(self.base);
        self.output_count = crate::audio::hw::output_stream_count(gcap);
        self.input_count = crate::audio::hw::input_stream_count(gcap);
        crate::audio::hw::enable_interrupts(self.base);
        self.initialized = true;
        true
    }

    pub fn base(&self) -> usize {
        self.base
    }

    pub fn output_count(&self) -> u8 {
        self.output_count
    }

    pub fn input_count(&self) -> u8 {
        self.input_count
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
