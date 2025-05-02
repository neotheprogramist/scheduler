use scheduler::{
    scheduler::{Scheduler, SchedulerTask},
    tasks::generic_exp::{Args, ExponentTask, Res},
};

fn main() {
    let mut scheduler = Scheduler::default();

    // Set up the exponentiation calculation: 3^4 = 81
    let args = Args { x: 3, y: 4 };
    scheduler.push_data(&args).unwrap();

    let exp_task: Box<dyn SchedulerTask> = Box::new(ExponentTask::new());
    scheduler.push_call(exp_task).unwrap();

    // Execute all tasks
    scheduler.execute_all().unwrap();

    // Get and print the result
    let res: Res = scheduler.pop_data().unwrap();
    println!("{}^{} = {}", args.x, args.y, res.result);
    println!("Notice how this task uses the generic PhasedTask trait!");
}
