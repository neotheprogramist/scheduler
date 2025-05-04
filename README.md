# Scheduler with Phased Tasks

A Rust library that provides a task scheduler with support for complex, multi-phase task execution. This library enables breaking down complex operations into smaller, composable tasks that can be executed sequentially.

## Features

- **Bidirectional Stack**: The scheduler maintains two separate stacks - a data stack for passing information between tasks and a call stack for tracking pending tasks.
- **Task Serialization**: All tasks and data are serialized, making the scheduler's state persistent and resumable.
- **Task Composition**: Simple tasks can be composed to create complex operations.
- **Phased Execution**: Complex tasks can be broken down into multiple phases, with each phase scheduled and executed separately.
- **Self-Scheduling**: Tasks can decide whether to reschedule themselves for continued execution.

## Core Components

### Scheduler

The central component that manages task execution and data flow. It provides:

```rust
pub struct Scheduler {
    stack: BidirectionalStack<65536, 2>,
}
```

- `push_task(task)`: Add a task to the call stack
- `push_data(data)`: Add data to the data stack
- `pop_task()`: Remove and return the most recently added task
- `pop_data<T>()`: Remove and return the most recently added data, deserialized to type T
- `execute()`: Execute the next task in the queue
- `execute_all()`: Execute all tasks until the call stack is empty

### SchedulerTask Trait

Interface for all executable tasks:

```rust
#[typetag::serde(tag = "type")]
pub trait SchedulerTask: Send + Sync {
    fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>>;
    fn push_self(&mut self) -> bool;
}
```

- `execute()`: Performs the task's operation and returns new tasks to be pushed to the scheduler
- `push_self()`: Controls whether the task should be pushed back onto the stack after execution

## Example Tasks

The library comes with several example implementations:

### Add

A simple task that adds two numbers:

```rust
use scheduler::{Scheduler, SchedulerTask};

let mut scheduler = Scheduler::default();

// Set up task
scheduler.push_task(Box::new(Add::new(5, 3))).unwrap();

// Execute
scheduler.execute_all().unwrap();

// Get result
let result: u128 = scheduler.pop_data().unwrap();
assert_eq!(result, 8);
```

### Multiplication by Repeated Addition

Implements multiplication as repeated addition:

```rust
use scheduler::{Scheduler, SchedulerTask};

let mut scheduler = Scheduler::default();

// Set up task
scheduler.push_task(Box::new(Mul::new(5, 3))).unwrap();

// Execute
scheduler.execute_all().unwrap();

// Get result (5 * 3 = 15)
let result: u128 = scheduler.pop_data().unwrap();
assert_eq!(result, 15);
```

### Exponentiation

Implements exponentiation using multiplication tasks:

```rust
use scheduler::{Scheduler, SchedulerTask};

let mut scheduler = Scheduler::default();

// Set up task
scheduler.push_task(Box::new(Exp::new(2, 3))).unwrap();

// Execute
scheduler.execute_all().unwrap();

// Get result (2^3 = 8)
let result: u128 = scheduler.pop_data().unwrap();
assert_eq!(result, 8);
```

### Fibonacci

Recursively calculates Fibonacci numbers by scheduling subtasks:

```rust
use scheduler::{Scheduler, SchedulerTask};
use tasks::fib::Fib;

let mut scheduler = Scheduler::default();

// Set up task to calculate F(5)
scheduler.push_task(Box::new(Fib::new(5))).unwrap();

// Execute
scheduler.execute_all().unwrap();

// Get result
let result: u128 = scheduler.pop_data().unwrap();
assert_eq!(result, 5);
```

## Getting Started

1. Add this library to your Cargo.toml:
   ```toml
   [dependencies]
   scheduler = { path = "..." }
   tasks = { path = "..." }
   ```

2. Create and execute tasks:
   ```rust
   use scheduler::Scheduler;
   use tasks::add::Add;
   
   let mut scheduler = Scheduler::default();
   scheduler.push_task(Box::new(Add::new(1, 2))).unwrap();
   scheduler.execute_all().unwrap();
   let result: u128 = scheduler.pop_data().unwrap();
   ```

3. Implement your own tasks by implementing the `SchedulerTask` trait:
   ```rust
   use serde::{Deserialize, Serialize};
   use scheduler::{Result, Scheduler, SchedulerTask};
   
   #[derive(Debug, Serialize, Deserialize)]
   struct MyTask {
       // Task state goes here
   }
   
   #[typetag::serde]
   impl SchedulerTask for MyTask {
       fn execute(&mut self, scheduler: &mut Scheduler) -> Result<Vec<Box<dyn SchedulerTask>>> {
           // Task implementation goes here
           Ok(vec![])
       }
   }
   ```

## Advanced Usage: Creating Multi-Phase Tasks

To create a task with multiple phases:

1. Create a main task that schedules the first phase
2. Implement an internal state-tracking task that schedules subsequent phases
3. Use the `push_self()` method to control when the task should be rescheduled

See the `Mul` and `Exp` implementations for examples of this pattern.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
