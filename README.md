# Scheduler with Phased Tasks

This library provides a scheduler that can execute complex tasks by breaking them down into multiple phases. It's designed to make task composition easy and efficient.

## Features

- **Bidirectional Stack**: The scheduler maintains a data stack for passing information between tasks and a call stack for pending tasks.
- **Task Composition**: Tasks can be composed to create complex operations from simpler ones.
- **Phased Execution**: Complex tasks can be divided into multiple phases, with each phase scheduled separately.
- **Generic Implementation**: A generic `PhasedTask` trait for easy implementation of complex multi-phase tasks.
- **Macro Support**: A `phased_task!` macro to reduce boilerplate when defining phased tasks.

## Examples

### Basic Addition Task

```rust
use scheduler::{Scheduler, tasks::{Add, AddArgs}};

let mut scheduler = Scheduler::default();

// Set up arguments
let args = AddArgs { x: 5, y: 3 };
scheduler.push_data(&args).unwrap();

// Schedule task
scheduler.push_call(Box::new(Add::new())).unwrap();

// Execute
scheduler.execute_all().unwrap();

// Get result
let result: scheduler::tasks::AddResult = scheduler.pop_data().unwrap();
assert_eq!(result.result, 8);
```

### Multiplication by Repeated Addition

The multiplication task demonstrates how to implement a complex operation by breaking it down into multiple phases and using other tasks (Add) as building blocks:

```rust
use scheduler::{Scheduler, tasks::{Mul, MulArgs}};

let mut scheduler = Scheduler::default();

// Set up arguments
let args = MulArgs { x: 5, y: 3 };
scheduler.push_data(&args).unwrap();

// Schedule task
scheduler.push_call(Box::new(Mul::new())).unwrap();

// Execute
scheduler.execute_all().unwrap();

// Get result (5 * 3 = 15)
let result: scheduler::tasks::MulResult = scheduler.pop_data().unwrap();
assert_eq!(result.result, 15);
```

### Exponentiation using the Generic PhasedTask Trait

The exponentiation task shows how to use the `PhasedTask` trait to implement a multi-phase task with less boilerplate:

```rust
use scheduler::{
    scheduler::{Scheduler, SchedulerTask},
    tasks::generic_exp::{ExponentTask, Args, Res},
};

let mut scheduler = Scheduler::default();

// Set up arguments
let args = Args { x: 2, y: 3 };
scheduler.push_data(&args).unwrap();

// Schedule task
scheduler.push_call(Box::new(ExponentTask::new())).unwrap();

// Execute
scheduler.execute_all().unwrap();

// Get result (2^3 = 8)
let result: Res = scheduler.pop_data().unwrap();
assert_eq!(result.result, 8);
```

### Exponentiation using the phased_task Macro

The macro approach further reduces boilerplate by generating common code patterns:

```rust
use scheduler::{
    error::Result,
    phased_task,
    scheduler::{Scheduler, SchedulerTask},
    tasks::{TaskArgs, TaskResult},
};
use serde::{Deserialize, Serialize};

// Define types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpArgs { pub x: u8, pub y: u8 }

impl TaskArgs for ExpArgs {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpRes { pub result: u8 }

impl TaskResult for ExpRes {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpState {
    pub x: u8,
    pub y: u8,
    pub result: u8,
    pub counter: u8,
}

// Use the macro to define our task
phased_task! {
    pub struct MacroExp {
        args: ExpArgs,
        result: ExpRes,
        state: ExpState,
        phases: [Initial, Next],
    }

    impl MacroExp {
        fn initial_phase(&mut self, scheduler: &mut Scheduler, args: Self::Args) -> Result<()> {
            // Initial phase implementation
            Ok(())
        }

        fn subsequent_phase(&mut self, scheduler: &mut Scheduler, state: &mut Self::State) -> Result<()> {
            // Subsequent phase implementation
            Ok(())
        }

        fn is_complete(&self, state: &Self::State) -> bool {
            // Check if task is complete
            state.counter >= state.y
        }

        fn produce_result(&self, state: &Self::State) -> Self::Result {
            // Produce the final result
            ExpRes { result: state.result }
        }
    }
}
```

## Getting Started

1. Add this library to your dependencies:
   ```toml
   [dependencies]
   scheduler = { path = "..." }
   ```

2. Create a new task (optional - you can use the built-in tasks):
   ```rust
   use scheduler::{phased_task, error::Result, scheduler::Scheduler};
   
   // Define your task using one of the approaches above
   ```

3. Use the scheduler to execute your tasks:
   ```rust
   let mut scheduler = Scheduler::default();
   
   // Push arguments and tasks
   // ...
   
   // Execute all tasks
   scheduler.execute_all().unwrap();
   
   // Get results
   // ...
   ```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
