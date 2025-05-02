//! A task scheduler library that allows for task composition and execution.
//!
//! This library provides a simple yet powerful scheduler that can execute tasks
//! in a controlled sequence, with support for serialization and deserialization.
//! Tasks operate on a data stack and can be composed to create complex operations.
//!
//! # Architecture
//!
//! The library consists of three main components:
//! - **Scheduler**: Manages task execution and maintains the bidirectional stack
//! - **Stack**: Provides the underlying data structure for the scheduler
//! - **Tasks**: Implements specific task operations (add, mul, etc.)
//!
//! # Example
//!
//! ```rust
//! use scheduler::{Scheduler, tasks::{add, Add, AddArgs}};
//!
//! let mut scheduler = Scheduler::default();
//!
//! // Create addition arguments
//! let args = AddArgs { x: 5, y: 3 };
//!
//! // Push arguments to data stack
//! scheduler.push_data(&args).unwrap();
//!
//! // Schedule an addition task
//! scheduler.push_call(Box::new(Add::new())).unwrap();
//!
//! // Execute the task
//! scheduler.execute().unwrap();
//!
//! // Retrieve the result
//! let result: add::Res = scheduler.pop_data().unwrap();
//! assert_eq!(result.result, 8);
//! ```

pub mod error;
pub mod scheduler;
pub mod stack;
pub mod tasks;

// Re-export key types for easier access by library users
pub use error::{Error, Result};
pub use scheduler::{Scheduler, SchedulerGeneric, SchedulerTask};
pub use tasks::{TaskArgs, TaskResult, add, mul};

// Library version and metadata
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
