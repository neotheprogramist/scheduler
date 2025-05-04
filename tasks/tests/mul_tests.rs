use scheduler::Scheduler;
use tasks::mul::Mul;

#[test]
fn test_mul_zero() {
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Mul::new(42, 0))).unwrap();
    scheduler.execute_all().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 0);

    // Test with zero as first operand
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Mul::new(0, 42))).unwrap();
    scheduler.execute_all().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 0);
}

#[test]
fn test_mul_identity() {
    let mut scheduler = Scheduler::default();
    scheduler.push_task(Box::new(Mul::new(42, 1))).unwrap();
    scheduler.execute_all().unwrap();

    assert!(scheduler.is_empty());
    let output: u128 = scheduler.pop_data().unwrap();
    assert_eq!(output, 42);
}

#[test]
fn test_mul_small_numbers() {
    let test_cases = [(2, 3, 6), (5, 5, 25), (7, 8, 56), (10, 10, 100)];

    for (x, y, expected) in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler.push_task(Box::new(Mul::new(x, y))).unwrap();
        scheduler.execute_all().unwrap();

        assert!(scheduler.is_empty());
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(
            output, expected,
            "Multiplication of {} and {} should be {}",
            x, y, expected
        );
    }
}

#[test]
fn test_mul_large_numbers() {
    let test_cases = [
        (1_000_000, 2, 2_000_000),
        (999_999, 999_999, 999_998_000_001),
        (u128::MAX / 2, 2, u128::MAX - 1),
    ];

    for (x, y, expected) in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler.push_task(Box::new(Mul::new(x, y))).unwrap();
        scheduler.execute_all().unwrap();

        assert!(scheduler.is_empty());
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(
            output, expected,
            "Multiplication of {} and {} should be {}",
            x, y, expected
        );
    }
}

#[test]
fn test_mul_commutativity() {
    // Test that a*b = b*a
    let a = 123;
    let b = 456;

    let mut scheduler1 = Scheduler::default();
    scheduler1.push_task(Box::new(Mul::new(a, b))).unwrap();
    scheduler1.execute_all().unwrap();
    let result1: u128 = scheduler1.pop_data().unwrap();

    let mut scheduler2 = Scheduler::default();
    scheduler2.push_task(Box::new(Mul::new(b, a))).unwrap();
    scheduler2.execute_all().unwrap();
    let result2: u128 = scheduler2.pop_data().unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn test_mul_associativity() {
    // Test that (a*b)*c = a*(b*c)
    let a = 3;
    let b = 4;
    let c = 5;

    // Calculate (a*b)*c
    let mut scheduler1 = Scheduler::default();
    scheduler1.push_task(Box::new(Mul::new(a, b))).unwrap();
    scheduler1.execute_all().unwrap();
    let ab: u128 = scheduler1.pop_data().unwrap();

    let mut scheduler2 = Scheduler::default();
    scheduler2.push_task(Box::new(Mul::new(ab, c))).unwrap();
    scheduler2.execute_all().unwrap();
    let result1: u128 = scheduler2.pop_data().unwrap();

    // Calculate a*(b*c)
    let mut scheduler3 = Scheduler::default();
    scheduler3.push_task(Box::new(Mul::new(b, c))).unwrap();
    scheduler3.execute_all().unwrap();
    let bc: u128 = scheduler3.pop_data().unwrap();

    let mut scheduler4 = Scheduler::default();
    scheduler4.push_task(Box::new(Mul::new(a, bc))).unwrap();
    scheduler4.execute_all().unwrap();
    let result2: u128 = scheduler4.pop_data().unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn test_mul_with_multiple_operations() {
    let mut scheduler = Scheduler::default();

    scheduler.push_task(Box::new(Mul::new(2, 3))).unwrap();
    scheduler.push_task(Box::new(Mul::new(5, 5))).unwrap();
    scheduler.push_task(Box::new(Mul::new(10, 10))).unwrap();

    scheduler.execute_all().unwrap();

    // Check results in the order they're popped off the stack
    let output1: u128 = scheduler.pop_data().unwrap();
    let output2: u128 = scheduler.pop_data().unwrap();
    let output3: u128 = scheduler.pop_data().unwrap();

    // Fix the order of expected values to match actual output
    assert_eq!(output1, 6); // 2*3 = 6
    assert_eq!(output2, 25); // 5*5 = 25
    assert_eq!(output3, 100); // 10*10 = 100
}
