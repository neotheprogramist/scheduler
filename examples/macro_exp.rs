use scheduler::{
    phased_task,
    scheduler::Scheduler,
    tasks::{TaskArgs, TaskResult},
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

// Define state type (not actually used in this implementation
// but required by the phased_task macro)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpState {
    pub x: u8,
    pub y: u8,
    pub result: u8,
}

// Use the phased_task macro to define our task with much less boilerplate
phased_task! {
    /// A task that performs exponentiation (x^y).
    pub struct MacroExp {
        args: ExpArgs,
        result: ExpRes,
        state: ExpState,
        phases: [P0, P1], // We only use P0 in this implementation
    }

    impl MacroExp {
        fn initial_phase(&mut self, scheduler: &mut Scheduler, args: Self::Args) -> Result<()> {
            println!("Calculating {}^{}", args.x, args.y);
            
            // Handle special cases
            if args.y == 0 {
                let result = ExpRes { result: 1 };
                scheduler.push_data(&result)?;
                println!("Special case: {}^0 = 1", args.x);
                return Ok(());
            }
            
            if args.y == 1 {
                let result = ExpRes { result: args.x };
                scheduler.push_data(&result)?;
                println!("Special case: {}^1 = {}", args.x, args.x);
                return Ok(());
            }
            
            // Calculate x^y directly
            let mut result = 1_u8;
            for _ in 0..args.y {
                result = result.saturating_mul(args.x);
            }
            
            // Push the result to the data stack
            let exp_result = ExpRes { result };
            scheduler.push_data(&exp_result)?;
            
            println!("{}^{} = {}", args.x, args.y, result);
            
            Ok(())
        }

        fn subsequent_phase(&mut self, scheduler: &mut Scheduler, state: &mut Self::State) -> Result<()> {
            // This phase is not used in this implementation,
            // but is required by the phased_task macro
            Ok(())
        }

        fn is_complete(&self, state: &Self::State) -> bool {
            // Always complete after initial phase
            true
        }

        fn produce_result(&self, state: &Self::State) -> Self::Result {
            // This should never be called
            ExpRes { result: state.result }
        }
    }
}

fn main() {
    let mut scheduler = Scheduler::default();

    // Set up the exponentiation calculation: 2^4 = 16
    let args = ExpArgs { x: 2, y: 4 };
    println!("Setting up calculation: {}^{}", args.x, args.y);

    // Push arguments to the data stack
    if let Err(err) = scheduler.push_data(&args) {
        eprintln!("Failed to push args: {:?}", err);
        return;
    }

    // Push the initial task to the call stack
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
            println!("\nFinal result: {}^{} = {}", args.x, args.y, res.result);
            println!("This implementation uses the phased_task macro to reduce boilerplate.");
        }
        Err(err) => {
            eprintln!("Failed to get result: {:?}", err);
        }
    }
}
