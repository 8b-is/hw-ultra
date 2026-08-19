use crate::tpu::graph::Graph;

pub struct CompiledGraph {
    pub sizes: [usize; 16],
    pub exec_order: [u8; 16],
    pub count: usize,
    pub total_memory: usize,
}

impl Default for CompiledGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl CompiledGraph {
    pub fn new() -> Self {
        CompiledGraph {
            sizes: [0; 16],
            exec_order: [0; 16],
            count: 0,
            total_memory: 0,
        }
    }
}

pub fn compile(g: &Graph) -> CompiledGraph {
    let mut out = CompiledGraph::new();
    let mut total_mem = 0usize;
    let mut idx = 0;
    while idx < g.tensors.len() {
        if let Some(t) = &g.tensors[idx] {
            out.sizes[out.count] = t.len;
            out.exec_order[out.count] = idx as u8;
            total_mem += t.len;
            out.count += 1;
        }
        idx += 1;
    }
    out.total_memory = total_mem;
    out
}

pub fn compile_graph(g: &Graph) -> CompiledGraph {
    compile(g)
}
