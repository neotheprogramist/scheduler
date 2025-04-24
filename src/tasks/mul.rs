use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::scheduler::Scheduler;

use super::{SchedulerTask, add};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Mul {
    #[default]
    P0,
    P1,
}

#[derive(Debug, Decode, Encode)]
pub struct State {
    pub x: u8,
    pub y: u8,
    pub result: u8,
    pub counter: u8,
}

#[derive(Debug, Decode, Encode)]
pub struct Args {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Decode, Encode)]
pub struct Res {
    pub result: u8,
}

impl Mul {
    pub fn execute(&mut self, scheduler: &mut Scheduler) {
        match self {
            Mul::P0 => self.p0(scheduler),
            Mul::P1 => self.p1(scheduler),
        }
    }

    pub fn p0(&mut self, scheduler: &mut Scheduler) {
        println!("execute: Mul p0");
        let reversed_data: Vec<u8> = scheduler.data_stack.iter().rev().cloned().collect();
        let (args, len): (Args, usize) =
            bincode::decode_from_slice(&reversed_data, bincode::config::standard()).unwrap();
        scheduler
            .data_stack
            .truncate(scheduler.data_stack.len() - len);

        println!("x: {}", args.x);
        println!("y: {}", args.y);
        let state = State {
            x: args.x,
            y: args.y,
            result: 0,
            counter: 0,
        };

        if state.counter < state.y {
            scheduler.call_stack.extend(
                vec![
                    SchedulerTask::Add(add::Add::P0),
                    SchedulerTask::Mul(Mul::P1),
                ]
                .into_iter()
                .rev(),
            );
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };
            scheduler.data_stack.extend(
                vec![
                    bincode::encode_to_vec(add_args, bincode::config::standard()).unwrap(),
                    bincode::encode_to_vec(state, bincode::config::standard()).unwrap(),
                ]
                .iter()
                .flatten()
                .rev(),
            );
        } else {
            let res = Res {
                result: state.result,
            };
            scheduler.data_stack.extend(
                vec![bincode::encode_to_vec(res, bincode::config::standard()).unwrap()]
                    .iter()
                    .flatten()
                    .rev(),
            );
        }
    }

    pub fn p1(&mut self, scheduler: &mut Scheduler) {
        println!("execute: Mul p1");
        let reversed_data: Vec<u8> = scheduler.data_stack.iter().rev().cloned().collect();
        let (add_res, len): (add::Res, usize) =
            bincode::decode_from_slice(&reversed_data, bincode::config::standard()).unwrap();
        scheduler
            .data_stack
            .truncate(scheduler.data_stack.len() - len);

        let reversed_data: Vec<u8> = scheduler.data_stack.iter().rev().cloned().collect();
        let (mut state, len): (State, usize) =
            bincode::decode_from_slice(&reversed_data, bincode::config::standard()).unwrap();
        scheduler
            .data_stack
            .truncate(scheduler.data_stack.len() - len);

        println!("add result: {:?}", add_res);
        println!("state: {:?}", state);

        state.result = add_res.result;
        state.counter += 1;
        if state.counter < state.y {
            scheduler.call_stack.extend(
                vec![
                    SchedulerTask::Add(add::Add::P0),
                    SchedulerTask::Mul(Mul::P1),
                ]
                .into_iter()
                .rev(),
            );
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };
            scheduler.data_stack.extend(
                vec![
                    bincode::encode_to_vec(add_args, bincode::config::standard()).unwrap(),
                    bincode::encode_to_vec(state, bincode::config::standard()).unwrap(),
                ]
                .iter()
                .flatten()
                .rev(),
            );
        } else {
            let res = Res {
                result: state.result,
            };
            scheduler.data_stack.extend(
                vec![bincode::encode_to_vec(res, bincode::config::standard()).unwrap()]
                    .iter()
                    .flatten()
                    .rev(),
            );
        }
    }
}
