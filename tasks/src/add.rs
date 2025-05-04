use serde::{Deserialize, Serialize};

use scheduler::{Result, Scheduler, SchedulerTask};

/// A task that adds two numbers together.
///
/// This is a simple example task that demonstrates the scheduler's capabilities.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Add {
    /// First operand for addition
    pub x: u8,
    /// Second operand for addition
    pub y: u8,
}

/// Output of the Add task containing the sum.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Output {
    /// The result of adding x and y
    pub result: u8,
}

impl Add {
    /// Creates a new Add task with the given operands.
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Performs the addition and returns the result.
    ///
    /// Uses saturating addition to prevent overflow.
    pub fn compute(&self) -> Output {
        Output {
            result: self.x.saturating_add(self.y),
        }
    }
}

#[typetag::serde]
impl SchedulerTask for Add {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        // Compute the result
        let output = self.compute();

        // Push the result to the data stack
        scheduler.push_data(&output)?;

        // No follow-up tasks
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scheduler::Scheduler;

    #[test]
    fn test_add_normal() {
        let mut scheduler = Scheduler::default();

        // Create and push an Add task
        scheduler.push_task(Box::new(Add::new(5, 10))).unwrap();

        // Execute task
        scheduler.execute().unwrap();

        // Verify the scheduler is empty of tasks
        assert!(scheduler.is_empty());

        // Check result
        let output: Output = scheduler.pop_data().unwrap();
        assert_eq!(output.result, 15);
    }

    #[test]
    fn test_add_overflow_prevention() {
        // Create task with values that would overflow u8
        let add = Add::new(250, 10);

        // Compute result directly
        let output = add.compute();

        // Check that it used saturating_add
        assert_eq!(output.result, 255);
    }
}
