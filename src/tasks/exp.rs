//! Exponentiation task for the scheduler.
//!
//! This task demonstrates a complex operation that uses multiple phases and
//! depends on other tasks. It implements exponentiation (x^y) by repeated multiplication.

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    scheduler::{Scheduler, SchedulerTask},
    tasks::{
        TaskArgs, TaskResult,
        mul::{self, Mul},
    },
};

/// A task that performs exponentiation by repeated multiplication.
///
/// This task demonstrates how to implement a complex operation
/// by breaking it down into multiple phases (P0, P1) and
/// using other tasks (Mul) as building blocks.
///
/// # Examples
///
/// ```
/// use scheduler::{Scheduler, tasks::{Exp, ExpArgs}};
///
/// let mut scheduler = Scheduler::default();
/// let args = ExpArgs { x: 2, y: 3 };
/// scheduler.push_data(&args).unwrap();
/// scheduler.push_call(Box::new(Exp::new())).unwrap();
/// scheduler.execute_all().unwrap();
/// ```
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Exp {
    #[default]
    P0,
    P1,
}

#[typetag::serde]
impl SchedulerTask for Exp {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        match self {
            Exp::P0 => {
                if let Err(err) = self.p0(scheduler) {
                    eprintln!("Exp P0 phase failed: {:?}", err);
                }
            }
            Exp::P1 => {
                if let Err(err) = self.p1(scheduler) {
                    eprintln!("Exp P1 phase failed: {:?}", err);
                }
            }
        }
    }
}

/// The internal state for the Exp task.
///
/// This state is passed between phases of the task.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    /// Base value
    pub x: u8,
    /// Exponent
    pub y: u8,
    /// Running result
    pub result: u8,
    /// Number of multiplications completed
    pub counter: u8,
}

/// Arguments for the Exp task.
#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    /// Base value
    pub x: u8,
    /// Exponent (number of times to multiply x by itself)
    pub y: u8,
}

impl TaskArgs for Args {}

/// Result of the Exp task.
#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    /// The result of x^y
    pub result: u8,
}

impl TaskResult for Res {}

impl Exp {
    /// Create a new exponentiation task.
    ///
    /// # Examples
    ///
    /// ```
    /// use scheduler::tasks::Exp;
    ///
    /// let exp_task = Exp::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// P0 phase: Initial exponentiation phase.
    ///
    /// This phase:
    /// 1. Decodes the arguments
    /// 2. Sets up the initial state
    /// 3. Determines if we need to schedule additional tasks
    /// 4. Either returns the result (if y is 0 or 1) or schedules more tasks
    fn p0(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode arguments from data stack
        let args: Args = scheduler.pop_data()?;

        // Special case: if y is 0, result is 1 (x^0 = 1)
        if args.y == 0 {
            let res = Res { result: 1 };
            scheduler.push_data(&res)?;
            return Ok(());
        }

        // Special case: if y is 1, result is x (x^1 = x)
        if args.y == 1 {
            let res = Res { result: args.x };
            scheduler.push_data(&res)?;
            return Ok(());
        }

        // Set up initial state - start with x as the initial result
        let state = State {
            x: args.x,
            y: args.y,
            result: args.x,
            counter: 1, // We already have x^1
        };

        // If counter < y, need to do more multiplications
        if state.counter < state.y {
            // Create tasks
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::new());
            let exp_task: Box<dyn SchedulerTask> = Box::new(Exp::P1);

            // Schedule tasks (Mul then Exp)
            scheduler.schedule_tasks(vec![mul_task, exp_task])?;

            // Prepare arguments for the Mul task
            let mul_args = mul::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and mul args to the stack
            scheduler.push_data(&state)?;
            scheduler.push_data(&mul_args)?;
        } else {
            // Return final result (should be x since counter is 1)
            let res = Res {
                result: state.result,
            };
            scheduler.push_data(&res)?;
        }

        Ok(())
    }

    /// P1 phase: Subsequent exponentiation phase.
    ///
    /// This phase:
    /// 1. Decodes the Mul result and previous state
    /// 2. Updates the state with the new result
    /// 3. Increments the counter
    /// 4. Either schedules more tasks or returns the final result
    fn p1(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode Mul result and state
        let mul_res: mul::Res = scheduler.pop_data()?;
        let mut state: State = scheduler.pop_data()?;

        // Update state with the result from Mul
        state.result = mul_res.result;
        state.counter = state.counter.saturating_add(1);

        // If we haven't reached the target count, schedule more tasks
        if state.counter < state.y {
            // Create tasks
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::new());
            let exp_task: Box<dyn SchedulerTask> = Box::new(Exp::P1);

            // Schedule tasks (Mul then Exp)
            scheduler.schedule_tasks(vec![mul_task, exp_task])?;

            // Prepare arguments for the Mul task
            let mul_args = mul::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and mul args to the stack
            scheduler.push_data(&state)?;
            scheduler.push_data(&mul_args)?;
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
    fn test_exp_zero() {
        let mut scheduler = Scheduler::default();

        // Set up arguments with y=0
        let args = Args { x: 5, y: 0 };
        scheduler.push_data(&args).unwrap();

        // Execute task
        let mut task = Exp::new();
        task.execute(&mut scheduler);

        // Check result (should be 1)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 1);
    }

    #[test]
    fn test_exp_one() {
        let mut scheduler = Scheduler::default();

        // Set up arguments with y=1
        let args = Args { x: 5, y: 1 };
        scheduler.push_data(&args).unwrap();

        // Execute task
        let mut task = Exp::new();
        task.execute(&mut scheduler);

        // Check result (should be x)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 5);
    }

    #[test]
    fn test_exp_normal() {
        let mut scheduler = Scheduler::default();

        // Set up arguments
        let args = Args { x: 2, y: 3 };
        scheduler.push_data(&args).unwrap();

        // Push tasks
        scheduler.push_call(Box::new(Exp::new())).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result (2^3 = 8)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 8);
    }

    #[test]
    fn test_exp_overflow() {
        let mut scheduler = Scheduler::default();

        // Set up arguments that would overflow u8
        let args = Args { x: 4, y: 4 };
        scheduler.push_data(&args).unwrap();

        // Push tasks
        scheduler.push_call(Box::new(Exp::new())).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result (should be saturated at 255 since 4^4 = 256 > 255)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 255);
    }
}
