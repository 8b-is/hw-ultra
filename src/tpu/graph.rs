use crate::tpu::tensor::Tensor;

pub struct Graph {
    pub tensors: [Option<Tensor>; 16],
    pub edges: [(u8, u8); 32],
    pub edge_count: usize,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            tensors: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
            edges: [(0, 0); 32],
            edge_count: 0,
        }
    }

    pub fn add_tensor(&mut self, idx: usize, t: Tensor) -> bool {
        if idx >= self.tensors.len() {
            return false;
        }
        self.tensors[idx] = Some(t);
        true
    }

    pub fn add_edge(&mut self, from: u8, to: u8) -> bool {
        if self.edge_count >= 32 {
            return false;
        }
        self.edges[self.edge_count] = (from, to);
        self.edge_count += 1;
        true
    }

    pub fn get_tensor(&self, idx: usize) -> Option<&Tensor> {
        self.tensors.get(idx).and_then(|o| o.as_ref())
    }

    pub fn tensor_count(&self) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < 16 {
            if self.tensors[i].is_some() {
                count += 1;
            }
            i += 1;
        }
        count
    }
}
