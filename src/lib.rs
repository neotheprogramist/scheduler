//! A task scheduler library that allows for task composition and execution.
//!
//! This library provides a simple yet powerful scheduler that can execute tasks
//! in a controlled sequence, with support for serialization and deserialization.
//! Tasks operate on a data stack and can be composed to create complex operations.

pub mod codec;
pub mod scheduler;
pub mod stack;
pub mod tasks;

// Re-export key types for easier access by library users
pub use scheduler::{Scheduler, SchedulerTask, TaskError};
pub use tasks::{add, mul};

// Library version and metadata
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
