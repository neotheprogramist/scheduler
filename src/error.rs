use crate::stack::StackError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Empty stack")]
    EmptyStack,

    #[error(transparent)]
    StackCapacity(#[from] StackError),

    #[error(transparent)]
    Serialization(#[from] ciborium::ser::Error<std::io::Error>),

    #[error(transparent)]
    Deserialization(#[from] ciborium::de::Error<std::io::Error>),

    #[error("Invalid task length")]
    InvalidTaskLength,

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Task error: {0}")]
    Task(String),
}
