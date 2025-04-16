use serde::{Deserialize, Serialize};

use super::{SchedulerTask, TaskError, TaskTrait};

#[derive(Debug, Deserialize, Serialize)]
pub struct Begin {
    a: u128,
    b: u128,
}
impl Begin {
    pub fn new(a: u128, b: u128) -> Self {
        Self { a, b }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddPhase {
    counter: u128,
    a: u128,
    b: u128,
    result: u128,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct End {
    result: u128,
}

impl End {
    /// Get the result of the multiplication
    pub fn result(&self) -> u128 {
        self.result
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MulTask {
    Begin(Begin),
    AddPhase(AddPhase),
    End(End),
}

impl MulTask {
    pub fn begin(data: Begin) -> Result<Vec<SchedulerTask>, TaskError> {
        if data.a == 0 || data.b == 0 {
            Ok(vec![SchedulerTask::Mul(MulTask::End(End { result: 0 }))])
        } else {
            Ok(vec![SchedulerTask::Mul(MulTask::AddPhase(AddPhase {
                counter: 0,
                a: data.a,
                b: data.b,
                result: 0,
            }))])
        }
    }

    pub fn add_phase(data: AddPhase) -> Result<Vec<SchedulerTask>, TaskError> {
        if data.counter < data.b {
            Ok(vec![SchedulerTask::Mul(MulTask::AddPhase(AddPhase {
                counter: data.counter + 1,
                a: data.a,
                b: data.b,
                result: data.result + data.a,
            }))])
        } else {
            Ok(vec![SchedulerTask::Mul(MulTask::End(End {
                result: data.result,
            }))])
        }
    }

    /// Get the result if this is an End task, or None otherwise
    pub fn get_result(&self) -> Option<u128> {
        match self {
            MulTask::End(end) => Some(end.result()),
            _ => None,
        }
    }
}

impl TaskTrait for MulTask {
    fn poll(self) -> Result<Vec<SchedulerTask>, TaskError> {
        match self {
            MulTask::Begin(data) => MulTask::begin(data),
            MulTask::AddPhase(data) => MulTask::add_phase(data),
            _ => Err(TaskError::EmptyStack),
        }
    }
}
