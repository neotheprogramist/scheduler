# Rust Task Scheduler

A composable task scheduler library for Rust that allows chaining operations with persistence capabilities.

## Features

- **Task Composition**: Chain multiple tasks together to build complex operations
- **Stack-Based Design**: Uses data and call stacks for passing information between tasks
- **Serialization**: Full support for serialization and deserialization of tasks and data
- **Error Handling**: Robust error handling for reliable operation
- **Type Safety**: Strong typing for task arguments and results

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
scheduler = "0.1.0"
```

## Basic Usage

```rust
use scheduler::{Scheduler, add, mul};
use scheduler::codec::stack;

fn main() {
    // Create a new scheduler
    let mut scheduler = Scheduler::new();
    
    // Schedule an Add task
    let add_task = Box::new(add::Add::new());
    scheduler.push_call(add_task).unwrap();
    
    // Prepare arguments for the Add task
    let args = add::Args { x: 5, y: 10 };
    stack::encode(&mut scheduler, args);
    
    // Execute the task
    scheduler.execute().unwrap();
    
    // Get the result
    let result: add::Res = stack::decode(&mut scheduler);
    println!("Result: {}", result.result); // Output: Result: 15
}
```

## Advanced Example: Multiplication by Repeated Addition

```rust
use scheduler::{Scheduler, mul};
use scheduler::codec::stack;

fn main() {
    // Create a new scheduler
    let mut scheduler = Scheduler::new();
    
    // Schedule a Mul task
    let mul_task = Box::new(mul::Mul::new());
    scheduler.push_call(mul_task).unwrap();
    
    // Prepare arguments for the Mul task
    let args = mul::Args { x: 5, y: 3 };
    stack::encode(&mut scheduler, args);
    
    // Execute all tasks until completion
    scheduler.execute_all().unwrap();
    
    // Get the result
    let result: mul::Res = stack::decode(&mut scheduler);
    println!("Result: {}", result.result); // Output: Result: 15
}
```

## Architecture

The scheduler consists of several core components:

- **Scheduler**: Manages the call stack and data stack
- **Tasks**: Implementations of the `SchedulerTask` trait
- **Codec**: Utilities for encoding and decoding data

Tasks can be chained together by pushing multiple tasks onto the call stack and passing data between them using the data stack.

## License

MIT
