//! Multiplication task using the builder pattern.
//!
//! This module implements a multiplication task that uses the builder pattern
//! to support multiple phases and customization options.
//!
//! # Key Design Features
//!
//! - **Type-Level Phase Tracking**: The phase is embedded in the type (MulBuilder<Initial>, MulBuilder<Next>),
//!   providing compile-time guarantees that phases are used correctly.
//!
//! - **Builder Pattern**: The task is configured using a fluent builder API, allowing for
//!   readable and customizable task creation.
//!
//! - **Custom Error Handling**: Three strategies are provided: LogAndContinue, FailFast, and DefaultValue.
//!
//! - **Result Processing**: Custom processing functions can be injected to transform results
//!   between phases.
//!
//! - **Serialization Support**: Tasks can be serialized and deserialized, allowing them to be
//!   passed between different execution contexts.
//!
//! # Examples
//!
//! ```
//! use scheduler::{Scheduler, tasks::{builder_mul::{MulBuilder, Initial, Args}, TaskResult}};
//!
//! // Create a scheduler
//! let mut scheduler = Scheduler::default();
//!
//! // Set up arguments for the multiplication
//! let args = Args { x: 5, y: 3 };
//! scheduler.push_data(&args).unwrap();
//!
//! // Create task with builder pattern and custom options
//! let task = MulBuilder::<Initial>::new()
//!     .with_name("MyMultiplication")
//!     .with_processor(|current, new| {
//!         println!("Current: {}, New: {}, Result: {}", current, new, current + new);
//!         current + new
//!     })
//!     .build();
//!
//! // Push task to scheduler and execute
//! scheduler.push_call(task).unwrap();
//! scheduler.execute_all().unwrap();
//!
//! // Retrieve the result
//! let res = scheduler.pop_data().unwrap();
//! assert_eq!(res.result, 15); // 5 * 3 = 15
//! ```

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::{
    error::Result,
    scheduler::{Scheduler, SchedulerTask},
    tasks::{
        TaskArgs, TaskResult,
        add::{self, Add},
    },
};

/// Arguments for the MulBuilder task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Args {
    /// First operand
    pub x: u8,
    /// Second operand (number of times to add x)
    pub y: u8,
}

impl TaskArgs for Args {}

/// Result of the MulBuilder task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Res {
    /// The product of x and y
    pub result: u8,
}

impl TaskResult for Res {}

/// Internal state for the MulBuilder task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// First operand
    pub x: u8,
    /// Second operand
    pub y: u8,
    /// Running result
    pub result: u8,
    /// Number of additions completed
    pub counter: u8,
}

/// Phase marker traits for type-level phase tracking
pub trait Phase {}

/// Initial phase marker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Initial;
impl Phase for Initial {}

/// Subsequent phase marker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Next;
impl Phase for Next {}

/// Error handling strategies for the MulBuilder task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorHandling {
    /// Log errors and continue
    LogAndContinue,
    /// Fail fast on first error
    FailFast,
    /// Return default value on error
    DefaultValue(u8),
}

impl Default for ErrorHandling {
    fn default() -> Self {
        Self::LogAndContinue
    }
}

/// A multiplication task builder using the builder pattern.
///
/// This implementation tracks the phase at the type level for type safety.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MulBuilder<P: Phase> {
    /// Phase type marker (phantom data doesn't exist at runtime)
    #[serde(skip)]
    phase: PhantomData<P>,
    /// Optional custom name for the task
    name: Option<String>,
    /// Optional error handling strategy
    error_handling: ErrorHandling,
    /// Optional custom processing function
    #[serde(skip)]
    process_fn: Option<ProcessFn>,
}

/// Type alias for the processing function
type ProcessFn = fn(u8, u8) -> u8;

impl MulBuilder<Initial> {
    /// Create a new multiplication task builder.
    pub fn new() -> Self {
        Self {
            phase: PhantomData,
            name: None,
            error_handling: ErrorHandling::default(),
            process_fn: None,
        }
    }

    /// Set a custom name for the task.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Configure error handling strategy.
    pub fn with_error_handling(mut self, strategy: ErrorHandling) -> Self {
        self.error_handling = strategy;
        self
    }

    /// Set a custom processing function for handling results
    pub fn with_processor(mut self, processor: ProcessFn) -> Self {
        self.process_fn = Some(processor);
        self
    }

    /// Build the initial task.
    pub fn build(self) -> Box<dyn SchedulerTask> {
        Box::new(self)
    }
}

impl MulBuilder<Next> {
    /// Create a task for the next phase.
    fn new_next_phase(
        name: Option<String>,
        error_handling: ErrorHandling,
        process_fn: Option<ProcessFn>,
    ) -> Self {
        Self {
            phase: PhantomData,
            name,
            error_handling,
            process_fn,
        }
    }
}

#[typetag::serde(name = "MulBuilderInitial")]
impl SchedulerTask for MulBuilder<Initial> {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        if let Err(err) = self.execute_initial(scheduler) {
            self.handle_error(err);
        }
    }
}

#[typetag::serde(name = "MulBuilderNext")]
impl SchedulerTask for MulBuilder<Next> {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        if let Err(err) = self.execute_next(scheduler) {
            self.handle_error(err);
        }
    }
}

