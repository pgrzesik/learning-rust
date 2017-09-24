fn main() {
    println!("Hello, world!");

    another();
    another_with_param(10);
}

fn another() {
    println!("Hello from another function!");
}

fn another_with_param(x: i32) {
    println!("Value of passed variable is: {}", x);
}
