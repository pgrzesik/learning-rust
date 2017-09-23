use std::io;

fn main() {
    println!("Hello in my guessing game!");

    println!("Input your guess, please:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    println!("Your guess is: {}", guess);
}
