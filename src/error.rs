use crate::stack::StackError;
use std::io;
use thiserror::Error;

/// Result type for the scheduler crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the scheduler crate.
#[derive(Debug, Error)]
pub enum Error {
    /// The stack is empty and cannot be popped from.
    #[error("Empty stack - attempted to read from an empty stack")]
    EmptyStack,

    /// Error propagated from the stack module.
    #[error(transparent)]
    StackCapacity(#[from] StackError),

    /// Error during serialization.
    #[error(transparent)]
    Serialization(#[from] ciborium::ser::Error<io::Error>),

    /// Error during deserialization.
    #[error(transparent)]
    Deserialization(#[from] ciborium::de::Error<io::Error>),

    /// The task data length is invalid.
    #[error("Invalid task length - task data exceeds maximum allowed size")]
    InvalidTaskLength,

    /// Error during task execution.
    #[error("Execution error: {0}")]
    Execution(String),

    /// Error in task implementation.
    #[error("Task error: {0}")]
    Task(String),

    /// Error for invalid data.
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// General IO error.
    #[error(transparent)]
    Io(#[from] io::Error),
}
