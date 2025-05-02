use serde::{Deserialize, Serialize};

use crate::{
    codec::stack,
    scheduler::{Scheduler, SchedulerTask},
    tasks::add::{self, Add},
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub x: u8,
    pub y: u8,
    pub result: u8,
    pub counter: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    pub result: u8,
}

impl Mul {
    pub fn p0(&mut self, scheduler: &mut Scheduler) {
        println!("execute: Mul p0");

        // Decode arguments from data stack
        let args: Args = stack::decode(scheduler);
        println!("x: {}", args.x);
        println!("y: {}", args.y);

        let state = State {
            x: args.x,
            y: args.y,
            result: 0,
            counter: 0,
        };

        if state.counter < state.y {
            // Create tasks
            let add_task: Box<dyn SchedulerTask> = Box::new(Add::default());
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::P1);

            // Schedule tasks (Add then Mul)
            scheduler.schedule_tasks(vec![add_task, mul_task]);

            // Prepare arguments and push to stack
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state first, then add args (they will be popped in reverse order)
            stack::encode(scheduler, state);
            stack::encode(scheduler, add_args);
        } else {
            // Return final result
            let res = Res {
                result: state.result,
            };
            stack::encode(scheduler, res);
        }
    }

    pub fn p1(&mut self, scheduler: &mut Scheduler) {
        println!("execute: Mul p1");

        // Decode Add result and state (in reverse order of pushing)
        let add_res: add::Res = stack::decode(scheduler);
        let mut state: State = stack::decode(scheduler);

        println!("add result: {:?}", add_res);
        println!("state: {:?}", state);

        // Update state
        state.result = add_res.result;
        state.counter += 1;

        if state.counter < state.y {
            // Create tasks
            let add_task: Box<dyn SchedulerTask> = Box::new(Add::default());
            let mul_task: Box<dyn SchedulerTask> = Box::new(Mul::P1);

            // Schedule tasks (Add then Mul)
            scheduler.schedule_tasks(vec![add_task, mul_task]);

            // Prepare arguments and push to stack
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state first, then add args (they will be popped in reverse order)
            stack::encode(scheduler, state);
            stack::encode(scheduler, add_args);
        } else {
            // Return final result
            let res = Res {
                result: state.result,
            };
            stack::encode(scheduler, res);
        }
    }
}
