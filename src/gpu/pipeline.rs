use crate::gpu::Shader;

pub struct Pipeline {
    shaders: [Option<Shader>; 4],
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            shaders: [None, None, None, None],
        }
    }

    pub fn attach_shader(&mut self, idx: usize, shader: Shader) {
        if idx < self.shaders.len() {
            self.shaders[idx] = Some(shader);
        }
    }

    pub fn validate(&self) -> bool {
        for s in &self.shaders {
            if s.is_some() {
                return true;
            }
        }
        false
    }
}
