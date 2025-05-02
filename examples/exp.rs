use scheduler::{
    scheduler::{Scheduler, SchedulerTask},
    tasks::exp,
};

fn main() {
    let mut scheduler = Scheduler::default();

    // Set up the exponentiation calculation: 2^5 = 32
    let args = exp::Args { x: 2, y: 5 };
    scheduler.push_data(&args).unwrap();

    let exp_task: Box<dyn SchedulerTask> = Box::new(exp::Exp::default());
    scheduler.push_call(exp_task).unwrap();

    // Track the number of execution steps
    let mut steps = 0;
    while let Ok(()) = scheduler.execute() {
        steps += 1;
    }

    // Print number of execution steps
    println!("Computation completed in {} steps", steps);

    // Get and print the result
    let res: exp::Res = scheduler.pop_data().unwrap();
    println!("{}^{} = {}", args.x, args.y, res.result);
}
