//! Task implementations for the scheduler.
//!
//! This module contains implementations of the `SchedulerTask` trait
//! that can be executed by the scheduler.

/// Addition task to demonstrate the scheduler capabilities.
pub mod add;

// Re-export task types for convenience
pub use add::{Add, Output};
