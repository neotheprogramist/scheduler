//! Multiplication task for the scheduler.
//!
//! This task demonstrates a complex operation that uses multiple phases and
//! depends on other tasks. It implements multiplication by repeated addition.

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    scheduler::{Scheduler, SchedulerTask},
    tasks::add::{self, Add},
};

/// A task that performs multiplication by repeated addition.
///
/// This task demonstrates how to implement a complex operation
/// by breaking it down into multiple phases (P0, P1) and
/// using other tasks (Add) as building blocks.
///
/// # Examples
///
/// ```
/// use scheduler::{Scheduler, tasks::{Mul, MulArgs}};
///
/// let mut scheduler = Scheduler::default();
/// let args = MulArgs { x: 5, y: 3 };
/// scheduler.push_data(&args).unwrap();
/// scheduler.push_call(Box::new(Mul::new())).unwrap();
/// scheduler.execute_all().unwrap();
/// ```
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Mul {
    #[default]
    P0,
    P1,
}

#[typetag::serde]
impl SchedulerTask for Mul {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        match self {
            Mul::P0 => {
                if let Err(err) = self.p0(scheduler) {
                    eprintln!("Mul P0 phase failed: {:?}", err);
                }
            }
            Mul::P1 => {
                if let Err(err) = self.p1(scheduler) {
                    eprintln!("Mul P1 phase failed: {:?}", err);
                }
            }
        }
    }
}

/// The internal state for the Mul task.
///
/// This state is passed between phases of the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    /// First operand
    pub x: u8,
    /// Second operand
    pub y: u8,
    /// Running result
    pub result: u8,
    /// Number of additions completed
    pub counter: u8,
}

/// Arguments for the Mul task.
#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    /// First operand
    pub x: u8,
    /// Second operand (number of times to add x)
    pub y: u8,
}

impl TaskArgs for Args {}

/// Result of the Mul task.
#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    /// The product of x and y
    pub result: u8,
}

impl TaskResult for Res {}

impl Mul {
    /// Create a new multiplication task.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::tasks::Mul;
    ///
    /// let mul_task = Mul::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// P0 phase: Initial multiplication phase.
    ///
    /// This phase:
    /// 1. Decodes the arguments
    /// 2. Sets up the initial state
    /// 3. Determines if we need to schedule additional tasks
    /// 4. Either returns the result (if y is 0) or schedules more tasks
    fn p0(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode arguments from data stack
        let args: Args = scheduler.pop_data()?;

        // Special case: if y is 0, result is 0
        if args.y == 0 {
            let res = Res { result: 0 };
            scheduler.push_data(&res)?;
            return Ok(());
        }

        // Set up initial state
        let state = State {
            x: args.x,
            y: args.y,
            result: 0,
            counter: 0,
        };

        // If counter < y, need to do more additions
        if state.counter < state.y {
            // Create tasks
            let add_task: Box<dyn SchedulerTask> = Box::new(Add::default());
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::P1);

            // Schedule tasks (Add then Mul)
            scheduler.schedule_tasks(vec![add_task, mul_task])?;

            // Prepare arguments for the Add task
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and add args to the stack
            scheduler.push_data(&state)?;
            scheduler.push_data(&add_args)?;
        } else {
            // Return final result (should be 0 since counter is 0)
            let res = Res {
                result: state.result,
            };
            scheduler.push_data(&res)?;
        }

        Ok(())
    }

    /// P1 phase: Subsequent multiplication phase.
    ///
    /// This phase:
    /// 1. Decodes the Add result and previous state
    /// 2. Updates the state with the new result
    /// 3. Increments the counter
    /// 4. Either schedules more tasks or returns the final result
    fn p1(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode Add result and state
        let add_res: add::Res = scheduler.pop_data()?;
        let mut state: State = scheduler.pop_data()?;

        // Update state with the result from Add
        state.result = add_res.result;
        state.counter = state.counter.saturating_add(1);

        // If we haven't reached the target count, schedule more tasks
        if state.counter < state.y {
            // Create tasks
            let add_task: Box<dyn SchedulerTask> = Box::new(Add::default());
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::P1);

            // Schedule tasks (Add then Mul)
            scheduler.schedule_tasks(vec![add_task, mul_task])?;

            // Prepare arguments for the Add task
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and add args to the stack
            scheduler.push_data(&state)?;
            scheduler.push_data(&add_args)?;
        } else {
            // Return final result
            let res = Res {
                result: state.result,
            };
            scheduler.push_data(&res)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;

    #[test]
    fn test_mul_zero() {
        let mut scheduler = Scheduler::default();

        // Set up arguments with y=0
        let args = Args { x: 5, y: 0 };
        scheduler.push_data(&args).unwrap();

        // Execute task
        let mut task = Mul::new();
        task.execute(&mut scheduler);

        // Check result (should be 0)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 0);
    }

    #[test]
    fn test_mul_normal() {
        let mut scheduler = Scheduler::default();

        // Set up arguments
        let args = Args { x: 5, y: 3 };
        scheduler.push_data(&args).unwrap();

        // Push tasks
        scheduler.push_call(Box::new(Mul::new())).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 15);
    }

    #[test]
    fn test_mul_overflow() {
        let mut scheduler = Scheduler::default();

        // Set up arguments that would overflow u8
        let args = Args { x: 100, y: 3 };
        scheduler.push_data(&args).unwrap();

        // Push tasks
        scheduler.push_call(Box::new(Mul::new())).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result (should be saturated at 255)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 255);
    }
}
