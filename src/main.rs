use scheduler::{
    scheduler::{Scheduler, SchedulerTask},
    tasks::mul,
};

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.extend_data(
        &bincode::encode_to_vec(mul::Args { x: 2, y: 5 }, bincode::config::standard()).unwrap(),
    );
    let mul_task: Box<dyn SchedulerTask> = Box::new(mul::Mul::default());
    scheduler.push_call(mul_task);

    let mut steps = 0;
    while let Ok(()) = scheduler.execute() {
        println!("Step {}: Task processed successfully", steps);
        steps += 1;
    }

    println!("Computation completed in {} steps", steps);

    let reversed_data: Vec<u8> = scheduler.data_stack.iter().rev().cloned().collect();
    let (res, len): (mul::Res, usize) =
        bincode::decode_from_slice(&reversed_data, bincode::config::standard()).unwrap();
    scheduler
        .data_stack
        .truncate(scheduler.data_stack.len() - len);

    println!("Computation result: {:?}", res);
}
