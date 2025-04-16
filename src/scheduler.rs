use crate::tasks::{SchedulerTask, TaskError, TaskTrait};

/// A scheduler that manages arithmetic tasks
pub struct Scheduler {
    stack: Vec<SchedulerTask>,
}

impl std::fmt::Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scheduler")
            .field("stack_size", &self.stack.len())
            .finish()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// Create a new empty scheduler
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a new task onto the scheduler's stack
    pub fn push(&mut self, task: SchedulerTask) {
        self.stack.push(task);
    }

    /// Execute the next task in the stack
    /// Returns Ok(()) if successful, or an error if the stack is empty or task execution fails
    pub fn poll(&mut self) -> Result<(), TaskError> {
        let task = self.stack.pop().ok_or(TaskError::EmptyStack)?;
        let new_tasks = task.poll()?;
        self.stack.extend(new_tasks.into_iter().rev());
        Ok(())
    }

    /// Check if there are any tasks remaining in the stack
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
