use crate::scheduler::Scheduler;
use crate::tasks::mul::Begin;
use crate::tasks::mul::MulTask;
use crate::tasks::{SchedulerTask, TaskError, TaskTrait};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_zero() {
        let mut scheduler = Scheduler::new();

        // Test multiplication by zero
        scheduler.push(SchedulerTask::Mul(MulTask::Begin(Begin::new(0, 5))));

        // The Begin task should transition to End task with result 0
        assert!(scheduler.poll().is_ok());
        
        // Verify the result is 0
        if let Some(SchedulerTask::Mul(mul_task)) = scheduler.peek() {
            if let Some(result) = mul_task.get_result() {
                assert_eq!(result, 0, "Expected result to be 0");
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Mul task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Now the scheduler should be empty
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_mul_zero_reversed() {
        let mut scheduler = Scheduler::new();

        // Test multiplication by zero (reversed operands)
        scheduler.push(SchedulerTask::Mul(MulTask::Begin(Begin::new(5, 0))));

        // The Begin task should transition to End task with result 0
        assert!(scheduler.poll().is_ok());
        
        // Verify the result is 0
        if let Some(SchedulerTask::Mul(mul_task)) = scheduler.peek() {
            if let Some(result) = mul_task.get_result() {
                assert_eq!(result, 0, "Expected result to be 0");
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Mul task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Now the scheduler should be empty
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_mul_simple() {
        let mut scheduler = Scheduler::new();

        // Test simple multiplication: 3 * 2 = 6
        let a = 3;
        let b = 2;
        let expected = a * b;
        scheduler.push(SchedulerTask::Mul(MulTask::Begin(Begin::new(a, b))));

        // First poll: Begin -> AddPhase
        assert!(scheduler.poll().is_ok());
        assert!(!scheduler.is_empty());

        // Poll until we've done all the AddPhase iterations (counter 0 to b-1)
        // For 3 * 2, we need 2 more polls for AddPhase
        for _ in 0..b {
            assert!(scheduler.poll().is_ok());
            assert!(!scheduler.is_empty());
        }

        // Final poll: AddPhase -> End
        assert!(scheduler.poll().is_ok());
        
        // Check the result
        if let Some(SchedulerTask::Mul(mul_task)) = scheduler.peek() {
            if let Some(result) = mul_task.get_result() {
                assert_eq!(result, expected, "Expected result to be {}", expected);
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Mul task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Now the scheduler should be empty
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_mul_large_numbers() {
        let mut scheduler = Scheduler::new();

        // Test multiplication with larger numbers: 10 * 5 = 50
        let a = 10;
        let b = 5;
        let expected = a * b;
        scheduler.push(SchedulerTask::Mul(MulTask::Begin(Begin::new(a, b))));

        // Step 1: Begin -> AddPhase (counter=0)
        assert!(scheduler.poll().is_ok());

        // Steps 2-6: AddPhase iterations
        // We need b steps for AddPhase counter to go from 0 to b
        for step in 0..b {
            assert!(scheduler.poll().is_ok(), "AddPhase step {} failed", step);
        }

        // Final poll: AddPhase (counter=b) -> End
        assert!(scheduler.poll().is_ok());
        
        // Check the result
        if let Some(SchedulerTask::Mul(mul_task)) = scheduler.peek() {
            if let Some(result) = mul_task.get_result() {
                assert_eq!(result, expected, "Expected result to be {}", expected);
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Mul task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Scheduler should be empty now
        assert!(scheduler.is_empty(), "Scheduler not empty after final step");
    }

    #[test]
    fn test_verify_mul_steps() {
        // Define test cases as (a, b, expected_steps)
        // where expected_steps is the total number of successful steps before the error
        // For multiplication by zero: Begin->End = 1 step
        // For a*b where a,b > 0: Begin->AddPhase + b AddPhase polls + AddPhase->End = b+2 steps
        let test_cases = vec![
            (3, 4, 6),     // Begin->AddPhase(1) + AddPhase iterations(4) + AddPhase->End(1) = 6
            (7, 0, 1),     // Begin->End = 1
            (0, 9, 1),     // Begin->End = 1
            (5, 5, 7),     // Begin->AddPhase(1) + AddPhase iterations(5) + AddPhase->End(1) = 7
            (1, 100, 102), // Begin->AddPhase(1) + AddPhase iterations(100) + AddPhase->End(1) = 102
            (100, 1, 3),   // Begin->AddPhase(1) + AddPhase iterations(1) + AddPhase->End(1) = 3
        ];

        for (a, b, expected_steps) in test_cases {
            let mut scheduler = Scheduler::new();

            // Push the initial multiplication task
            scheduler.push(SchedulerTask::Mul(MulTask::Begin(Begin::new(a, b))));

            // Run all the successful steps
            let mut steps_completed = 0;
            while steps_completed < expected_steps {
                let result = scheduler.poll();
                assert!(
                    result.is_ok(),
                    "Failed on step {} for {} * {} with {:?}",
                    steps_completed,
                    a,
                    b,
                    result
                );
                steps_completed += 1;
            }

            // The next poll should error with EmptyStack
            let result = scheduler.poll();
            assert!(
                result.is_err(),
                "Expected error after {} steps for {} * {}",
                expected_steps,
                a,
                b
            );
            assert!(
                matches!(result, Err(TaskError::EmptyStack)),
                "Expected EmptyStack error for {} * {}",
                a,
                b
            );

            // Verify the scheduler is empty
            assert!(
                scheduler.is_empty(),
                "Scheduler not empty after final step for {} * {}",
                a,
                b
            );

            // Verify we used the expected number of steps
            assert_eq!(
                steps_completed, expected_steps,
                "Incorrect number of steps for {} * {}",
                a, b
            );
        }
    }

    #[test]
    fn test_direct_task_execution() {
        // Test the multiplication algorithm directly using task polling
        // Since we can't clone SchedulerTask, we'll create new tasks at each step

        // Test multiplication: 4 * 3 = 12
        let a = 4;
        let b = 3;
        let expected = a * b;

        // Step 1: Start with Begin task
        let begin_task = MulTask::Begin(Begin::new(a, b));
        let begin_result = begin_task.poll().expect("Begin task should succeed");
        assert_eq!(begin_result.len(), 1, "Begin should produce 1 task");

        // Verify we got an AddPhase task
        match &begin_result[0] {
            SchedulerTask::Mul(MulTask::AddPhase(_)) => {
                // This is correct, we have an AddPhase state
            }
            _ => panic!("Expected AddPhase task"),
        };

        // Step 2+: Poll through AddPhase tasks
        let mut current_tasks = begin_result;

        // Poll through the AddPhase iterations
        // The AddPhase::poll() method checks counter < b, so we need b iterations
        // plus one final poll where counter = b to transition to End
        for i in 0..=b {
            // Get the task
            let task = match current_tasks.pop() {
                Some(task) => task,
                None => panic!("No task available at step {}", i),
            };

            // Poll the task
            match task {
                SchedulerTask::Mul(mul_task) => {
                    current_tasks = mul_task.poll().expect("AddPhase task should succeed");
                }
                _ => panic!("Expected Mul task at step {}", i),
            };

            assert_eq!(current_tasks.len(), 1, "AddPhase should produce 1 task");

            // After b iterations, we should have an End task
            if i == b {
                match &current_tasks[0] {
                    SchedulerTask::Mul(MulTask::End(end)) => {
                        // This is correct, we have the End state
                        assert_eq!(end.result(), expected, "Expected result to be {}", expected);
                    }
                    other => panic!(
                        "Expected End task after {} AddPhase steps, got {:?}",
                        b, other
                    ),
                };
            }
        }

        // Polling the End task should produce an error
        let end_task = match current_tasks.pop().unwrap() {
            SchedulerTask::Mul(task) => task,
            _ => panic!("Expected Mul task"),
        };

        let result = end_task.poll();
        assert!(result.is_err(), "End task should produce an error");
        assert!(
            matches!(result, Err(TaskError::EmptyStack)),
            "Expected EmptyStack error"
        );
    }
}
