use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::scheduler::{Scheduler, SchedulerTask};
use crate::codec::stack;

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
    pub fn p0(&mut self, scheduler: &mut Scheduler) {
        // Decode arguments from data stack
        let args: Args = stack::decode(scheduler);
        println!("add args: {:?}", args);

        // Calculate result
        let res = Res {
            result: args.x + args.y,
        };
        
        // Push result to data stack
        stack::encode(scheduler, res);
    }
}
