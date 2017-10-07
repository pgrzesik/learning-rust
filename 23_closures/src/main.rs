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
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break!");
        } else {
            println!("Run for {} minutes!", expensive_result);
        }
    }
}