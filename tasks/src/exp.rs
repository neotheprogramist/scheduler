use serde::{Deserialize, Serialize};

use scheduler::{Result, Scheduler, SchedulerTask};

use crate::mul::Mul;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Exp {
    pub x: u128,
    pub y: u128,
}

impl Exp {
    /// Creates a new Add task with the given operands.
    pub fn new(x: u128, y: u128) -> Self {
        Self { x, y }
    }
}

#[typetag::serde]
impl SchedulerTask for Exp {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        if self.y == 0 {
            scheduler.push_data(&1_u128)?;
            Ok(vec![])
        } else {
            Ok(vec![
                Box::new(Mul::new(1, self.x)),
                Box::new(ExpInternal::new(self.x, self.y, 0, 0)),
            ])
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ExpInternal {
    pub x: u128,
    pub y: u128,
    pub result: u128,
    pub counter: u128,
}

impl ExpInternal {
    pub fn new(x: u128, y: u128, result: u128, counter: u128) -> Self {
        Self {
            x,
            y,
            result,
            counter,
        }
    }
}
#[typetag::serde]
impl SchedulerTask for ExpInternal {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        let add_result: u128 = scheduler.pop_data()?;

        self.counter += 1;
        self.result = add_result;

        if self.counter < self.y {
            Ok(vec![Box::new(Mul::new(self.result, self.x))])
        } else {
            scheduler.push_data(&self.result)?;
            Ok(vec![])
        }
    }

    fn push_self(&mut self) -> bool {
        self.counter < self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scheduler::Scheduler;

    #[test]
    fn test_exp_normal() {
        let mut scheduler = Scheduler::default();

        // Create and push an Add task
        scheduler.push_task(Box::new(Exp::new(2, 5))).unwrap();

        // Execute task
        scheduler.execute_all().unwrap();

        // Verify the scheduler is empty of tasks
        assert!(scheduler.is_empty());

        // Check result
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, 32);
    }
}
