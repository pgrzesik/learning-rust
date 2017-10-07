use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 3;
    let simulated_random_number = 4;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}


fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("Very expensive calculation...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: i32, random_number: i32) {
    if intensity < 25 {
        println!("Do {} pushups!", simulated_expensive_calculation(intensity));
        println!("Next, do {} situps!", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break!");
        } else {
            println!("Run for {} minutes!", simulated_expensive_calculation(intensity));
        }
    }
}