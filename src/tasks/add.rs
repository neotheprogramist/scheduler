//! Addition task for the scheduler.
//!
//! This task demonstrates a simple operation that adds two numbers.
//! It reads two arguments from the data stack, adds them, and pushes the result back.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::scheduler::{Scheduler, SchedulerTask};
use crate::tasks::{TaskArgs, TaskResult};

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
pub enum Add {
    #[default]
    P0,
}

#[typetag::serde]
impl SchedulerTask for Add {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        match self {
            Add::P0 => {
                if let Err(err) = self.p0(scheduler) {
                    eprintln!("Addition task failed: {:?}", err);
                }
            }
        }
    }
}

/// Arguments for the Add task.
#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    /// First operand
    pub x: u8,
    /// Second operand
    pub y: u8,
}

impl TaskArgs for Args {}

/// Result of the Add task.
#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    /// The sum of x and y
    pub result: u8,
}

impl TaskResult for Res {}

impl Add {
    /// Create a new Add task.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::tasks::Add;
    ///
    /// let add_task = Add::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// P0 phase: Main execution phase.
    ///
    /// 1. Decode the arguments from the data stack
    /// 2. Perform the addition
    /// 3. Encode the result back to the data stack
    fn p0(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode arguments from data stack
        let args: Args = scheduler.pop_data()?;

        // Calculate result (using saturating_add to prevent overflow)
        let res = Res {
            result: args.x.saturating_add(args.y),
        };

        // Push result to data stack
        scheduler.push_data(&res)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;

    #[test]
    fn test_add_normal() {
        let mut scheduler = Scheduler::default();

        // Set up arguments
        let args = Args { x: 5, y: 10 };
        scheduler.push_data(&args).unwrap();

        // Execute task
        let mut task = Add::new();
        task.execute(&mut scheduler);

        // Check result
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 15);
    }

    #[test]
    fn test_add_overflow() {
        let mut scheduler = Scheduler::default();

        // Set up arguments that would overflow u8
        let args = Args { x: 250, y: 10 };
        scheduler.push_data(&args).unwrap();

        // Execute task
        let mut task = Add::new();
        task.execute(&mut scheduler);

        // Check result (should be saturated at 255)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 255);
    }
}
