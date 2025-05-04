use serde::{Deserialize, Serialize};

use crate::{
    Result,
    scheduler::{Scheduler, SchedulerTask},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Add {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub result: u8,
}

#[typetag::serde]
impl SchedulerTask for Add {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        let output: Output = Output {
            result: self.x.saturating_add(self.y),
        };

        scheduler.push_data(&output)?;

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;

    #[test]
    fn test_add_normal() {
        let mut scheduler = Scheduler::default();

        scheduler.push_task(Box::new(Add { x: 5, y: 10 })).unwrap();

        // Execute task
        scheduler.execute().unwrap();

        assert!(scheduler.is_empty());
        // Check result
        let output: Output = scheduler.pop_data().unwrap();
        assert_eq!(output.result, 15);
    }
}
