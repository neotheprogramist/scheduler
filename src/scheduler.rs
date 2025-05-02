use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::io::Cursor;

#[typetag::serde(tag = "type")]
pub trait SchedulerTask {
    fn execute(&mut self, scheduler: &mut Scheduler);
}

/// Represents errors that can occur during task execution
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("empty stack")]
    EmptyStack,
    #[error("serialization error: {0}")]
    SerializationError(String),
    #[error("deserialization error: {0}")]
    DeserializationError(String),
    #[error("invalid task length")]
    InvalidTaskLength,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Scheduler {
    pub call_stack: Vec<u8>,
    pub data_stack: Vec<u8>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_call(&mut self, task: Box<dyn SchedulerTask>) {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&task, &mut buffer)
            .map_err(|e| TaskError::SerializationError(e.to_string()))
            .unwrap();
        
        // Get the length before extending the call stack
        let len = buffer.len() as u32;
        
        // Push the task data
        self.call_stack.extend(buffer);
        
        // Push the length as a u32 (4 bytes)
        self.call_stack.extend_from_slice(&len.to_le_bytes());
    }

    pub fn extend_data(&mut self, data: &[u8]) {
        self.data_stack.extend(data.iter().rev());
    }

    pub fn execute(&mut self) -> Result<(), TaskError> {
        println!("Data Stack: {:?}", self.data_stack);
        if self.call_stack.len() < 4 {
            return Err(TaskError::EmptyStack);
        }

        // Pop the length (last 4 bytes)
        let len_bytes = self.call_stack.split_off(self.call_stack.len() - 4);
        let len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;

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
        
        task.execute(self);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.call_stack.is_empty()
    }

    // Helper method to get reversed data for decoding
    pub fn get_reversed_data(&self) -> Vec<u8> {
        self.data_stack.iter().rev().cloned().collect()
    }

    // Helper method to truncate stack after reading
    pub fn truncate_stack(&mut self, len: usize) {
        self.data_stack.truncate(self.data_stack.len() - len);
    }

    // Schedule multiple tasks at once (in reverse order)
    pub fn schedule_tasks(&mut self, tasks: Vec<Box<dyn SchedulerTask>>) {
        for task in tasks.into_iter().rev() {
            self.push_call(task);
        }
    }

    // Push multiple byte vectors to the data stack (in reverse order)
    pub fn push_multiple_data(&mut self, data_items: Vec<Vec<u8>>) {
        self.data_stack.extend(data_items.iter().flatten().rev());
    }
}
