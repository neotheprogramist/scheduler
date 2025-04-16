use scheduler::{
    scheduler::Scheduler,
    tasks::{
        SchedulerTask,
        mul::{Begin, MulTask},
    },
};
fn main() {
    println!("Hello World");

    let mut scheduler = Scheduler::new();
    scheduler.push(SchedulerTask::Mul(MulTask::Begin(Begin::new(3, 4))));
}
