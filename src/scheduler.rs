use serde::{Deserialize, Serialize};
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
}

/// The main scheduler that manages task execution.
///
/// The scheduler maintains two stacks:
/// 1. A call stack for pending tasks
/// 2. A data stack for passing data between tasks
#[derive(Default, Deserialize, Serialize)]
pub struct Scheduler {
    /// Stack of serialized tasks to be executed
    pub call_stack: Vec<u8>,

    /// Stack of data that tasks can read from and write to
    pub data_stack: Vec<u8>,
}

impl Scheduler {
    /// Create a new empty scheduler
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a task onto the call stack
    ///
    /// The task is serialized and its length is pushed after the task data
    pub fn push_call(&mut self, task: Box<dyn SchedulerTask>) -> Result<(), TaskError> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&task, &mut buffer)
            .map_err(|e| TaskError::SerializationError(e.to_string()))?;

        // Get the length before extending the call stack
        let len = buffer.len() as u32;

        // Push the task data
        self.call_stack.extend(buffer);

        // Push the length as a u32 (4 bytes)
        self.call_stack.extend_from_slice(&len.to_le_bytes());

        Ok(())
    }

    /// Extend the data stack with new data (in reverse order)
    ///
    /// The data is added in reverse order to enable FIFO behavior when reading
    pub fn extend_data(&mut self, data: &[u8]) {
        self.data_stack.extend(data.iter().rev());
    }

    /// Execute the next task on the call stack
    ///
    /// Returns an error if the call stack is empty or if there is an error
    /// during deserialization or execution
    pub fn execute(&mut self) -> Result<(), TaskError> {
        if self.call_stack.len() < 4 {
            return Err(TaskError::EmptyStack);
        }

        // Pop the length (last 4 bytes)
        let len_bytes = self.call_stack.split_off(self.call_stack.len() - 4);
        let len =
            u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;

        // Check if we have enough data
        if self.call_stack.len() < len {
            return Err(TaskError::InvalidTaskLength);
        }

        // Get the task data
        let task_data = self.call_stack.split_off(self.call_stack.len() - len);

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
        self.call_stack.is_empty()
    }

    /// Get the data stack in reversed order (for decoding)
    ///
    /// This is used by the codec module to read data in the correct order
    pub fn get_reversed_data(&self) -> Vec<u8> {
        self.data_stack.iter().rev().cloned().collect()
    }

    /// Truncate the data stack after reading
    ///
    /// This is used by the codec module to remove consumed data
    pub fn truncate_stack(&mut self, len: usize) {
        if len <= self.data_stack.len() {
            self.data_stack.truncate(self.data_stack.len() - len);
        }
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
    pub fn push_multiple_data(&mut self, data_items: Vec<Vec<u8>>) {
        for data in data_items {
            self.extend_data(&data);
        }
    }

    /// Clear all data from both stacks
    pub fn clear(&mut self) {
        self.call_stack.clear();
        self.data_stack.clear();
    }

    /// Get the current size of the data stack
    pub fn data_stack_size(&self) -> usize {
        self.data_stack.len()
    }

    /// Get the current size of the call stack
    pub fn call_stack_size(&self) -> usize {
        self.call_stack.len()
    }
}
