use scheduler::{
    scheduler::{Scheduler, SchedulerTask},
    tasks::mul,
};

fn main() {
    let mut scheduler = Scheduler::default();

    let args = mul::Args { x: 9, y: 11 };
    scheduler.push_data(&args).unwrap();

    let mul_task: Box<dyn SchedulerTask> = Box::new(mul::Mul::default());
    scheduler.push_call(mul_task).unwrap();

    let mut steps = 0;
    while let Ok(()) = scheduler.execute() {
        // println!("Step {}: Task processed successfully", steps);
        steps += 1;
    }

    println!("Computation completed in {} steps", steps);

    let res: mul::Res = scheduler.pop_data().unwrap();

    println!("Computation result: {:?}", res);
}
