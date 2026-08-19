use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Scheduler {
    current_task: AtomicUsize,
    task_count: AtomicUsize,
}

static SCHED: Scheduler = Scheduler {
    current_task: AtomicUsize::new(0),
    task_count: AtomicUsize::new(0),
};

impl Scheduler {
    pub fn register_task(&self) -> usize {
        self.task_count.fetch_add(1, Ordering::AcqRel)
    }

    pub fn current(&self) -> usize {
        self.current_task.load(Ordering::Acquire)
    }

    pub fn switch_to(&self, task_id: usize) {
        self.current_task.store(task_id, Ordering::Release);
        crate::sys::sched_yield();
    }

    pub fn task_count(&self) -> usize {
        self.task_count.load(Ordering::Acquire)
    }

    pub fn round_robin(&self) {
        let cur = self.current_task.load(Ordering::Acquire);
        let count = self.task_count.load(Ordering::Acquire);
        if count == 0 {
            return;
        }
        let next = (cur + 1) % count;
        self.current_task.store(next, Ordering::Release);
        crate::sys::sched_yield();
    }
}

pub fn global_scheduler() -> &'static Scheduler {
    &SCHED
}
