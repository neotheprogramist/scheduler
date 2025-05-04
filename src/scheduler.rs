use crate::error::{Error, Result};
use crate::stack::BidirectionalStack;
use serde::{Serialize, de::DeserializeOwned};
use std::io::Cursor;

/// Trait for tasks that can be executed by the scheduler.
///
/// Implementations must be serializable and deserializable.
#[typetag::serde(tag = "type")]
pub trait SchedulerTask: Send + Sync {
    /// Execute the task and return new tasks to be pushed onto the scheduler.
    ///
    /// The scheduler is provided for pushing/popping data during execution.
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>>;
}

/// Scheduler that manages task execution and data flow.
///
/// Uses a bidirectional stack to store tasks and data.
#[derive(Debug, Default)]
pub struct Scheduler {
    /// The stack used for storing tasks and data.
    /// Tasks are stored at the back, data at the front.
    stack: BidirectionalStack<4096>,
}

impl Scheduler {
    /// Creates a new empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a task onto the scheduler's task stack.
    pub fn push_task(&mut self, task: Box<dyn SchedulerTask>) -> Result<()> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&task, &mut buffer).map_err(Error::Serialization)?;

        self.stack
            .push_back(&buffer)
            .map_err(Error::StackCapacity)?;

        Ok(())
    }

    /// Pushes data onto the scheduler's data stack.
    pub fn push_data<T: Serialize>(&mut self, data: &T) -> Result<()> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(data, &mut buffer).map_err(Error::Serialization)?;

        self.stack
            .push_front(&buffer)
            .map_err(Error::StackCapacity)?;

        Ok(())
    }

    /// Pops a task from the scheduler's task stack.
    pub fn pop_task(&mut self) -> Result<Box<dyn SchedulerTask>> {
        let data = self.stack.pop_back()?;

        let mut cursor = Cursor::new(&data);
        let result = ciborium::de::from_reader(&mut cursor).map_err(Error::Deserialization)?;

        Ok(result)
    }

    /// Pops data from the scheduler's data stack.
    pub fn pop_data<T: DeserializeOwned>(&mut self) -> Result<T> {
        let data = self.stack.pop_front()?;

        let mut cursor = Cursor::new(&data);
        let result = ciborium::de::from_reader(&mut cursor).map_err(Error::Deserialization)?;

        Ok(result)
    }

    /// Executes the next task in the scheduler.
    ///
    /// Returns an error if there are no tasks or if execution fails.
    pub fn execute(&mut self) -> Result<()> {
        let mut task = self.pop_task()?;

        let tasks = task
            .execute(self)
            .map_err(|e| Error::Execution(format!("Task execution failed: {}", e)))?;

        // Push tasks in reverse order so they execute in the order they were returned
        for task in tasks.into_iter().rev() {
            self.push_task(task)?;
        }

        Ok(())
    }

    /// Executes all tasks in the scheduler until there are no more.
    pub fn execute_all(&mut self) -> Result<()> {
        while !self.is_empty() {
            self.execute()?;
        }
        Ok(())
    }

    /// Returns true if there are no tasks in the scheduler.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty_back()
    }

    /// Returns true if there is no data in the scheduler.
    pub fn is_empty_data(&self) -> bool {
        self.stack.is_empty_front()
    }

    /// Clears all tasks and data from the scheduler.
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
