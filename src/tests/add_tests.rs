use crate::scheduler::Scheduler;
use crate::tasks::add::{AddTask, Begin, End};
use crate::tasks::{SchedulerTask, TaskError, TaskTrait};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_simple() {
        let mut scheduler = Scheduler::new();

        // Test simple addition: 3 + 5 = 8
        let a = 3;
        let b = 5;
        let expected = a + b;
        scheduler.push(SchedulerTask::Add(AddTask::Begin(Begin::new(a, b))));

        // The Begin task should transition to End task with the result
        assert!(scheduler.poll().is_ok());

        // Check the result
        if let Some(SchedulerTask::Add(add_task)) = scheduler.peek() {
            if let Some(result) = add_task.get_result() {
                assert_eq!(result, expected, "Expected result to be {}", expected);
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Add task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Now the scheduler should be empty
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_add_zero() {
        let mut scheduler = Scheduler::new();

        // Test addition with zero: 0 + 5 = 5
        let a = 0;
        let b = 5;
        let expected = a + b;
        scheduler.push(SchedulerTask::Add(AddTask::Begin(Begin::new(a, b))));

        // The Begin task should transition to End task with the result
        assert!(scheduler.poll().is_ok());

        // Check the result
        if let Some(SchedulerTask::Add(add_task)) = scheduler.peek() {
            if let Some(result) = add_task.get_result() {
                assert_eq!(result, expected, "Expected result to be {}", expected);
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Add task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Now the scheduler should be empty
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_add_zero_reversed() {
        let mut scheduler = Scheduler::new();

        // Test addition with zero (reversed operands): 5 + 0 = 5
        let a = 5;
        let b = 0;
        let expected = a + b;
        scheduler.push(SchedulerTask::Add(AddTask::Begin(Begin::new(a, b))));

        // The Begin task should transition to End task with the result
        assert!(scheduler.poll().is_ok());

        // Check the result
        if let Some(SchedulerTask::Add(add_task)) = scheduler.peek() {
            if let Some(result) = add_task.get_result() {
                assert_eq!(result, expected, "Expected result to be {}", expected);
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Add task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Now the scheduler should be empty
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_add_large_numbers() {
        let mut scheduler = Scheduler::new();

        // Test addition with larger numbers: 1000 + 2000 = 3000
        let a = 1000;
        let b = 2000;
        let expected = a + b;
        scheduler.push(SchedulerTask::Add(AddTask::Begin(Begin::new(a, b))));

        // The Begin task should transition to End task with the result
        assert!(scheduler.poll().is_ok());

        // Check the result
        if let Some(SchedulerTask::Add(add_task)) = scheduler.peek() {
            if let Some(result) = add_task.get_result() {
                assert_eq!(result, expected, "Expected result to be {}", expected);
            } else {
                panic!("Expected End task with a result");
            }
        } else {
            panic!("Expected Add task");
        }

        // The End task will return EmptyStack error when polled
        let result = scheduler.poll();
        assert!(result.is_err());
        assert!(matches!(result, Err(TaskError::EmptyStack)));

        // Now the scheduler should be empty
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_direct_task_execution() {
        // Test the addition algorithm directly using task polling

        // Test addition: 7 + 9 = 16
        let a = 7;
        let b = 9;
        let expected = a + b;

        // Start with Begin task
        let begin_task = AddTask::Begin(Begin::new(a, b));
        let begin_result = begin_task.poll().expect("Begin task should succeed");
        assert_eq!(begin_result.len(), 1, "Begin should produce 1 task");

        // Verify we got an End task with the correct result
        match &begin_result[0] {
            SchedulerTask::Add(AddTask::End(end)) => {
                assert_eq!(end.result(), expected, "Expected result to be {}", expected);
            }
            _ => panic!("Expected End task"),
        };

        // Create a new End task for testing since we can't clone the existing one
        let end_task = AddTask::End(End::new(expected));
        
        // Polling the End task should produce an error
        let result = end_task.poll();
        assert!(result.is_err(), "End task should produce an error");
        assert!(
            matches!(result, Err(TaskError::EmptyStack)),
            "Expected EmptyStack error"
        );
    }

    #[test]
    fn test_add_steps_count() {
        // Define test cases as (a, b, expected_steps)
        // For addition: Begin->End = 1 step
        let test_cases = vec![
            (3, 4, 1),  // Begin->End = 1
            (0, 0, 1),  // Begin->End = 1
            (100, 200, 1), // Begin->End = 1
        ];

        for (a, b, expected_steps) in test_cases {
            let mut scheduler = Scheduler::new();

            // Push the initial addition task
            scheduler.push(SchedulerTask::Add(AddTask::Begin(Begin::new(a, b))));

            // Run all the successful steps
            let mut steps_completed = 0;
            while steps_completed < expected_steps {
                let result = scheduler.poll();
                assert!(
                    result.is_ok(),
                    "Failed on step {} for {} + {} with {:?}",
                    steps_completed,
                    a,
                    b,
                    result
                );
                steps_completed += 1;
            }

            // Verify the result
            if let Some(SchedulerTask::Add(add_task)) = scheduler.peek() {
                if let Some(result) = add_task.get_result() {
                    assert_eq!(result, a + b, "Expected result to be {}", a + b);
                } else {
                    panic!("Expected End task with a result");
                }
            } else {
                panic!("Expected Add task");
            }

            // The next poll should error with EmptyStack
            let result = scheduler.poll();
            assert!(
                result.is_err(),
                "Expected error after {} steps for {} + {}",
                expected_steps,
                a,
                b
            );
            assert!(
                matches!(result, Err(TaskError::EmptyStack)),
                "Expected EmptyStack error for {} + {}",
                a,
                b
            );

            // Verify the scheduler is empty
            assert!(
                scheduler.is_empty(),
                "Scheduler not empty after final step for {} + {}",
                a,
                b
            );

            // Verify we used the expected number of steps
            assert_eq!(
                steps_completed, expected_steps,
                "Incorrect number of steps for {} + {}",
                a, b
            );
        }
    }
} 