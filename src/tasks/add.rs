use serde::{Deserialize, Serialize};

use crate::codec::stack;
use crate::scheduler::{Scheduler, SchedulerTask};
use crate::tasks::{TaskArgs, TaskResult};

/// A task that performs addition of two numbers
///
/// This is a simple task that adds two numbers and returns the result.
/// It demonstrates the basic pattern for implementing a task:
/// 1. Define the task state (if needed)
/// 2. Define the arguments and result types
/// 3. Implement the SchedulerTask trait
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Add {
    #[default]
    P0,
}

#[typetag::serde]
impl SchedulerTask for Add {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        match self {
            Add::P0 => self.p0(scheduler),
        }
    }
}

/// Arguments for the Add task
#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    pub x: u8,
    pub y: u8,
}

impl TaskArgs for Args {}

/// Result of the Add task
#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    pub result: u8,
}

impl TaskResult for Res {}

impl Add {
    /// Create a new Add task
    pub fn new() -> Self {
        Self::default()
    }

    /// P0 phase: Main execution phase
    ///
    /// 1. Decode the arguments from the data stack
    /// 2. Perform the addition
    /// 3. Encode the result back to the data stack
    pub fn p0(&mut self, scheduler: &mut Scheduler) {
        // Decode arguments from data stack
        let args: Args = match stack::try_decode(scheduler) {
            Ok(args) => args,
            Err(e) => {
                eprintln!("Error decoding arguments: {:?}", e);
                return;
            }
        };

        // Calculate result
        let res = Res {
            result: args.x.saturating_add(args.y), // Use saturating_add to prevent overflow
        };

        // Push result to data stack
        if let Err(e) = stack::try_encode(scheduler, res) {
            eprintln!("Error encoding result: {:?}", e);
        }
    }
}