impl MulBuilder<Initial> {
    /// Execute the initial phase of the multiplication.
    fn execute_initial(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode arguments from data stack
        let args: Args = scheduler.pop_data()?;

        // Special case: if y is 0, result is 0
        if args.y == 0 {
            let res = Res { result: 0 };
            scheduler.push_data(&res)?;
            return Ok(());
        }

        // Set up initial state
        let state = State {
            x: args.x,
            y: args.y,
            result: 0,
            counter: 0,
        };

        // If counter < y, need to do more additions
        if state.counter < state.y {
            // Schedule tasks (Add then Mul)
            scheduler.schedule_tasks(vec![
                Box::new(Add::new()),
                Box::new(MulBuilder::<Next>::new_next_phase(
                    self.name.clone(),
                    self.error_handling.clone(),
                    self.process_fn,
                )),
            ])?;

            // Prepare arguments for the Add task
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and add args to the stack
            scheduler.push_data(&state)?;
            scheduler.push_data(&add_args)?;
        } else {
            // Return final result (should be 0 since counter is 0)
            let res = Res {
                result: state.result,
            };
            scheduler.push_data(&res)?;
        }

        Ok(())
    }
}

impl MulBuilder<Next> {
    /// Execute the subsequent phase of the multiplication.
    fn execute_next(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode Add result and state
        let add_res: add::Res = scheduler.pop_data()?;
        let mut state: State = scheduler.pop_data()?;

        // Update state with the result from Add
        let new_result = if let Some(process_fn) = self.process_fn {
            process_fn(state.result, add_res.result)
        } else {
            add_res.result
        };

        state.result = new_result;
        state.counter = state.counter.saturating_add(1);

        // If we haven't reached the target count, schedule more tasks
        if state.counter < state.y {
            // Create tasks
            let add_task: Box<dyn SchedulerTask> = Box::new(Add::new());
            let mul_task: Box<dyn SchedulerTask> = Box::new(MulBuilder::<Next>::new_next_phase(
                self.name.clone(),
                self.error_handling.clone(),
                self.process_fn,
            ));

            // Schedule tasks (Add then Mul)
            scheduler.schedule_tasks(vec![add_task, mul_task])?;

            // Prepare arguments for the Add task
            let add_args = add::Args {
                x: state.result,
                y: state.x,
            };

            // Push state and add args to the stack
            scheduler.push_data(&state)?;
            scheduler.push_data(&add_args)?;
        } else {
            // Return final result
            let res = Res {
                result: state.result,
            };
            scheduler.push_data(&res)?;
        }

        Ok(())
    }
}

// Common methods for all phase types
impl<P: Phase> MulBuilder<P> {
    /// Handle errors according to the configured strategy
    fn handle_error(&self, err: crate::error::Error) {
        match &self.error_handling {
            ErrorHandling::LogAndContinue => {
                let task_name = self.name.as_deref().unwrap_or("MulBuilder");
                eprintln!("{} phase failed: {:?}", task_name, err);
            }
            ErrorHandling::FailFast => {
                let task_name = self.name.as_deref().unwrap_or("MulBuilder");
                panic!("{} phase failed: {:?}", task_name, err);
            }
            ErrorHandling::DefaultValue(default) => {
                let res = Res { result: *default };
                // Try to push the default result, but ignore errors
                let _ = Scheduler::default().push_data(&res);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::Scheduler;

    #[test]
    fn test_mul_builder_zero() {
        let mut scheduler = Scheduler::default();

        // Set up arguments with y=0
        let args = Args { x: 5, y: 0 };
        scheduler.push_data(&args).unwrap();

        // Execute task
        let task = MulBuilder::<Initial>::new().with_name("ZeroTest").build();
        scheduler.push_call(task).unwrap();
        scheduler.execute().unwrap();

        // Check result (should be 0)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 0);
    }

    #[test]
    fn test_mul_builder_normal() {
        let mut scheduler = Scheduler::default();

        // Set up arguments
        let args = Args { x: 5, y: 3 };
        scheduler.push_data(&args).unwrap();

        // Create task with builder pattern
        let task = MulBuilder::<Initial>::new()
            .with_name("NormalTest")
            .with_error_handling(ErrorHandling::FailFast)
            .build();

        // Push task to scheduler
        scheduler.push_call(task).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 15);
    }

    #[test]
    fn test_mul_builder_with_processor() {
        let mut scheduler = Scheduler::default();

        // Set up arguments
        let args = Args { x: 5, y: 3 };
        scheduler.push_data(&args).unwrap();

        // Create task with custom processor
        let task = MulBuilder::<Initial>::new()
            .with_name("ProcessorTest")
            .with_processor(|current, new| {
                // Custom processing logic
                println!("Processing: current={}, new={}", current, new);
                current + new
            })
            .build();

        // Push task to scheduler
        scheduler.push_call(task).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 15);
    }

    #[test]
    fn test_mul_builder_overflow() {
        let mut scheduler = Scheduler::default();

        // Set up arguments that would overflow u8
        let args = Args { x: 100, y: 3 };
        scheduler.push_data(&args).unwrap();

        // Create task with builder pattern and default value fallback
        let task = MulBuilder::<Initial>::new()
            .with_name("OverflowTest")
            .with_error_handling(ErrorHandling::DefaultValue(42))
            .build();

        // Push task to scheduler
        scheduler.push_call(task).unwrap();

        // Execute all tasks
        scheduler.execute_all().unwrap();

        // Check result (should be saturated at 255)
        let res: Res = scheduler.pop_data().unwrap();
        assert_eq!(res.result, 255);
    }
}
