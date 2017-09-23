extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello in my guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Input your guess, please:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = guess.trim().parse()
        .expect("It's not a valid number :(");

    println!("Your guess is: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too low!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("Great guess! You win!")
    }
}
