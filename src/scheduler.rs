use crate::stack::{BidirectionalStack, StackError};
use serde::{Serialize, de::DeserializeOwned};
use std::io::Cursor;
use thiserror::Error;

/// A trait for tasks that can be executed by the `Scheduler`.
///
/// Tasks implementing this trait can be added to the call stack and executed.
/// Each task should implement the `execute` method which will be called when
/// the task is popped from the call stack.
#[typetag::serde(tag = "type")]
pub trait SchedulerTask {
    /// Execute the task using the provided scheduler.
    ///
    /// This method is called when the task is popped from the call stack.
    /// It can read from and write to the data stack, and also push more tasks
    /// onto the call stack if needed.
    fn execute(&mut self, scheduler: &mut Scheduler);
}

/// Represents errors that can occur during task execution
#[derive(Debug, Error)]
pub enum TaskError {
    /// The stack is empty when trying to pop a value
    #[error("empty stack")]
    EmptyStack,

    /// Error during serialization of a task
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Error during deserialization of a task
    #[error("deserialization error: {0}")]
    DeserializationError(String),

    /// The task length is invalid (e.g., not enough bytes in the call stack)
    #[error("invalid task length")]
    InvalidTaskLength,

    /// Generic execution error
    #[error("execution error: {0}")]
    ExecutionError(String),

    /// Stack capacity error
    #[error("stack capacity error: {0}")]
    StackCapacityError(String),
}

// Convert from StackError to TaskError
impl From<StackError> for TaskError {
    fn from(err: StackError) -> Self {
        TaskError::StackCapacityError(err.to_string())
    }
}

/// The main scheduler that manages task execution.
///
/// The scheduler maintains two stacks:
/// 1. A call stack for pending tasks
/// 2. A data stack for passing data between tasks
#[derive(Debug, Default)]
pub struct Scheduler {
    /// Bidirectional stack for both call and data stacks
    stack: BidirectionalStack<4096>, // Choose an appropriate capacity
}

impl Scheduler {
    /// Push a task onto the call stack
    ///
    /// The task is serialized and its length is pushed after the task data
    pub fn push_call(&mut self, task: Box<dyn SchedulerTask>) -> Result<(), TaskError> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&task, &mut buffer)
            .map_err(|e| TaskError::SerializationError(e.to_string()))?;

        // Push the task data
        self.stack.push_back(&buffer)?;

        Ok(())
    }

    /// Extend the data stack with new data
    pub fn extend_data(&mut self, data: &[u8]) -> Result<(), TaskError> {
        self.stack.push_front(data)?;
        Ok(())
    }

    /// Execute the next task on the call stack
    ///
    /// Returns an error if the call stack is empty or if there is an error
    /// during deserialization or execution
    pub fn execute(&mut self) -> Result<(), TaskError> {
        // Pop the task data
        let task_data = self.stack.pop_back().ok_or(TaskError::InvalidTaskLength)?;

        // Deserialize the task
        let mut cursor = Cursor::new(&task_data);
        let mut task: Box<dyn SchedulerTask> = ciborium::de::from_reader(&mut cursor)
            .map_err(|e| TaskError::DeserializationError(e.to_string()))?;

        // Execute the task
        task.execute(self);
        Ok(())
    }

    /// Execute all tasks on the call stack until it's empty
    ///
    /// Returns an error if any task execution fails
    pub fn execute_all(&mut self) -> Result<(), TaskError> {
        while !self.is_empty() {
            self.execute()?;
        }
        Ok(())
    }

    /// Check if the call stack is empty
    pub fn is_empty(&self) -> bool {
        // Back stack (call stack) is empty when no more tasks can be popped
        self.stack.is_empty_back()
    }

    /// Schedule multiple tasks at once (in reverse order)
    ///
    /// Tasks are scheduled in reverse order so that they execute in the order provided
    pub fn schedule_tasks(&mut self, tasks: Vec<Box<dyn SchedulerTask>>) -> Result<(), TaskError> {
        for task in tasks.into_iter().rev() {
            self.push_call(task)?;
        }
        Ok(())
    }

    /// Push multiple byte vectors to the data stack (in reverse order)
    pub fn push_multiple_data(&mut self, data_items: Vec<Vec<u8>>) -> Result<(), TaskError> {
        for data in data_items {
            self.extend_data(&data)?;
        }
        Ok(())
    }

    /// Clear all data from both stacks
    pub fn clear(&mut self) {
        // Replace with a new empty stack
        self.stack = BidirectionalStack::default();
    }

    pub fn push_data<T: Serialize>(&mut self, data: &T) -> Result<(), TaskError> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(data, &mut buffer)
            .map_err(|e| TaskError::SerializationError(e.to_string()))?;

        // Push the task data
        self.stack.push_front(&buffer)?;
        Ok(())
    }

    pub fn pop_data<T: DeserializeOwned>(&mut self) -> Result<T, TaskError> {
        // Pop the task data
        let result_data = self.stack.pop_front().ok_or(TaskError::InvalidTaskLength)?;

        // Deserialize the task
        let mut cursor = Cursor::new(&result_data);
        let result: T = ciborium::de::from_reader(&mut cursor)
            .map_err(|e| TaskError::DeserializationError(e.to_string()))?;

        Ok(result)
    }
}
