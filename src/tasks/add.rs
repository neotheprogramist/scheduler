use serde::{Deserialize, Serialize};

use super::{SchedulerTask, TaskError, TaskTrait};

#[derive(Debug, Deserialize, Serialize)]
pub struct Begin {
    a: u128,
    b: u128,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct End {
    result: u128,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum AddTask {
    Begin(Begin),
    End(End),
}

impl AddTask {
    pub fn begin(data: Begin) -> Result<Vec<SchedulerTask>, TaskError> {
        let result = data.a + data.b;
        Ok(vec![SchedulerTask::Add(AddTask::End(End { result }))])
    }
}

impl TaskTrait for AddTask {
    fn poll(self) -> Result<Vec<SchedulerTask>, TaskError> {
        match self {
            AddTask::Begin(data) => AddTask::begin(data),
            _ => Err(TaskError::EmptyStack),
        }
    }
}
