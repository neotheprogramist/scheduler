use serde::{Deserialize, Serialize};

use crate::{
    codec::stack,
    scheduler::{Scheduler, SchedulerTask},
    tasks::{
        TaskArgs, TaskResult,
        add::{self, Add},
    },
};

/// A task that performs multiplication by repeated addition
///
/// This task demonstrates how to implement a complex operation
/// by breaking it down into multiple phases (P0, P1) and
/// using other tasks (Add) as building blocks.
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
            Mul::P0 => self.p0(scheduler),
            Mul::P1 => self.p1(scheduler),
        }
    }
}

/// The internal state for the Mul task
///
/// This state is passed between phases of the task
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub x: u8,       // First operand
    pub y: u8,       // Second operand
    pub result: u8,  // Running result
    pub counter: u8, // Number of additions completed
}

/// Arguments for the Mul task
#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    pub x: u8,
    pub y: u8,
}

impl TaskArgs for Args {}

/// Result of the Mul task
#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    pub result: u8,
}

impl TaskResult for Res {}

impl Mul {
    /// Create a new multiplication task
    pub fn new() -> Self {
        Self::default()
    }

    /// P0 phase: Initial multiplication phase
    ///
    /// This phase:
    /// 1. Decodes the arguments
    /// 2. Sets up the initial state
    /// 3. Determines if we need to schedule additional tasks
    /// 4. Either returns the result (if y is 0) or schedules more tasks
    pub fn p0(&mut self, scheduler: &mut Scheduler) {
        // Decode arguments from data stack
        let args: Args = match stack::try_decode(scheduler) {
            Ok(args) => args,
            Err(e) => {
                eprintln!("Error decoding multiplication arguments: {:?}", e);
                return;
            }
        };

        // Special case: if y is 0, result is 0
        if args.y == 0 {
            let res = Res { result: 0 };
            if let Err(e) = stack::try_encode(scheduler, res) {
                eprintln!("Error encoding multiplication result: {:?}", e);
            }
            return;
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
            let add_task: Box<dyn SchedulerTask> = Box::new(Add::new());
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::P1);

            // Schedule tasks (Add then Mul)
            if let Err(e) = scheduler.schedule_tasks(vec![add_task, mul_task]) {
                eprintln!("Error scheduling tasks: {:?}", e);
                return;
            }

            // Prepare arguments for the Add task
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and add args to the stack
            if let Err(e) = stack::try_encode(scheduler, state) {
                eprintln!("Error encoding state: {:?}", e);
                return;
            }

            if let Err(e) = stack::try_encode(scheduler, add_args) {
                eprintln!("Error encoding add args: {:?}", e);
            }
        } else {
            // Return final result (should be 0 since counter is 0)
            let res = Res {
                result: state.result,
            };
            if let Err(e) = stack::try_encode(scheduler, res) {
                eprintln!("Error encoding result: {:?}", e);
            }
        }
    }

    /// P1 phase: Subsequent multiplication phase
    ///
    /// This phase:
    /// 1. Decodes the Add result and previous state
    /// 2. Updates the state with the new result
    /// 3. Increments the counter
    /// 4. Either schedules more tasks or returns the final result
    pub fn p1(&mut self, scheduler: &mut Scheduler) {
        // Decode Add result and state
        let add_res: add::Res = match stack::try_decode(scheduler) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error decoding add result: {:?}", e);
                return;
            }
        };

        let mut state: State = match stack::try_decode(scheduler) {
            Ok(state) => state,
            Err(e) => {
                eprintln!("Error decoding state: {:?}", e);
                return;
            }
        };

        // Update state with the result from Add
        state.result = add_res.result;
        state.counter = state.counter.saturating_add(1);

        // If we haven't reached the target count, schedule more tasks
        if state.counter < state.y {
            // Create tasks
            let add_task: Box<dyn SchedulerTask> = Box::new(Add::new());
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::P1);

            // Schedule tasks (Add then Mul)
            if let Err(e) = scheduler.schedule_tasks(vec![add_task, mul_task]) {
                eprintln!("Error scheduling tasks: {:?}", e);
                return;
            }

            // Prepare arguments for the Add task
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and add args to the stack
            if let Err(e) = stack::try_encode(scheduler, state) {
                eprintln!("Error encoding state: {:?}", e);
                return;
            }

            if let Err(e) = stack::try_encode(scheduler, add_args) {
                eprintln!("Error encoding add args: {:?}", e);
            }
        } else {
            // Return final result
            let res = Res {
                result: state.result,
            };
            if let Err(e) = stack::try_encode(scheduler, res) {
                eprintln!("Error encoding result: {:?}", e);
            }
        }
    }
}
