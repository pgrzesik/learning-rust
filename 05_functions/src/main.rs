fn main() {
    println!("Hello, world!");

    another();
    another_with_param(10);

    let x = returning_value();
    println!("Value returned is equal to: {}", x);
}

fn another() {
    println!("Hello from another function!");
}

fn another_with_param(x: i32) {
    println!("Value of passed variable is: {}", x);
}

fn returning_value() -> i32 {
    10
}
