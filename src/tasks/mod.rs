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
pub mod builder_mul;
pub mod exp;
pub mod generic_exp;
pub mod macros;
pub mod mul;
pub mod traits;

// Re-export key types from task modules
pub use add::{Add, Args as AddArgs, Res as AddResult};
pub use exp::{Args as ExpArgs, Exp, Res as ExpResult};
pub use generic_exp::{Args as GenericExpArgs, ExponentTask, Res as GenericExpResult};
pub use mul::{Args as MulArgs, Mul, Res as MulResult};
pub use builder_mul::{MulBuilder, Initial, Next, ErrorHandling, Args as MulBuilderArgs, Res as MulBuilderResult};
pub use traits::PhasedTask;

// We don't re-export macros here since they are re-exported at the crate root level
// This avoids duplicate macro definition errors

/// Common trait for task arguments.
///
/// All task argument types should implement this trait to ensure they can be
/// serialized and deserialized by the scheduler.
///
/// # Examples
///
/// ```
/// use scheduler::tasks::TaskArgs;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct MyTaskArgs {
///     pub value: u32,
/// }
///
/// impl TaskArgs for MyTaskArgs {}
/// ```
pub trait TaskArgs: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug {}

/// Common trait for task results.
///
/// All task result types should implement this trait to ensure they can be
/// serialized and deserialized by the scheduler.
///
/// # Examples
///
/// ```
/// use scheduler::tasks::TaskResult;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct MyTaskResult {
///     pub output: u32,
/// }
///
/// impl TaskResult for MyTaskResult {}
/// ```
pub trait TaskResult: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug {}

// Implement TaskArgs and TaskResult for common types
impl TaskArgs for u8 {}
impl TaskArgs for u16 {}
impl TaskArgs for u32 {}
impl TaskArgs for u64 {}
impl TaskArgs for i8 {}
impl TaskArgs for i16 {}
impl TaskArgs for i32 {}
impl TaskArgs for i64 {}
impl TaskArgs for f32 {}
impl TaskArgs for f64 {}
impl TaskArgs for bool {}
impl TaskArgs for String {}

impl TaskResult for u8 {}
impl TaskResult for u16 {}
impl TaskResult for u32 {}
impl TaskResult for u64 {}
impl TaskResult for i8 {}
impl TaskResult for i16 {}
impl TaskResult for i32 {}
impl TaskResult for i64 {}
impl TaskResult for f32 {}
impl TaskResult for f64 {}
impl TaskResult for bool {}
impl TaskResult for String {}
