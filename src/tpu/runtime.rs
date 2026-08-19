use crate::tpu::executor;
use crate::tpu::graph::Graph;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct Runtime {
    ready: AtomicBool,
    submitted_bytes: AtomicUsize,
    completed_bytes: AtomicUsize,
    error_count: AtomicUsize,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            ready: AtomicBool::new(true),
            submitted_bytes: AtomicUsize::new(0),
            completed_bytes: AtomicUsize::new(0),
            error_count: AtomicUsize::new(0),
        }
    }

    pub fn run_graph(&self, graph: &Graph) -> Result<usize, &'static str> {
        if !self.ready.load(Ordering::Acquire) {
            return Err("runtime not ready");
        }
        let submitted = executor::execute_graph(graph)?;
        self.submitted_bytes.fetch_add(submitted, Ordering::AcqRel);
        self.completed_bytes.fetch_add(submitted, Ordering::AcqRel);
        Ok(submitted)
    }

    pub fn mark_error(&self) {
        self.error_count.fetch_add(1, Ordering::AcqRel);
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (
            self.submitted_bytes.load(Ordering::Acquire),
            self.completed_bytes.load(Ordering::Acquire),
            self.error_count.load(Ordering::Acquire),
        )
    }
}

pub fn init() -> Runtime {
    Runtime::new()
}
