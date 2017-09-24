fn main() {
    while_loop(5);
}

fn infinite_loop() {
    loop {
        println!("Hello, world!");
    }
}

fn while_loop(x: i32) {
    let mut number = x;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("Looping done!");
}