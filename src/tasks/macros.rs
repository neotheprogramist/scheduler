//! Macros for easily defining phased tasks
//!
//! This module provides macros that reduce boilerplate when implementing
//! phased tasks.

/// A macro that generates a phased task implementation.
///
/// This macro generates the boilerplate code for implementing the SchedulerTask
/// and PhasedTask traits. It takes the task name, phase enum, and state struct
/// as inputs.
///
/// # Example
///
/// ```
/// use scheduler::{phased_task, tasks::{TaskArgs, TaskResult, PhasedTask}, Scheduler, Result};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct ExpArgs { pub x: u8, pub y: u8 }
///
/// // Implement the TaskArgs trait for ExpArgs
/// impl TaskArgs for ExpArgs {}
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct ExpRes { pub result: u8 }
///
/// // Implement the TaskResult trait for ExpRes
/// impl TaskResult for ExpRes {}
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct ExpState {
///     pub x: u8,
///     pub y: u8,
///     pub result: u8,
///     pub counter: u8,
/// }
///
/// phased_task! {
///     /// A task that computes exponents using repeated multiplication.
///     pub struct ExponentTask {
///         args: ExpArgs,
///         result: ExpRes,
///         state: ExpState,
///         phases: [Initial, Next],
///     }
///
///     impl ExponentTask {
///         fn initial_phase(&mut self, scheduler: &mut Scheduler, args: Self::Args) -> Result<()> {
///             // Initial phase implementation
///             Ok(())
///         }
///
///         fn subsequent_phase(&mut self, scheduler: &mut Scheduler, state: &mut Self::State) -> Result<()> {
///             // Subsequent phase implementation
///             Ok(())
///         }
///
///         fn is_complete(&self, state: &Self::State) -> bool {
///             // Completion check
///             true
///         }
///
///         fn produce_result(&self, state: &Self::State) -> Self::Result {
///             // Result production
///             Self::Result { result: 0 }
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! phased_task {
    (
        $(#[$struct_attr:meta])*
        $vis:vis struct $name:ident {
            args: $args_type:ty,
            result: $result_type:ty,
            state: $state_type:ty,
            phases: [$initial_phase:ident, $next_phase:ident $(,$other_phases:ident)*],
        }

        impl $impl_name:ident {
            fn initial_phase(&mut $self_initial:ident, $scheduler_initial:ident: &mut Scheduler, $args:ident: Self::Args) -> Result<()> $initial_block:block

            fn subsequent_phase(&mut $self_subsequent:ident, $scheduler_subsequent:ident: &mut Scheduler, $state:ident: &mut Self::State) -> Result<()> $subsequent_block:block

            fn is_complete(&$self_complete:ident, $state_complete:ident: &Self::State) -> bool $complete_block:block

            fn produce_result(&$self_result:ident, $state_result:ident: &Self::State) -> Self::Result $result_block:block
        }
    ) => {
        paste::paste! {
            $(#[$struct_attr])*
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            $vis enum [<$name Phase>] {
                #[default]
                $initial_phase,
                $next_phase,
                $($other_phases,)*
            }

            $(#[$struct_attr])*
            #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
            $vis struct $name {
                phase: [<$name Phase>],
            }

            impl $name {
                /// Create a new task in the initial phase.
                pub fn new() -> Self {
                    Self::default()
                }

                /// Create a task in the next phase.
                pub fn next() -> Self {
                    Self {
                        phase: [<$name Phase>]::$next_phase,
                    }
                }

                $(
                    /// Create a task in a specific phase.
                    pub fn $other_phases() -> Self {
                        Self {
                            phase: [<$name Phase>]::$other_phases,
                        }
                    }
                )*
            }

            #[typetag::serde]
            impl $crate::scheduler::SchedulerTask for $name {
                fn execute(&mut self, scheduler: &mut $crate::scheduler::Scheduler) {
                    $crate::tasks::PhasedTask::execute_phase(self, scheduler);
                }
            }

            impl $crate::tasks::PhasedTask for $name {
                type Args = $args_type;
                type Result = $result_type;
                type State = $state_type;

                fn initial_phase(&mut $self_initial, $scheduler_initial: &mut $crate::scheduler::Scheduler, $args: Self::Args) -> $crate::error::Result<()> $initial_block

                fn subsequent_phase(&mut $self_subsequent, $scheduler_subsequent: &mut $crate::scheduler::Scheduler, $state: &mut Self::State) -> $crate::error::Result<()> $subsequent_block

                fn next_phase(&self) -> Box<dyn $crate::scheduler::SchedulerTask> {
                    Box::new(Self::next())
                }

                fn is_initial_phase(&self) -> bool {
                    matches!(self.phase, [<$name Phase>]::$initial_phase)
                }

                fn produce_result(&$self_result, $state_result: &Self::State) -> Self::Result $result_block

                fn is_complete(&$self_complete, $state_complete: &Self::State) -> bool $complete_block
            }
        }
    };
}
