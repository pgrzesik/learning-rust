use std::thread;
use std::time::Duration;

fn main() {
    simulated_expensive_calculation(2);
}


fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("Very expensive calculation...");
    thread::sleep(Duration::from_secs(2));
    intensity
}