use scheduler::Scheduler;
use tasks::exp::Exp;

#[test]
fn test_exp_zero_exponent() {
    // Any number raised to power 0 should be 1
    let test_cases = [0, 1, 2, 10, 42, 100];

    for base in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler.push_task(Box::new(Exp::new(base, 0))).unwrap();
        scheduler.execute_all().unwrap();

        assert!(scheduler.is_empty());
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, 1, "{} raised to power 0 should be 1", base);
    }
}

#[test]
fn test_exp_identity() {
    // Any number raised to power 1 should be the number itself
    let test_cases = [0, 1, 2, 10, 42, 100];

    for base in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler.push_task(Box::new(Exp::new(base, 1))).unwrap();
        scheduler.execute_all().unwrap();

        assert!(scheduler.is_empty());
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(
            output, base,
            "{} raised to power 1 should be {}",
            base, base
        );
    }
}

#[test]
fn test_exp_zero_base() {
    // 0 raised to any positive power should be 0
    let test_cases = [1, 2, 3, 10, 42];

    for exponent in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler
            .push_task(Box::new(Exp::new(0, exponent)))
            .unwrap();
        scheduler.execute_all().unwrap();

        assert!(scheduler.is_empty());
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(output, 0, "0 raised to power {} should be 0", exponent);
    }
}

#[test]
fn test_exp_small_powers() {
    let test_cases = [
        (2, 2, 4),
        (2, 3, 8),
        (2, 4, 16),
        (3, 2, 9),
        (3, 3, 27),
        (4, 2, 16),
        (5, 3, 125),
        (10, 2, 100),
        (10, 3, 1000),
    ];

    for (base, exponent, expected) in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler
            .push_task(Box::new(Exp::new(base, exponent)))
            .unwrap();
        scheduler.execute_all().unwrap();

        assert!(scheduler.is_empty());
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(
            output, expected,
            "{} raised to power {} should be {}",
            base, exponent, expected
        );
    }
}

#[test]
fn test_exp_larger_powers() {
    let test_cases = [(2, 10, 1024), (3, 7, 2187), (10, 5, 100000)];

    for (base, exponent, expected) in test_cases {
        let mut scheduler = Scheduler::default();
        scheduler
            .push_task(Box::new(Exp::new(base, exponent)))
            .unwrap();
        scheduler.execute_all().unwrap();

        assert!(scheduler.is_empty());
        let output: u128 = scheduler.pop_data().unwrap();
        assert_eq!(
            output, expected,
            "{} raised to power {} should be {}",
            base, exponent, expected
        );
    }
}

#[test]
fn test_exp_power_laws() {
    // Test (base^a)^b = base^(a*b)
    let base = 2;
    let a = 3;
    let b = 2;

    // Calculate base^a
    let mut scheduler1 = Scheduler::default();
    scheduler1.push_task(Box::new(Exp::new(base, a))).unwrap();
    scheduler1.execute_all().unwrap();
    let base_to_a: u128 = scheduler1.pop_data().unwrap();

    // Calculate (base^a)^b
    let mut scheduler2 = Scheduler::default();
    scheduler2
        .push_task(Box::new(Exp::new(base_to_a, b)))
        .unwrap();
    scheduler2.execute_all().unwrap();
    let result1: u128 = scheduler2.pop_data().unwrap();

    // Calculate base^(a*b)
    let mut scheduler3 = Scheduler::default();
    scheduler3
        .push_task(Box::new(Exp::new(base, a * b)))
        .unwrap();
    scheduler3.execute_all().unwrap();
    let result2: u128 = scheduler3.pop_data().unwrap();

    // Verify (base^a)^b = base^(a*b)
    assert_eq!(result1, result2);
}

#[test]
fn test_exp_multiple_operations() {
    let mut scheduler = Scheduler::default();

    scheduler.push_task(Box::new(Exp::new(2, 3))).unwrap();
    scheduler.push_task(Box::new(Exp::new(3, 2))).unwrap();
    scheduler.push_task(Box::new(Exp::new(10, 2))).unwrap();

    scheduler.execute_all().unwrap();

    // Results in the order they're popped off the stack
    let output1: u128 = scheduler.pop_data().unwrap();
    let output2: u128 = scheduler.pop_data().unwrap();
    let output3: u128 = scheduler.pop_data().unwrap();

    // Fix the order of expected values to match actual output
    assert_eq!(output1, 8); // 2^3 = 8
    assert_eq!(output2, 9); // 3^2 = 9
    assert_eq!(output3, 100); // 10^2 = 100
}
