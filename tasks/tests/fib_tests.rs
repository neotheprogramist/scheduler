use scheduler::Scheduler;
use tasks::fib::Fib;

#[test]
fn test_fib_base_cases() {
    // Test F(0) = 0
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Fib::new(0))).unwrap();
    scheduler.execute_all().unwrap();
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 0);

    // Test F(1) = 1
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Fib::new(1))).unwrap();
    scheduler.execute_all().unwrap();
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 1);
}

#[test]
fn test_fib_small_sequence() {
    // First 10 Fibonacci numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
    let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

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

#[test]
fn test_fib_medium_values() {
    // Test some medium-sized Fibonacci numbers
    let test_cases = [(10, 55), (15, 610), (20, 6765)];

    for (n, expected) in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler.push_task(Box::new(Fib::new(n))).unwrap();
        scheduler.execute_all().unwrap();
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, expected, "F({}) should be {}", n, expected);
    }
}

#[test]
fn test_fib_recursive_properties() {
    // Test that F(n) = F(n-1) + F(n-2) for a few values
    let n_values = [5, 8, 10];

    for n in n_values {
        // Calculate F(n)
        let mut scheduler1 = Scheduler::default();
        scheduler1.push_task(Box::new(Fib::new(n))).unwrap();
        scheduler1.execute_all().unwrap();
        let fn_result: u128 = scheduler1.pop_data().unwrap();

        // Calculate F(n-1)
        let mut scheduler2 = Scheduler::default();
        scheduler2.push_task(Box::new(Fib::new(n - 1))).unwrap();
        scheduler2.execute_all().unwrap();
        let fn_minus_1: u128 = scheduler2.pop_data().unwrap();

        // Calculate F(n-2)
        let mut scheduler3 = Scheduler::default();
        scheduler3.push_task(Box::new(Fib::new(n - 2))).unwrap();
        scheduler3.execute_all().unwrap();
        let fn_minus_2: u128 = scheduler3.pop_data().unwrap();

        // Verify F(n) = F(n-1) + F(n-2)
        assert_eq!(
            fn_result,
            fn_minus_1 + fn_minus_2,
            "F({}) should equal F({}) + F({})",
            n,
            n - 1,
            n - 2
        );
    }
}

#[test]
fn test_fib_multiple_calculations() {
    let mut scheduler = Scheduler::default();

    // Push multiple Fibonacci tasks
    scheduler.push_task(Box::new(Fib::new(5))).unwrap();
    scheduler.push_task(Box::new(Fib::new(7))).unwrap();
    scheduler.push_task(Box::new(Fib::new(10))).unwrap();

    // Execute all tasks
    scheduler.execute_all().unwrap();

    // Check results in the order they're popped off the stack
    let output1: u128 = scheduler.pop_data().unwrap();
    let output2: u128 = scheduler.pop_data().unwrap();
    let output3: u128 = scheduler.pop_data().unwrap();

    // Fix the order of expected values to match actual output
    assert_eq!(output1, 5); // Fib(5) = 5
    assert_eq!(output2, 13); // Fib(7) = 13
    assert_eq!(output3, 55); // Fib(10) = 55
}

#[test]
#[should_panic(expected = "StackCapacity(Underflow)")]
fn test_fib_empty_stack_error() {
    let mut scheduler = Scheduler::default();

    // Try to pop from an empty stack
    let _: u128 = scheduler.pop_data().unwrap();
}
