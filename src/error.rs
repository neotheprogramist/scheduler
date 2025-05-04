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
    #[error(transparent)]
    StackCapacity(#[from] StackError),

    /// Error during serialization
    #[error(transparent)]
    Serialization(#[from] ciborium::ser::Error<std::io::Error>),

    /// Error during deserialization
    #[error(transparent)]
    Deserialization(#[from] ciborium::de::Error<std::io::Error>),

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
