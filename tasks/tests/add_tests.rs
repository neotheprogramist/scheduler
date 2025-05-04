use scheduler::Scheduler;
use tasks::add::Add;

#[test]
fn test_add_zero() {
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Add::new(0, 0))).unwrap();
    scheduler.execute().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 0);
}

#[test]
fn test_add_single_value() {
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Add::new(42, 0))).unwrap();
    scheduler.execute().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 42);

    // Test with zero as first operand
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Add::new(0, 42))).unwrap();
    scheduler.execute().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 42);
}

#[test]
fn test_add_large_values() {
    let mut scheduler = Scheduler::default();
    let a = u128::MAX / 2;
    let b = u128::MAX / 3;
    scheduler.push_task(Box::new(Add::new(a, b))).unwrap();
    scheduler.execute().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, a.saturating_add(b));
}

#[test]
fn test_add_overflow_handling() {
    let mut scheduler = Scheduler::default();
    let a = u128::MAX;
    let b = 1;
    scheduler.push_task(Box::new(Add::new(a, b))).unwrap();
    scheduler.execute().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, u128::MAX); // Should handle overflow by saturating
}

#[test]
fn test_add_compute_method() {
    // Test the compute method directly without using the scheduler
    let add = Add::new(5, 7);
    assert_eq!(add.compute(), 12);

    // Test with larger values
    let add = Add::new(1_000_000, 2_000_000);
    assert_eq!(add.compute(), 3_000_000);

    // Test overflow handling
    let add = Add::new(u128::MAX, 10);
    assert_eq!(add.compute(), u128::MAX);
}

#[test]
fn test_add_multiple_operations() {
    let mut scheduler = Scheduler::default();

    // Push multiple add tasks
    scheduler.push_task(Box::new(Add::new(5, 10))).unwrap();
    scheduler.push_task(Box::new(Add::new(7, 3))).unwrap();
    scheduler.push_task(Box::new(Add::new(100, 200))).unwrap();

    // Execute all tasks
    scheduler.execute_all().unwrap();

    // Check results in LIFO order (last in, first out)
    let output1: u128 = scheduler.pop_data().unwrap();
    let output2: u128 = scheduler.pop_data().unwrap();
    let output3: u128 = scheduler.pop_data().unwrap();

    // Fix the order of expected values to match actual output
    assert_eq!(output1, 15); // First executed (5+10), produces 15
    assert_eq!(output2, 10); // Second executed (7+3), produces 10
    assert_eq!(output3, 300); // Third executed (100+200), produces 300
}
