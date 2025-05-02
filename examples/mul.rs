use scheduler::{
    scheduler::{Scheduler, SchedulerTask},
    tasks::mul,
};

fn main() {
    let mut scheduler = Scheduler::new();
    
    // Encode arguments using CBOR
    let mut buffer = Vec::new();
    let args = mul::Args { x: 9, y: 11 };
    ciborium::ser::into_writer(&args, &mut buffer).unwrap();
    scheduler.extend_data(&buffer);
    
    let mul_task: Box<dyn SchedulerTask> = Box::new(mul::Mul::default());
    scheduler.push_call(mul_task);

    let mut steps = 0;
    while let Ok(()) = scheduler.execute() {
        println!("Step {}: Task processed successfully", steps);
        steps += 1;
    }

    println!("Computation completed in {} steps", steps);

    // Decode result using CBOR
    let reversed_data = scheduler.get_reversed_data();
    let mut cursor = std::io::Cursor::new(&reversed_data);
    let res: mul::Res = ciborium::de::from_reader(&mut cursor).unwrap();
    let pos = cursor.position() as usize;
    scheduler.truncate_stack(pos);

    println!("Computation result: {:?}", res);
}
