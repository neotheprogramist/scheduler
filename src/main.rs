use scheduler::{
    scheduler::Scheduler,
    tasks::{
        SchedulerTask,
        mul::{Begin, MulTask},
    },
};

fn main() {
    println!("Hello World");

    // Create a scheduler and push a multiplication task (3 * 4)
    let a = 3;
    let b = 4;
    let mut scheduler = Scheduler::new();
    scheduler.push(SchedulerTask::Mul(MulTask::Begin(Begin::new(a, b))));

    // Poll the scheduler until we reach the final state (End)
    // We execute each computational step
    let mut steps = 0;

    // Process tasks until we're at the End task or we encounter an error
    while let Ok(()) = scheduler.poll() {
        steps += 1;
        println!("Step {}: Task processed successfully", steps);

        // Check if we're at the End task
        if let Some(SchedulerTask::Mul(mul_task)) = scheduler.peek() {
            if let Some(result) = mul_task.get_result() {
                println!("Found result: {} * {} = {}", a, b, result);
                break;
            }
        }
    }

    println!("Computation completed in {} steps", steps);

    if scheduler.is_empty() {
        println!("All tasks processed!");
    } else {
        // Try to extract the result one more time
        if let Some(SchedulerTask::Mul(mul_task)) = scheduler.peek() {
            if let Some(result) = mul_task.get_result() {
                println!("Final result: {}", result);
            }
        }
    }
}
