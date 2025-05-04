use serde::{Deserialize, Serialize};

use scheduler::{Result, Scheduler, SchedulerTask};

use crate::add::Add;

/// A task that calculates the nth Fibonacci number
///
/// This task implements a recursive algorithm using the scheduler:
/// - For n = 0 or n = 1, it directly returns the base case values
/// - For n > 1, it schedules subtasks to calculate F(n-1) and F(n-2),
///   followed by a task to add these results
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Fib {
    /// The index of the Fibonacci number to calculate
    pub n: u128,
}

impl Fib {
    /// Creates a new Fibonacci task with the given index.
    pub fn new(n: u128) -> Self {
        Self { n }
    }
}

#[typetag::serde]
impl SchedulerTask for Fib {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        match self.n {
            // Base case: F(0) = 0
            0 => {
                scheduler.push_data(&0_u128)?;
                Ok(vec![])
            }
            // Base case: F(1) = 1
            1 => {
                scheduler.push_data(&1_u128)?;
                Ok(vec![])
            }
            // Recursive case: F(n) = F(n-1) + F(n-2)
            n => Ok(vec![
                Box::new(Fib::new(n - 1)),
                Box::new(Fib::new(n - 2)),
                Box::new(FibCombiner::new()),
            ]),
        }
    }
}

/// A helper task that combines the results of two Fibonacci subtasks
///
/// This task takes the two most recent Fibonacci results from the data stack
/// and schedules an addition task followed by a result formatter
#[derive(Debug, Default, Serialize, Deserialize)]
struct FibCombiner {}

impl FibCombiner {
    pub fn new() -> Self {
        Self {}
    }
}

#[typetag::serde]
impl SchedulerTask for FibCombiner {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
        // Pop the two Fibonacci results (F(n-2) and F(n-1))
        let output1: u128 = scheduler.pop_data()?;
        let output2: u128 = scheduler.pop_data()?;

        // Schedule the addition task and the formatter
        Ok(vec![Box::new(Add::new(output1, output2))])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scheduler::Scheduler;

    #[test]
    fn test_fib_base_cases() {
        let mut scheduler = Scheduler::default();

        // Test F(0) = 0
        scheduler.push_task(Box::new(Fib::new(0))).unwrap();
        scheduler.execute_all().unwrap();
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, 0);

        // Test F(1) = 1
        scheduler.push_task(Box::new(Fib::new(1))).unwrap();
        scheduler.execute_all().unwrap();
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, 1);
    }

    #[test]
    fn test_fib_small_n() {
        let mut scheduler = Scheduler::default();

        // Test F(5) = 5
        scheduler.push_task(Box::new(Fib::new(5))).unwrap();
        scheduler.execute_all().unwrap();
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, 5);
    }

    #[test]
    fn test_fib_sequence() {
        // Test first few Fibonacci numbers
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

        for (n, expected_value) in expected.iter().enumerate() {
            let mut scheduler = Scheduler::default();
            scheduler.push_task(Box::new(Fib::new(n as u128))).unwrap();
            scheduler.execute_all().unwrap();
            let output: u128 = scheduler.pop_data().unwrap();
            assert_eq!(
                output, *expected_value,
                "F({}) should be {}",
                n, expected_value
            );
        }
    }
}
