use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Virtio {
    pub device_id: u32,
    pub features: usize,
}

static VIRTIO_COUNT: AtomicUsize = AtomicUsize::new(0);

impl Virtio {
    pub fn new(device_id: u32) -> Self {
        Virtio {
            device_id,
            features: 0,
        }
    }

    pub fn negotiate_features(&mut self, host_features: usize) -> usize {
        let agreed = host_features & self.features;
        self.features = agreed;
        agreed
    }

    pub fn init(&mut self) -> bool {
        true
    }
}

pub fn detect_virtio_devices() -> usize {
    VIRTIO_COUNT.load(Ordering::Acquire)
}

pub fn virtio_device_count() -> usize {
    let c = VIRTIO_COUNT.load(Ordering::Acquire);
    if c == 0 {
        detect_virtio_devices()
    } else {
        c
    }
}
