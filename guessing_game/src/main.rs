extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Hello in my guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Input your guess, please:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    println!("Your guess is: {}", guess);
}
