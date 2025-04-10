use std::time::Duration;

use monoio::time::sleep;

// timer_enabled is required for sleep() to work
#[monoio::main(timer_enabled = true)]
async fn main() {
    println!(
        "Starting Monoio application on thread: {:?}",
        std::thread::current().id()
    );

    // Spawn a few tasks
    for i in 1..=3 {
        let task_id = i;
        monoio::spawn(async move {
            println!("Task {task_id} started");

            // Simulate some async work
            sleep(Duration::from_secs(1)).await;

            println!("Task {task_id} completed after 1s");
        });
    }

    // Main task does its own work
    println!("Main task doing work...");
    sleep(Duration::from_secs(2)).await;
    println!("Main task completed after 2s");
}

