use crate::scheduler::Scheduler;

pub mod add;
pub mod mul;

#[derive(Debug)]
pub enum SchedulerTask {
    Add(add::Add),
    Mul(mul::Mul),
}
impl SchedulerTask {
    pub fn execute(&mut self, scheduler: &mut Scheduler) {
        match self {
            SchedulerTask::Add(task) => task.execute(scheduler),
            SchedulerTask::Mul(task) => task.execute(scheduler),
        }
    }
}
