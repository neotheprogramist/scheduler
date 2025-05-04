//! Scheduler implementation for task execution.
//!
//! The scheduler manages a bidirectional stack where:
//! - Front stack: Used as a data stack for passing data between tasks
//! - Back stack: Used as a call stack for pending tasks
//!
//! Tasks are serialized and pushed to the call stack, then executed in sequence.
//! Each task can read from and write to the data stack, as well as push more tasks
//! onto the call stack.

use crate::error::{Error, Result};
use crate::stack::BidirectionalStack;
use serde::{Serialize, de::DeserializeOwned};
use std::io::Cursor;

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
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>>;
}

/// The main scheduler that manages task execution.
///
/// The scheduler maintains two stacks:
/// 1. A call stack for pending tasks
/// 2. A data stack for passing data between tasks
///
/// # Examples
///
/// ```
/// use scheduler::{Scheduler, tasks::{add, Add, AddArgs}};
///
/// // Create a new scheduler
/// let mut scheduler = Scheduler::default();
///
/// // Push arguments to the data stack
/// let args = AddArgs { x: 5, y: 10 };
/// scheduler.push_data(&args).unwrap();
///
/// // Schedule an addition task
/// scheduler.push_call(Box::new(Add::new())).unwrap();
///
/// // Execute the task
/// scheduler.execute().unwrap();
///
/// // Retrieve the result
/// let result: add::Res = scheduler.pop_data().unwrap();
/// assert_eq!(result.result, 15);
/// ```
#[derive(Debug, Default)]
pub struct Scheduler {
    /// Bidirectional stack for both call and data stacks
    stack: BidirectionalStack<4096>, // 4KB stack by default
}

impl Scheduler {
    /// Push a task onto the call stack.
    ///
    /// The task is serialized and pushed to the back of the stack.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to push onto the call stack
    ///
    /// # Returns
    ///
    /// * `Ok(())` if successful
    /// * `Error` if serialization fails or the stack is full
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::{Scheduler, tasks::Add};
    ///
    /// let mut scheduler = Scheduler::default();
    /// scheduler.push_call(Box::new(Add::new())).unwrap();
    /// ```
    pub fn push_task(&mut self, task: Box<dyn SchedulerTask>) -> Result<()> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&task, &mut buffer)?;

        // Push the task data
        self.stack.push_back(&buffer)?;

        Ok(())
    }

    /// Push data onto the data stack.
    ///
    /// The data is serialized and pushed to the front of the stack.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to push onto the data stack
    ///
    /// # Returns
    ///
    /// * `Ok(())` if successful
    /// * `Error` if serialization fails or the stack is full
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::{Scheduler, tasks::AddArgs};
    ///
    /// let mut scheduler = Scheduler::default();
    /// let args = AddArgs { x: 5, y: 10 };
    /// scheduler.push_data(&args).unwrap();
    /// ```
    pub fn push_data<T: Serialize>(&mut self, data: &T) -> Result<()> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(data, &mut buffer)?;

        // Push the data
        self.stack.push_front(&buffer)?;
        Ok(())
    }

    pub fn pop_task(&mut self) -> Result<Box<dyn SchedulerTask>> {
        // Pop the data
        let data = self.stack.pop_back().ok_or(Error::EmptyStack)?;

        // Deserialize the data
        let mut cursor = Cursor::new(&data);
        let result = ciborium::de::from_reader(&mut cursor)?;

        Ok(result)
    }

    /// Pop data from the data stack.
    ///
    /// The data is deserialized and returned.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` with the deserialized data if successful
    /// * `Error` if deserialization fails or the stack is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::{Scheduler, tasks::{Add, AddArgs, AddResult}};
    ///
    /// let mut scheduler = Scheduler::default();
    ///
    /// // Push arguments, execute task, and retrieve result
    /// let args = AddArgs { x: 5, y: 10 };
    /// scheduler.push_data(&args).unwrap();
    /// scheduler.push_call(Box::new(Add::new())).unwrap();
    /// scheduler.execute().unwrap();
    ///
    /// let result: AddResult = scheduler.pop_data().unwrap();
    /// assert_eq!(result.result, 15);
    /// ```
    pub fn pop_data<T: DeserializeOwned>(&mut self) -> Result<T> {
        // Pop the data
        let data = self.stack.pop_front().ok_or(Error::EmptyStack)?;

        // Deserialize the data
        let mut cursor = Cursor::new(&data);
        let result = ciborium::de::from_reader(&mut cursor)?;

        Ok(result)
    }

    /// Execute the next task on the call stack.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if successful
    /// * `Error` if deserialization fails or the stack is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::{Scheduler, tasks::{Add, AddArgs}};
    ///
    /// let mut scheduler = Scheduler::default();
    ///
    /// // Push arguments and task
    /// let args = AddArgs { x: 5, y: 10 };
    /// scheduler.push_data(&args).unwrap();
    /// scheduler.push_call(Box::new(Add::new())).unwrap();
    ///
    /// // Execute the task
    /// scheduler.execute().unwrap();
    /// ```
    pub fn execute(&mut self) -> Result<()> {
        let mut task = self.pop_task()?;

        // Execute the task
        let tasks = task.execute(self)?;

        for task in tasks.into_iter().rev() {
            self.push_task(task)?;
        }

        Ok(())
    }

    /// Execute all tasks on the call stack until it's empty.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if all tasks executed successfully
    /// * `Error` if any task execution fails
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::{Scheduler, tasks::{Add, AddArgs}};
    ///
    /// let mut scheduler = Scheduler::default();
    ///
    /// // Schedule multiple tasks
    /// let args1 = AddArgs { x: 5, y: 10 };
    /// let args2 = AddArgs { x: 20, y: 30 };
    ///
    /// scheduler.push_data(&args1).unwrap();
    /// scheduler.push_call(Box::new(Add::new())).unwrap();
    /// scheduler.push_data(&args2).unwrap();
    /// scheduler.push_call(Box::new(Add::new())).unwrap();
    ///
    /// // Execute all tasks
    /// scheduler.execute_all().unwrap();
    /// ```
    pub fn execute_all(&mut self) -> Result<()> {
        while !self.is_empty() {
            self.execute()?;
        }
        Ok(())
    }

    /// Check if the call stack is empty.
    ///
    /// # Returns
    ///
    /// * `true` if the call stack is empty
    /// * `false` if there are tasks pending execution
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty_back()
    }
}
