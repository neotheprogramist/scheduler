# Scheduler

A Rust library for task composition and execution with a bidirectional stack architecture.

## Features

- **Task-Based Architecture**: Define tasks that can be composed to create complex operations
- **Bidirectional Stack**: Efficient communication between tasks using a dual-purpose stack
- **Serialization Support**: Tasks and data can be serialized for storage or transport
- **Error Handling**: Comprehensive error handling with proper propagation
- **Type Safety**: Strongly typed task arguments and results

## Architecture

The library consists of three main components:

1. **Scheduler**: Manages task execution and maintains the bidirectional stack
2. **Stack**: Provides the underlying data structure for the scheduler
3. **Tasks**: Implements specific task operations (add, mul, etc.)

## Usage

### Basic Example

```rust
use scheduler::{Scheduler, tasks::{Add, AddArgs}};

// Create a new scheduler
let mut scheduler = Scheduler::default();

// Create addition arguments
let args = AddArgs { x: 5, y: 3 };

// Push arguments to data stack
scheduler.push_data(&args).unwrap();

// Schedule an addition task
scheduler.push_call(Box::new(Add::new())).unwrap();

// Execute the task
scheduler.execute().unwrap();

// Retrieve the result
let result: add::Res = scheduler.pop_data().unwrap();
assert_eq!(result.result, 8);
```

### Complex Example (Multiplication via Addition)

```rust
use scheduler::{Scheduler, tasks::{Mul, MulArgs}};

// Create a new scheduler
let mut scheduler = Scheduler::default();

// Create multiplication arguments
let args = MulArgs { x: 5, y: 3 };

// Push arguments to data stack
scheduler.push_data(&args).unwrap();

// Schedule a multiplication task
scheduler.push_call(Box::new(Mul::new())).unwrap();

// Execute all tasks (multiplication schedules additional tasks)
scheduler.execute_all().unwrap();

// Retrieve the result
let result: mul::Res = scheduler.pop_data().unwrap();
assert_eq!(result.result, 15);
```

### Creating Custom Tasks

1. Define your task structure:

```rust
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum MyTask {
    #[default]
    P0,
}

#[typetag::serde]
impl SchedulerTask for MyTask {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        if let Err(err) = self.p0(scheduler) {
            eprintln!("Task failed: {:?}", err);
        }
    }
}
```

2. Define argument and result types:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Args {
    pub value: u32,
}

impl TaskArgs for Args {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    pub result: u32,
}

impl TaskResult for Res {}
```

3. Implement the task execution:

```rust
impl MyTask {
    pub fn new() -> Self {
        Self::default()
    }

    fn p0(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        // Decode arguments from data stack
        let args: Args = scheduler.pop_data()?;

        // Perform operation
        let res = Res {
            result: args.value * 2,
        };

        // Push result to data stack
        scheduler.push_data(&res)?;
        
        Ok(())
    }
}
```

## Advanced Usage

### Custom Stack Capacity

```rust
// Create a scheduler with a 1024-byte stack
let scheduler = Scheduler::with_capacity::<1024>();
```

### Task Composition

Tasks can schedule additional tasks, allowing complex workflows:

```rust
// Create tasks
let task1: Box<dyn SchedulerTask> = Box::new(MyTask1::new());
let task2: Box<dyn SchedulerTask> = Box::new(MyTask2::new());

// Schedule in reverse order (task2 will execute after task1)
scheduler.schedule_tasks(vec![task2, task1]).unwrap();
```

## License

MIT
