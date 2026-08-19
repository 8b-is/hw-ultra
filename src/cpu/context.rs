pub struct Context {
    regs: [usize; 16],
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Context { regs: [0usize; 16] }
    }

    pub fn set_reg(&mut self, idx: usize, val: usize) {
        if idx < self.regs.len() {
            self.regs[idx] = val;
        }
    }

    pub fn get_reg(&self, idx: usize) -> Option<usize> {
        if idx < self.regs.len() {
            Some(self.regs[idx])
        } else {
            None
        }
    }

    pub fn save(&mut self, src: &[usize]) {
        let n = core::cmp::min(self.regs.len(), src.len());
        self.regs[..n].copy_from_slice(&src[..n]);
    }

    pub fn restore(&self, dst: &mut [usize]) {
        let n = core::cmp::min(self.regs.len(), dst.len());
        dst[..n].copy_from_slice(&self.regs[..n]);
    }
}
