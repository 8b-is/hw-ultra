use super::detection::AudioController;

pub struct Device {
    pub info: AudioController,
    pub mmio_base: Option<usize>,
}
