use std::io;

fn main() {
    println!("Enter value in Fahrenheit:");

    let mut f = String::new();

    io::stdin().read_line(&mut f)
            .expect("Failed to read line!");

    let f: f32 = f.trim().parse().expect("Enter valid Fahrenheit value!");

    let c = f_to_c(f);

    println!("Fahrenheit: {}", f);
    println!("Celsius: {:.2}", c);
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * (5.0/9.0)
}