use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::scheduler::Scheduler;

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Add {
    #[default]
    P0,
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

impl Add {
    pub fn execute(&mut self, scheduler: &mut Scheduler) {
        println!("execute: Add");
        match self {
            Add::P0 => self.p0(scheduler),
        }
    }

    pub fn p0(&mut self, scheduler: &mut Scheduler) {
        let reversed_data: Vec<u8> = scheduler.data_stack.iter().rev().cloned().collect();
        let (args, len): (Args, usize) =
            bincode::decode_from_slice(&reversed_data, bincode::config::standard()).unwrap();
        scheduler
            .data_stack
            .truncate(scheduler.data_stack.len() - len);

        println!("add args: {:?}", args);

        let res = Res {
            result: args.x + args.y,
        };

        scheduler.data_stack.extend(
            bincode::encode_to_vec(res, bincode::config::standard())
                .unwrap()
                .into_iter()
                .rev(),
        );
    }
}
