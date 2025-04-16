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
pub struct End {
    result: u128,
}

impl End {
    pub fn new(result: u128) -> Self {
        Self { result }
    }
    
    /// Get the result of the addition
    pub fn result(&self) -> u128 {
        self.result
    }
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
    
    /// Get the result if this is an End task, or None otherwise
    pub fn get_result(&self) -> Option<u128> {
        match self {
            AddTask::End(end) => Some(end.result()),
            _ => None,
        }
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
