use crate::tpu::compiler::CompiledGraph;
pub fn execute(cg: &CompiledGraph) -> Result<usize, &'static str> {
    if cg.count == 0 {
        return Ok(0);
    }
    let mut bytes = 0usize;
    for idx in 0..cg.sizes.len() {
        bytes = bytes.saturating_add(cg.sizes[idx]);
    }
    Ok(bytes)
}

use crate::tpu::graph::Graph;

pub struct Executor {
    pub graph: Graph,
}

pub fn execute_graph(graph: &Graph) -> Result<usize, &'static str> {
    let device = crate::tpu::device::get().ok_or("no tpu device")?;
    let mut total = 0usize;
    for entry in &graph.tensors {
        if let Some(tensor) = entry.as_ref() {
            let ptr = tensor.as_ptr();
            if ptr.is_null() || tensor.len == 0 {
                continue;
            }
            let slice = unsafe { core::slice::from_raw_parts(ptr, tensor.len) };
            let sent = device.transfer(slice, 0, 64)?;
            total = total.saturating_add(sent);
        }
    }
    Ok(total)
}

impl Executor {
    pub fn new(g: Graph) -> Self {
        Executor { graph: g }
    }

    pub fn execute(&self) -> usize {
        execute_graph(&self.graph).unwrap_or(0)
    }
}
