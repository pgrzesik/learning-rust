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

struct Cacher<T>
    where T: Fn(i32) -> i32
{
    calculation: T,
    value: Option<i32>
}

impl<T> Cacher<T>
    where T: Fn(i32) -> i32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Very expensive calculation...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break!");
        } else {
            println!("Run for {} minutes!", expensive_result.value(intensity));
        }
    }
}
