# CPU Scheduler

Simple round-robin task scheduler with atomic state tracking.

## Structure

```rust
pub struct Scheduler {
    current_task: AtomicUsize,
    task_count: AtomicUsize,
}
```

## Functions

```rust
pub fn global_scheduler() -> &'static Scheduler
pub fn register_task(&self) -> usize        // Returns new task ID
pub fn current(&self) -> usize              // Current task ID
pub fn switch_to(&self, task_id: usize)     // Switch active task
pub fn task_count(&self) -> usize           // Total registered tasks
pub fn round_robin(&self)                   // Advance to next task
```

## Behavior

- `register_task()` atomically increments the task counter and returns the new ID
- `round_robin()` advances `current_task` by 1, wrapping at `task_count`
- `switch_to()` sets `current_task` directly
- The global scheduler is a `Once<Scheduler>` singleton
