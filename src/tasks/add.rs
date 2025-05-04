//! Addition task for the scheduler.
//!
//! This task demonstrates a simple operation that adds two numbers.
//! It reads two arguments from the data stack, adds them, and pushes the result back.

use serde::{Deserialize, Serialize};

use crate::{
    Result,
    scheduler::{Scheduler, SchedulerTask},
};

/// A task that performs addition of two numbers.
///
/// This is a simple task that adds two numbers and returns the result.
/// It demonstrates the basic pattern for implementing a task:
/// 1. Define the task state (if needed)
/// 2. Define the arguments and result types
/// 3. Implement the SchedulerTask trait
///
/// # Examples
///
/// ```
/// use scheduler::{Scheduler, tasks::{Add, AddArgs}};
///
/// let mut scheduler = Scheduler::default();
/// let args = AddArgs { x: 5, y: 10 };
/// scheduler.push_data(&args).unwrap();
/// scheduler.push_call(Box::new(Add::new())).unwrap();
/// scheduler.execute().unwrap();
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Add {
    /// First operand
    pub x: u8,
    /// Second operand
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub result: u8,
}

#[typetag::serde]
impl SchedulerTask for Add {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        // Calculate result (using saturating_add to prevent overflow)
        let output: Output = Output {
            result: self.x.saturating_add(self.y),
        };

        // Push result to data stack
        scheduler.push_data(&output)?;

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;

    #[test]
    fn test_add_normal() {
        let mut scheduler = Scheduler::default();

        scheduler.push_task(Box::new(Add { x: 5, y: 10 })).unwrap();

        // Execute task
        scheduler.execute().unwrap();

        assert!(scheduler.is_empty());
        // Check result
        let output: Output = scheduler.pop_data().unwrap();
        assert_eq!(output.result, 15);
    }
}
