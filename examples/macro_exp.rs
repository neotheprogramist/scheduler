use scheduler::{
    error::Result,
    scheduler::{Scheduler, SchedulerTask},
    tasks::{TaskArgs, TaskResult, mul},
};
use serde::{Deserialize, Serialize};

// Define argument type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpArgs {
    pub x: u8,
    pub y: u8,
}

impl TaskArgs for ExpArgs {}

// Define result type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpRes {
    pub result: u8,
}

impl TaskResult for ExpRes {}

// Define state type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpState {
    pub x: u8,
    pub y: u8,
    pub result: u8,
    pub counter: u8,
}

// Define the task enum with a different name to avoid tag conflicts
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum MacroExp {
    #[default]
    P0,
    P1,
}

#[typetag::serde]
impl SchedulerTask for MacroExp {
    fn execute(&mut self, scheduler: &mut Scheduler) {
        println!("Executing MacroExp task with phase: {:?}", self);
        match self {
            MacroExp::P0 => {
                if let Err(err) = self.p0(scheduler) {
                    eprintln!("MacroExp P0 phase failed: {:?}", err);
                }
            }
            MacroExp::P1 => {
                if let Err(err) = self.p1(scheduler) {
                    eprintln!("MacroExp P1 phase failed: {:?}", err);
                }
            }
        }
    }
}

impl MacroExp {
    /// Create a new exponentiation task.
    pub fn new() -> Self {
        Self::default()
    }

    /// P0 phase: Initial phase that sets up the calculation.
    fn p0(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        println!("Executing P0 phase");
        // Decode arguments from data stack
        let args: ExpArgs = scheduler.pop_data()?;
        println!("Initial phase: x={}, y={}", args.x, args.y);

        // Special case: if y is 0, result is 1 (x^0 = 1)
        if args.y == 0 {
            let res = ExpRes { result: 1 };
            scheduler.push_data(&res)?;
            println!("Special case: y=0, result=1");
            return Ok(());
        }

        // Special case: if y is 1, result is x (x^1 = x)
        if args.y == 1 {
            let res = ExpRes { result: args.x };
            scheduler.push_data(&res)?;
            println!("Special case: y=1, result={}", args.x);
            return Ok(());
        }

        // Set up initial state - start with x as the initial result
        let state = ExpState {
            x: args.x,
            y: args.y,
            result: args.x, // Start with x since we're computing x^y and x^1 = x
            counter: 1,     // We already have x^1
        };
        println!("Initial state: {:?}", state);

        // If counter < y, need to do more multiplications
        if state.counter < state.y {
            // Push state to the stack first
            scheduler.push_data(&state)?;

            // Schedule tasks - order matters!
            // We must schedule the next task of our own (MacroExp::P1) before the
            // multiplication so it executes after multiplication completes
            scheduler.push_call(Box::new(MacroExp::P1))?;
            scheduler.push_call(Box::new(mul::Mul::new()))?;

            // Prepare arguments for the Mul task
            let mul_args = mul::Args {
                x: state.result,
                y: state.x,
            };
            println!("Scheduling multiplication: {} * {}", mul_args.x, mul_args.y);

            // Push args to the stack
            scheduler.push_data(&mul_args)?;
        } else {
            // Return final result (should be x since counter is 1)
            let res = ExpRes {
                result: state.result,
            };
            scheduler.push_data(&res)?;
            println!(
                "No more multiplications needed, returning result: {}",
                res.result
            );
        }

        Ok(())
    }

    /// P1 phase: Subsequent phase that processes multiplication results.
    fn p1(&mut self, scheduler: &mut Scheduler) -> Result<()> {
        println!("Executing P1 phase");

        // Get multiplication result
        let mul_res: mul::Res = match scheduler.pop_data() {
            Ok(res) => {
                println!("Got multiplication result: {:?}", res);
                res
            }
            Err(err) => {
                eprintln!("Failed to get multiplication result: {:?}", err);
                return Err(err);
            }
        };

        // Get state
        let mut state: ExpState = match scheduler.pop_data() {
            Ok(state) => {
                println!("Got state: {:?}", state);
                state
            }
            Err(err) => {
                eprintln!("Failed to get state: {:?}", err);
                return Err(err);
            }
        };

        println!(
            "Subsequent phase: got multiplication result: {} from state: {:?}",
            mul_res.result, state
        );

        // Update state with the result from Mul
        state.result = mul_res.result;
        state.counter += 1;
        println!("Updated state: {:?}", state);

        // If we need more multiplications, schedule another one
        if state.counter < state.y {
            // Push updated state back to stack first
            scheduler.push_data(&state)?;

            // Create tasks
            let exp_task: Box<dyn SchedulerTask> = Box::new(MacroExp::P1);
            let mul_task: Box<dyn SchedulerTask> = Box::new(mul::Mul::new());

            // Schedule tasks
            scheduler.push_call(exp_task)?;
            scheduler.push_call(mul_task)?;

            // Prepare arguments for the Mul task
            let mul_args = mul::Args {
                x: state.result,
                y: state.x,
            };
            println!(
                "Scheduling next multiplication: {} * {}",
                mul_args.x, mul_args.y
            );

            // Push args to the stack
            scheduler.push_data(&mul_args)?;
        } else {
            // Return final result
            let res = ExpRes {
                result: state.result,
            };
            scheduler.push_data(&res)?;
            println!("Computation complete, final result: {}", res.result);
        }

        Ok(())
    }
}

fn main() {
    let mut scheduler = Scheduler::default();

    // Set up the exponentiation calculation: 2^4 = 16
    let args = ExpArgs { x: 2, y: 4 };
    println!("Setting up calculation: {}^{}", args.x, args.y);

    if let Err(err) = scheduler.push_data(&args) {
        eprintln!("Failed to push args: {:?}", err);
        return;
    }

    if let Err(err) = scheduler.push_call(Box::new(MacroExp::new())) {
        eprintln!("Failed to push task: {:?}", err);
        return;
    }

    println!("Starting execution...");

    // Execute all tasks until completion
    match scheduler.execute_all() {
        Ok(_) => println!("All tasks executed successfully"),
        Err(err) => {
            eprintln!("Error during execution: {:?}", err);
            return;
        }
    }

    // Get and print the result
    match scheduler.pop_data::<ExpRes>() {
        Ok(res) => {
            println!("{}^{} = {}", args.x, args.y, res.result);
            println!("This implementation uses a simpler enum-based approach.");
        }
        Err(err) => {
            eprintln!("Failed to get result: {:?}", err);

            // Try to pop whatever is on the stack as a debug measure
            if let Ok(value) = scheduler.pop_data::<mul::Res>() {
                println!("Found mul::Res on stack: {:?}", value);
            } else if let Ok(value) = scheduler.pop_data::<ExpState>() {
                println!("Found ExpState on stack: {:?}", value);
            } else if let Ok(value) = scheduler.pop_data::<ExpArgs>() {
                println!("Found ExpArgs on stack: {:?}", value);
            } else {
                println!("No recognizable data on stack");
            }
        }
    }
}
