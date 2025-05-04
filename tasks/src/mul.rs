use serde::{Deserialize, Serialize};

use scheduler::{Result, Scheduler, SchedulerTask};

use crate::add::Add;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Mul {
    pub x: u128,
    pub y: u128,
}

impl Mul {
    /// Creates a new Add task with the given operands.
    pub fn new(x: u128, y: u128) -> Self {
        Self { x, y }
    }
}

#[typetag::serde]
impl SchedulerTask for Mul {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        if self.y == 0 {
            scheduler.push_data(&0_u128)?;
            Ok(vec![])
        } else {
            Ok(vec![
                Box::new(Add::new(0, self.x)),
                Box::new(MulInternal::new(self.x, self.y, 0, 0)),
            ])
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct MulInternal {
    pub x: u128,
    pub y: u128,
    pub result: u128,
    pub counter: u128,
}

impl MulInternal {
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
impl SchedulerTask for MulInternal {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        let add_result: u128 = scheduler.pop_data()?;

        self.counter += 1;
        self.result = add_result;

        if self.counter < self.y {
            Ok(vec![Box::new(Add::new(self.result, self.x))])
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
    fn test_mul_normal() {
        let mut scheduler = Scheduler::default();

        // Create and push an Add task
        scheduler.push_task(Box::new(Mul::new(5, 11))).unwrap();

        // Execute task
        scheduler.execute_all().unwrap();

        // Verify the scheduler is empty of tasks
        assert!(scheduler.is_empty());

        // Check result
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, 55);
    }
}
