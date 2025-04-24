use thiserror::Error;

use crate::tasks::SchedulerTask;

/// Represents errors that can occur during task execution
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("empty stack")]
    EmptyStack,
}

#[derive(Default)]
pub struct Scheduler {
    pub call_stack: Vec<SchedulerTask>,
    pub data_stack: Vec<u8>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_call(&mut self, task: SchedulerTask) {
        self.call_stack.push(task);
    }

    pub fn extend_data(&mut self, data: &[u8]) {
        self.data_stack.extend(data.into_iter().rev());
    }

    pub fn poll(&mut self) -> Result<(), TaskError> {
        println!("Call Stack: {:?}", self.call_stack);
        println!("Data Stack: {:?}", self.data_stack);
        let mut task = self.call_stack.pop().ok_or(TaskError::EmptyStack)?;
        task.execute(self);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.call_stack.is_empty()
    }
}
