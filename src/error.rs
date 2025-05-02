//! Error types for the scheduler library.
//!
//! This module provides a consolidated error type for the entire library,
//! which wraps more specific errors from individual modules.

use crate::stack::StackError;
use thiserror::Error;

/// A type alias for Result with the library's error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all possible errors in the scheduler library.
#[derive(Debug, Error)]
pub enum Error {
    /// The stack is empty when trying to pop a value
    #[error("Empty stack")]
    EmptyStack,

    /// Error related to stack capacity
    #[error("Stack capacity error: {0}")]
    StackCapacity(String),

    /// Error during serialization
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Error during deserialization
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Invalid task length
    #[error("Invalid task length")]
    InvalidTaskLength,

    /// Generic execution error
    #[error("Execution error: {0}")]
    Execution(String),

    /// Task-specific error
    #[error("Task error: {0}")]
    Task(String),
}

// Convert from StackError to Error
impl From<StackError> for Error {
    fn from(err: StackError) -> Self {
        match err {
            StackError::InsufficientCapacity => {
                Error::StackCapacity("Not enough space in stack".to_string())
            }
        }
    }
}

// Convert from ciborium::ser::Error to Error
impl From<ciborium::ser::Error<std::io::Error>> for Error {
    fn from(err: ciborium::ser::Error<std::io::Error>) -> Self {
        Error::Serialization(err.to_string())
    }
}

// Convert from ciborium::de::Error to Error
impl From<ciborium::de::Error<std::io::Error>> for Error {
    fn from(err: ciborium::de::Error<std::io::Error>) -> Self {
        Error::Deserialization(err.to_string())
    }
}
