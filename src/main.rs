use scheduler::{
    scheduler::Scheduler,
    tasks::{SchedulerTask, mul},
};

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.extend_data(
        &bincode::encode_to_vec(mul::Args { x: 2, y: 5 }, bincode::config::standard()).unwrap(),
    );
    scheduler.push_call(SchedulerTask::Mul(mul::Mul::default()));

    let mut steps = 0;
    while let Ok(()) = scheduler.poll() {
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
