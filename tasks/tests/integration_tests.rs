// Import the task modules for testing
use scheduler::Scheduler;
use tasks::add::Add;
use tasks::exp::Exp;
use tasks::fib::Fib;
use tasks::mul::Mul;

// Include the module tests
mod add_tests;
mod exp_tests;
mod fib_tests;
mod mul_tests;

#[test]
fn test_task_composition() {
    // Test combining multiple task types together
    let mut scheduler = Scheduler::default();

    // Calculate 3^2 + 5 (should be 14)
    scheduler.push_task(Box::new(Exp::new(3, 2))).unwrap(); // 3^2 = 9
    scheduler.execute_all().unwrap();
    let nine: u128 = scheduler.pop_data().unwrap();
    assert_eq!(nine, 9, "Expected 3^2 to be 9");

    // Add 5 to the result
    scheduler.push_task(Box::new(Add::new(nine, 5))).unwrap(); // 9 + 5 = 14
    scheduler.execute_all().unwrap();

    // Check result
    let result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(result, 14, "Expected 9 + 5 to be 14");
}

#[test]
fn test_complex_calculation() {
    // Test a more complex calculation: (2^3) * (Fib(5) + 1)
    let mut scheduler = Scheduler::default();

    // Calculate 2^3 = 8
    scheduler.push_task(Box::new(Exp::new(2, 3))).unwrap();
    scheduler.execute_all().unwrap();
    let exp_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(exp_result, 8, "Expected 2^3 to be 8");

    // Calculate Fib(5) = 5
    scheduler.push_task(Box::new(Fib::new(5))).unwrap();
    scheduler.execute_all().unwrap();
    let fib_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(fib_result, 5, "Expected Fib(5) to be 5");

    // Add Fib(5) + 1 = 6
    scheduler
        .push_task(Box::new(Add::new(fib_result, 1)))
        .unwrap();
    scheduler.execute_all().unwrap();
    let add_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(add_result, 6, "Expected 5 + 1 to be 6");

    // Multiply (2^3) * (Fib(5) + 1) = 8 * 6 = 48
    scheduler
        .push_task(Box::new(Mul::new(exp_result, add_result)))
        .unwrap();
    scheduler.execute_all().unwrap();

    let final_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(final_result, 48, "Expected 8 * 6 to be 48");
}

#[test]
fn test_scheduler_multiple_tasks() {
    // Test the scheduler executing multiple tasks in the right order
    let mut scheduler = Scheduler::default();

    // Push a series of tasks
    scheduler.push_task(Box::new(Add::new(1, 2))).unwrap(); // 1+2 = 3
    scheduler.execute_all().unwrap();
    let add_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(add_result, 3);

    scheduler.push_task(Box::new(Mul::new(3, 4))).unwrap(); // 3*4 = 12
    scheduler.execute_all().unwrap();
    let mul_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(mul_result, 12);

    scheduler.push_task(Box::new(Exp::new(2, 2))).unwrap(); // 2^2 = 4
    scheduler.execute_all().unwrap();
    let exp_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(exp_result, 4);

    scheduler.push_task(Box::new(Fib::new(3))).unwrap(); // Fib(3) = 2
    scheduler.execute_all().unwrap();
    let fib_result: u128 = scheduler.pop_data().unwrap();
    assert_eq!(fib_result, 2);
}
