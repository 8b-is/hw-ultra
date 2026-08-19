pub struct Link {
    pub speed_gen: u8,
    pub width: u8,
    pub active: bool,
}

impl Link {
    pub fn new(speed_gen: u8, width: u8) -> Self {
        Link {
            speed_gen,
            width,
            active: false,
        }
    }

    pub fn configure(&mut self, speed_gen: u8, width: u8) {
        self.speed_gen = speed_gen;
        self.width = width;
    }

    pub fn enable(&mut self) {
        self.active = true;
    }

    pub fn disable(&mut self) {
        self.active = false;
    }
}
