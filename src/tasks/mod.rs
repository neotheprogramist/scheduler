//! Task implementations for the scheduler.
//!
//! This module contains various task implementations that can be scheduled
//! and executed by the scheduler. Each task:
//!
//! 1. Reads its arguments from the data stack
//! 2. Performs its operation
//! 3. Pushes its result back to the data stack
//!
//! Some tasks may also schedule additional tasks for more complex operations.

pub mod add;
// pub mod exp;
// pub mod mul;

// Re-export key types from task modules
pub use add::Add;
// pub use exp::Exp;
// pub use mul::Mul;
