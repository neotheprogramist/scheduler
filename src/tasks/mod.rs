use add::AddTask;
use mul::MulTask;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod add;
pub mod mul;

/// Trait defining the behavior of an arithmetic task
pub trait TaskTrait {
    /// Execute one step of the task and return any new tasks that need to be scheduled
    fn poll(self: Self) -> Result<Vec<SchedulerTask>, TaskError>;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SchedulerTask {
    Add(AddTask),
    Mul(MulTask),
}

impl TaskTrait for SchedulerTask {
    fn poll(self: Self) -> Result<Vec<SchedulerTask>, TaskError> {
        match self {
            SchedulerTask::Add(task) => task.poll(),
            SchedulerTask::Mul(task) => task.poll(),
        }
    }
}

/// Represents errors that can occur during task execution
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("empty stack")]
    EmptyStack,
}
