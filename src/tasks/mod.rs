//! Task implementations for the scheduler.
//!
//! This module contains various task implementations that can be scheduled
//! and executed by the scheduler.

pub mod add;
pub mod mul;

// Re-export key types from task modules
pub use add::{Add, Args as AddArgs, Res as AddResult};
pub use mul::{Args as MulArgs, Mul, Res as MulResult};

/// Common trait for task arguments
pub trait TaskArgs: serde::Serialize + serde::de::DeserializeOwned {}

/// Common trait for task results
pub trait TaskResult: serde::Serialize + serde::de::DeserializeOwned {}
