use serde::{Deserialize, Serialize};
use thiserror::Error;

#[typetag::serde(tag = "type")]
pub trait SchedulerTask {
    fn execute(&mut self, scheduler: &mut Scheduler);
}

/// Represents errors that can occur during task execution
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("empty stack")]
    EmptyStack,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Scheduler {
    pub call_stack: Vec<Box<dyn SchedulerTask>>,
    pub data_stack: Vec<u8>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_call(&mut self, task: Box<dyn SchedulerTask>) {
        self.call_stack.push(task);
    }

    pub fn extend_data(&mut self, data: &[u8]) {
        self.data_stack.extend(data.into_iter().rev());
    }

    pub fn execute(&mut self) -> Result<(), TaskError> {
        println!("Data Stack: {:?}", self.data_stack);
        let mut task = self.call_stack.pop().ok_or(TaskError::EmptyStack)?;
        task.execute(self);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.call_stack.is_empty()
    }
}
