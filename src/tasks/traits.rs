//! Traits for implementing generic phased tasks
//!
//! This module provides a more generic approach to implementing phased tasks,
//! making it easier to create complex operations with multiple phases.

use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

use crate::{
    error::Result,
    scheduler::{Scheduler, SchedulerTask},
    tasks::{TaskArgs, TaskResult},
};

/// Trait for tasks that can be divided into phases.
///
/// This trait provides a generic way to implement tasks that require multiple phases.
/// It handles the common logic for phase execution and state management.
pub trait PhasedTask:
    SchedulerTask + Default + Serialize + DeserializeOwned + Debug + 'static
{
    /// The type of arguments that this task accepts
    type Args: TaskArgs;

    /// The type of result that this task produces
    type Result: TaskResult;

    /// The type of internal state used between phases
    type State: Serialize + DeserializeOwned + Debug;

    /// The initial phase of the task.
    ///
    /// This is called when the task is first executed. It should set up the initial state
    /// and either produce a result directly or schedule the next phase.
    fn initial_phase(&mut self, scheduler: &mut Scheduler, args: Self::Args) -> Result<()>;

    /// The subsequent phase of the task.
    ///
    /// This is called for all phases after the initial phase. It should update the state
    /// based on the results of the previous phase and either produce a final result or
    /// schedule the next phase.
    fn subsequent_phase(
        &mut self,
        scheduler: &mut Scheduler,
        state: &mut Self::State,
    ) -> Result<()>;

    /// Creates a new instance of the task's next phase.
    ///
    /// This method should create a new instance of the task in its next phase.
    fn next_phase(&self) -> Box<dyn SchedulerTask>;

    /// Determine if the current phase is the initial phase.
    ///
    /// Returns true if this is the first phase of the task.
    fn is_initial_phase(&self) -> bool;

    /// Produce the final result of the task.
    ///
    /// This is called when the task has completed all phases.
    fn produce_result(&self, state: &Self::State) -> Self::Result;

    /// Check if the task has completed all phases.
    ///
    /// This is called to determine if the task should produce a final result
    /// or schedule the next phase.
    fn is_complete(&self, state: &Self::State) -> bool;

    /// Execute the appropriate phase of the task.
    ///
    /// This method handles the common logic for executing the task's phases.
    /// It determines which phase to execute, calls the appropriate method,
    /// and handles error reporting.
    fn execute_phase(&mut self, scheduler: &mut Scheduler) {
        let result = if self.is_initial_phase() {
            // If this is the initial phase, decode arguments and execute
            match scheduler.pop_data::<Self::Args>() {
                Ok(args) => self.initial_phase(scheduler, args),
                Err(err) => {
                    eprintln!("Failed to decode arguments: {:?}", err);
                    return;
                }
            }
        } else {
            // If this is a subsequent phase, decode state and execute
            match scheduler.pop_data::<Self::State>() {
                Ok(mut state) => {
                    let result = self.subsequent_phase(scheduler, &mut state);

                    // If we're not done yet, schedule the next phase
                    if result.is_ok() && !self.is_complete(&state) {
                        match scheduler.push_call(self.next_phase()) {
                            Ok(_) => {
                                // Push state back to stack
                                if let Err(err) = scheduler.push_data(&state) {
                                    eprintln!("Failed to push state: {:?}", err);
                                    return;
                                }
                            }
                            Err(err) => {
                                eprintln!("Failed to schedule next phase: {:?}", err);
                                return;
                            }
                        }
                    } else if result.is_ok() {
                        // If we're done, produce the final result
                        let res = self.produce_result(&state);
                        if let Err(err) = scheduler.push_data(&res) {
                            eprintln!("Failed to push result: {:?}", err);
                            return;
                        }
                    }

                    result
                }
                Err(err) => {
                    eprintln!("Failed to decode state: {:?}", err);
                    return;
                }
            }
        };

        // Report any errors
        if let Err(err) = result {
            eprintln!("Phase execution failed: {:?}", err);
        }
    }
}
