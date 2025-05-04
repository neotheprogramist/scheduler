use scheduler::{scheduler::Scheduler, tasks::add};

#[test]
fn test_add() {
    let mut scheduler = Scheduler::default();

    scheduler
        .push_task(Box::new(add::Add { x: 9, y: 11 }))
        .unwrap();

    let mut steps = 0;
    while let Ok(()) = scheduler.execute() {
        steps += 1;
    }

    assert_eq!(steps, 1);

    let output: add::Output = scheduler.pop_data().unwrap();

    assert_eq!(output.result, 20);
}
