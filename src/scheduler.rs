use crate::error::{Error, Result};
use crate::stack::BidirectionalStack;
use serde::{Serialize, de::DeserializeOwned};
use std::io::Cursor;

#[typetag::serde(tag = "type")]
pub trait SchedulerTask {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>>;
}

#[derive(Debug, Default)]
pub struct Scheduler {
    stack: BidirectionalStack<4096>,
}

impl Scheduler {
    pub fn push_task(&mut self, task: Box<dyn SchedulerTask>) -> Result<()> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(&task, &mut buffer)?;

        self.stack.push_back(&buffer)?;

        Ok(())
    }

    pub fn push_data<T: Serialize>(&mut self, data: &T) -> Result<()> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(data, &mut buffer)?;

        self.stack.push_front(&buffer)?;
        Ok(())
    }

    pub fn pop_task(&mut self) -> Result<Box<dyn SchedulerTask>> {
        let data = self.stack.pop_back().ok_or(Error::EmptyStack)?;

        let mut cursor = Cursor::new(&data);
        let result = ciborium::de::from_reader(&mut cursor)?;

        Ok(result)
    }

    pub fn pop_data<T: DeserializeOwned>(&mut self) -> Result<T> {
        let data = self.stack.pop_front().ok_or(Error::EmptyStack)?;

        let mut cursor = Cursor::new(&data);
        let result = ciborium::de::from_reader(&mut cursor)?;

        Ok(result)
    }

    pub fn execute(&mut self) -> Result<()> {
        let mut task = self.pop_task()?;

        let tasks = task.execute(self)?;

        for task in tasks.into_iter().rev() {
            self.push_task(task)?;
        }

        Ok(())
    }

    pub fn execute_all(&mut self) -> Result<()> {
        while !self.is_empty() {
            self.execute()?;
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty_back()
    }
}
