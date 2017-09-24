fn main() {
    countdown_for(6);
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

fn countdown_for(x: i32) {
    for number in (1..x).rev() {
        println!("{}!", number);
    }

    println!("Looping done!");
}