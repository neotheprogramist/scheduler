//! # Scheduler
//!
//! A task scheduler library that allows for serialization and execution of tasks.
//!
//! ## Features
//!
//! - Task-based execution model
//! - Bidirectional stack for storing tasks and data
//! - Serialization of tasks using CBOR
//! - Error handling
//!
//! ## Example
//!
//! ```rust
//! use scheduler::{Scheduler, tasks::Add};
//! use scheduler::tasks::Output;
//!
//! // Create a scheduler
//! let mut scheduler = Scheduler::default();
//!
//! // Push a task to add two numbers
//! scheduler.push_task(Box::new(Add::new(5, 10))).unwrap();
//!
//! // Execute the task
//! scheduler.execute().unwrap();
//!
//! // Retrieve the result
//! let output: Output = scheduler.pop_data().unwrap();
//! assert_eq!(output.result, 15);
//! ```

/// Error handling types and utilities
pub mod error;

/// Task scheduler implementation
pub mod scheduler;

/// Bidirectional stack implementation
pub mod stack;

/// Task implementations
pub mod tasks;

// Re-export commonly used types
pub use error::{Error, Result};
pub use scheduler::{Scheduler, SchedulerTask};
pub use tasks::add::{Add, Output};
