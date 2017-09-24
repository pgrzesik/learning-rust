fn main() {
    for_loop();
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

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("{}!", element);
    }
}