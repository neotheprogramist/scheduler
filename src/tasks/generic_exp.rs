//! Generic Exponentiation task using the PhasedTask trait.
//!
//! This task demonstrates how to use the generic PhasedTask trait to implement
//! a complex operation with multiple phases.

use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    scheduler::{Scheduler, SchedulerTask},
    tasks::{
        TaskArgs, TaskResult,
        mul::{self, Mul},
        traits::PhasedTask,
    },
};

/// Arguments for the ExponentTask.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Args {
    /// Base value
    pub x: u8,
    /// Exponent
    pub y: u8,
}

impl TaskArgs for Args {}

/// Result of the ExponentTask.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Res {
    /// The result of x^y
    pub result: u8,
}

impl TaskResult for Res {}

/// Internal state for the ExponentTask.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Phases of the exponentiation task.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ExponentPhase {
    #[default]
    /// Initial phase
    Initial,
    /// Subsequent phase
    Next,
}

/// A task that performs exponentiation by repeated multiplication.
///
/// This implementation uses the generic PhasedTask trait to simplify
/// the task's implementation and reduce boilerplate.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExponentTask {
    phase: ExponentPhase,
}

impl ExponentTask {
    /// Create a new exponentiation task.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a task for the next phase.
    pub fn next() -> Self {
        Self {
            phase: ExponentPhase::Next,
        }
    }
}

#[typetag::serde]
impl SchedulerTask for ExponentTask {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        self.execute_phase(scheduler);
    }
}

impl PhasedTask for ExponentTask {
    type Args = Args;
    type Result = Res;
    type State = State;

    fn initial_phase(&mut self, scheduler: &mut Scheduler, args: Self::Args) -> Result<()> {
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
            let mul_task = Box::new(Mul::new());
            let exp_task = self.next_phase();

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

    fn subsequent_phase(
        &mut self,
        scheduler: &mut Scheduler,
        state: &mut Self::State,
    ) -> Result<()> {
        // Get multiplication result
        let mul_res: mul::Res = scheduler.pop_data()?;

        // Update state with the result from Mul
        state.result = mul_res.result;
        state.counter = state.counter.saturating_add(1);

        // If we need more multiplications, schedule another one
        if !self.is_complete(state) {
            // Create tasks
            let mul_task = Box::new(Mul::new());
            let exp_task = self.next_phase();

            // Schedule tasks (Mul then Exp)
            scheduler.schedule_tasks(vec![mul_task, exp_task])?;

            // Prepare arguments for the Mul task
            let mul_args = mul::Args {
                x: state.result,
                y: state.x,
            };

            // Push mul args to the stack
            scheduler.push_data(&mul_args)?;
        }

        Ok(())
    }

    fn next_phase(&self) -> Box<dyn SchedulerTask> {
        Box::new(ExponentTask::next())
    }

    fn is_initial_phase(&self) -> bool {
        matches!(self.phase, ExponentPhase::Initial)
    }

    fn produce_result(&self, state: &Self::State) -> Self::Result {
        Res {
            result: state.result,
        }
    }

    fn is_complete(&self, state: &Self::State) -> bool {
        state.counter >= state.y
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
        let mut task = ExponentTask::new();
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
        let mut task = ExponentTask::new();
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
        scheduler.push_call(Box::new(ExponentTask::new())).unwrap();

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
        scheduler.push_call(Box::new(ExponentTask::new())).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result (should be saturated at 255 since 4^4 = 256 > 255)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 255);
    }
}
